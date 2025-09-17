use crate::optimization::cfg::{Annotation, TackyCfg, TackyNode};
use crate::symbol::Symbol;
use crate::tacky::{Instruction, Val};
use std::collections::{HashSet, VecDeque};

pub fn dead_store_elimination(
    instructions: &[Instruction],
    static_vars: &HashSet<Val>,
    aliased_vars: &HashSet<Val>,
    trace: bool,
) -> Vec<Instruction> {
    let mut cfg = TackyCfg::new(instructions);
    if trace {
        println!("=======================");
        println!("Dead store elimination");
        println!("=======================");
        println!("INITIAL\n {cfg:#?}");
    }
    let all_static_vars = VarSet::from_vars(static_vars);
    let annotations = find_live_vars(&cfg, &all_static_vars, aliased_vars);
    if trace {
        println!("Live variables:\n {annotations:?}");
    }
    rewrite_instructions(&mut cfg, &annotations);
    if trace {
        println!("Instructions Rewritten:\n {cfg:#?}");
    }
    cfg.dump()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct VarSet(HashSet<Symbol>);

impl VarSet {
    fn empty() -> Self {
        Self(HashSet::new())
    }

    fn from_vars(vars: &HashSet<Val>) -> Self {
        let set = vars.iter().map(|v| {
            let Val::Var(symbol) = v else {
                panic!("Expected variable");
            };
            symbol.clone()
        }).collect();
        Self(set)
    }
    
    fn remove(&mut self, var: &Val) {
        let Val::Var(symbol) = var else {
            return;
        };
        self.0.remove(symbol);
    }

    fn add(&mut self, var: &Val) {
        let Val::Var(symbol) = var else {
            return;
        };
        self.0.insert(symbol.clone());
    }

    fn contains(&self, var: &Val) -> bool {
        let Val::Var(symbol) = var else {
            return false;
        };
        self.0.contains(symbol)
    }

    fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.iter().cloned());
    }
}

type LiveVars = Annotation<VarSet>;

fn transfer_function(annotations: &mut LiveVars, node: &TackyNode, all_static_vars: &VarSet, aliased_vars: &HashSet<Val>, all_live_vars: &VarSet) {
    let mut current_live_vars = all_live_vars.clone();
    for (i, instruction) in node.instructions.iter().enumerate().rev() {
        annotations.annotate_instruction(node.id, i, current_live_vars.clone());

        match instruction {
            Instruction::Binary { src1, src2, dst, .. } => {
                current_live_vars.remove(dst);
                current_live_vars.add(src1);
                current_live_vars.add(src2);
            }
            Instruction::Copy { src, dst }
            | Instruction::SignExtend { src, dst }
            | Instruction::Truncate { src, dst }
            | Instruction::ZeroExtend { src, dst }
            | Instruction::DoubleToInt { src, dst }
            | Instruction::DoubleToUInt { src, dst }
            | Instruction::IntToDouble { src, dst }
            | Instruction::UIntToDouble { src, dst }
            | Instruction::Unary { src, dst, .. } => {
                current_live_vars.remove(dst);
                current_live_vars.add(src);
            }
            Instruction::JumpIfNotZero { cond, ..}
            | Instruction::JumpIfZero { cond, .. } => {
                current_live_vars.add(cond);
            }

            Instruction::FnCall { name, args, dst } => {
                if let Some(dst) = dst {
                    current_live_vars.remove(dst);
                }
                for arg in args {
                    current_live_vars.add(arg);
                }
                current_live_vars.extend(all_static_vars);
                for var in aliased_vars.iter() {
                    current_live_vars.add(var)
                }
            }

            Instruction::Load { ptr, dst } => {
                current_live_vars.remove(dst);
                current_live_vars.add(ptr);
                for var in aliased_vars.iter() {
                    current_live_vars.add(var)
                }
            }
            Instruction::AddPtr { ptr, index, dst, .. } => {
                current_live_vars.remove(dst);
                current_live_vars.add(ptr);
                current_live_vars.add(index);
                for var in aliased_vars.iter() {
                    current_live_vars.add(var)
                }
            }
            Instruction::GetAddress { src, dst } => {
                current_live_vars.remove(dst);
            }
            Instruction::Store { src, ptr } => {
                current_live_vars.add(src);
                current_live_vars.add(ptr);
            }
            Instruction::CopyToOffset { src, dst , ..} => {
                current_live_vars.add(src);
            }
            Instruction::CopyFromOffset { src, dst, ..} => {
                current_live_vars.0.insert(src.clone());
                current_live_vars.remove(dst);
            }

            Instruction::Return(val) => {
                if let Some(val) = val {
                    current_live_vars.add(val)
                }
            }
            Instruction::Jump { .. } => {}
            Instruction::Label(_) => {}
        }
    }
    annotations.annotate_block(node.id, current_live_vars);
}

fn meet_operator(annotations: &mut LiveVars, cfg: &TackyCfg, node: &TackyNode, all_static_vars: &VarSet) -> VarSet {
    let mut live_vars = VarSet::empty();
    for succ_id in &node.successors {
        if succ_id == &cfg.exit_id() {
            live_vars.extend(all_static_vars);
        } else {
            let succ_live_vars = annotations.get_block_annotation(succ_id);
            live_vars.extend(succ_live_vars);
        }
    }
    live_vars
}

fn find_live_vars(cfg: &TackyCfg, all_static_vars: &VarSet, aliased_vars: &HashSet<Val>) -> LiveVars {
    let live_vars = VarSet::empty();
    let mut annotations = LiveVars::empty();

    let mut worklist = VecDeque::new();
    for node_id in cfg.all_ids() {
        if node_id == cfg.exit_id() || node_id == cfg.entry_id() {
            continue;
        }
        worklist.push_back(node_id);
        annotations.annotate_block(node_id, live_vars.clone());
    }

    while let Some(node_id) = worklist.pop_back() {
        let old_vars = &annotations.get_block_annotation(&node_id).clone();
        let node = cfg.get_node(node_id);
        let incoming_vars = meet_operator(&mut annotations, cfg, node, all_static_vars);
        transfer_function(&mut annotations, node, all_static_vars, aliased_vars, &incoming_vars);
        if old_vars != annotations.get_block_annotation(&node_id) {
            for pred_id in &node.predecessors {
                if pred_id == &cfg.entry_id() {
                    continue;
                }
                if !worklist.contains(pred_id) {
                    worklist.push_front(*pred_id);
                }
            }
        }
    }

    annotations
}

fn rewrite_instructions(cfg: &mut TackyCfg, annotations: &LiveVars) {
    for node in cfg.nodes_mut() {
        let mut new_instructions = Vec::new();
        for (i, instruction) in node.instructions.iter().enumerate() {
            match instruction {
                Instruction::Unary { dst, .. }
                | Instruction::Binary { dst, .. }
                | Instruction::Copy { dst, .. }
                | Instruction::SignExtend { dst, .. }
                | Instruction::Truncate { dst, .. }
                | Instruction::ZeroExtend { dst, .. }
                | Instruction::DoubleToInt { dst, .. }
                | Instruction::DoubleToUInt { dst, .. }
                | Instruction::IntToDouble { dst, .. }
                | Instruction::UIntToDouble { dst, .. }
                | Instruction::GetAddress { dst, .. }
                | Instruction::Load { dst, .. }
                | Instruction::AddPtr { dst, .. }
                | Instruction::CopyFromOffset { dst, .. } => {
                    let live_vars = annotations.get_instruction_annotation(node.id, i);
                    if !live_vars.contains(dst) {
                        continue;
                    }
                }
                Instruction::CopyToOffset { dst, .. } => {
                    let live_vars = annotations.get_instruction_annotation(node.id, i);
                    if !live_vars.0.contains(dst) {
                        continue;
                    }
                }

                _ => {}
            }
            new_instructions.push(instruction.clone());
        }
        node.instructions = new_instructions;
    }
}