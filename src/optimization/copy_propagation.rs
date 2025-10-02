use crate::optimization::cfg::Annotation;
use crate::optimization::VariableData;
use crate::tacky::cfg::{Cfg, CfgNode};
use crate::tacky::{Instruction, Val};
use std::collections::{HashSet, VecDeque};

pub fn copy_propagation(cfg: &mut Cfg, var_data: &VariableData, trace: bool) {
    if trace {
        println!("=======================");
        println!("Copy propagation");
        println!("=======================");
        println!("INITIAL\n {cfg:#?}");
    }

    let annotations = find_reaching_copies(cfg, var_data);

    if trace {
        println!("Reaching copies:\n {annotations:?}");
    }

    rewrite_instructions(cfg, &annotations);

    if trace {
        println!("Instructions Rewritten:\n {cfg:#?}");
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Copies(HashSet<Instruction>);

impl Copies {
    fn empty() -> Self {
        Self(HashSet::new())
    }

    fn from_cfg(cfg: &Cfg) -> Self {
        let mut result = HashSet::new();
        for node_id in cfg.all_ids() {
            let node = cfg.get_node(node_id);
            for instruction in &node.instructions {
                if let Instruction::Copy { .. } = instruction {
                    result.insert(instruction.clone());
                }
            }
        }
        Self(result)
    }

    fn intersect(&mut self, other: &Self) {
        self.0.retain(|copy| other.contains(copy));
    }

    fn add(&mut self, instruction: Instruction) {
        self.0.insert(instruction);
    }

    fn remove_if(&mut self, condition: impl Fn(&Val, &Val) -> bool) {
        self.0.retain(|copy| {
            let Instruction::Copy { src, dst } = copy else {
                panic!("Expected copy instruction")
            };
            !condition(src, dst)
        });
    }

    fn has_copy(&self, src: &Val, dst: &Val) -> bool {
        self.0.iter().any(|copy| {
            let Instruction::Copy {
                src: copy_src,
                dst: copy_dst,
            } = copy
            else {
                panic!("Expected copy instruction")
            };
            (copy_src == src && copy_dst == dst) || (copy_src == dst && copy_dst == src)
        })
    }

    fn contains(&self, instruction: &Instruction) -> bool {
        self.0.contains(instruction)
    }
}

type ReachingCopies = Annotation<Copies>;

fn find_reaching_copies(cfg: &Cfg, var_data: &VariableData) -> ReachingCopies {
    let all_copies = Copies::from_cfg(cfg);
    let mut annotations = ReachingCopies::empty();

    let mut worklist = VecDeque::new();
    for node_id in cfg.all_ids() {
        if node_id == cfg.exit_id() || node_id == cfg.entry_id() {
            continue;
        }
        worklist.push_back(node_id);
        annotations.annotate_block(node_id, all_copies.clone());
    }

    while let Some(node_id) = worklist.pop_front() {
        let old_copies = &annotations.get_block_annotation(&node_id).clone();
        let node = cfg.get_node(node_id);
        let incoming_copies = meet_operator(&mut annotations, cfg, node, &all_copies);
        transfer_function(&mut annotations, node, &incoming_copies, var_data);
        if old_copies != annotations.get_block_annotation(&node_id) {
            for succ_id in &node.successors {
                if succ_id == &cfg.exit_id() {
                    continue;
                }
                if !worklist.contains(succ_id) {
                    worklist.push_back(*succ_id);
                }
            }
        }
    }

    annotations
}

fn rewrite_instructions(cfg: &mut Cfg, annotations: &ReachingCopies) {
    for node in cfg.nodes_mut() {
        let mut new_instructions = Vec::new();
        for (i, instruction) in node.instructions.iter().enumerate() {
            let reaching_copies = annotations.get_instruction_annotation(node.id, i);
            let mut new_instruction = instruction.clone();
            match &mut new_instruction {
                Instruction::Copy { src, dst } => {
                    if reaching_copies.has_copy(src, dst) {
                        continue; //delete instruction
                    }
                    *src = replace_operand(src.clone(), reaching_copies);
                }
                Instruction::Unary { src, .. }
                | Instruction::SignExtend { src, .. }
                | Instruction::Truncate { src, .. }
                | Instruction::ZeroExtend { src, .. }
                | Instruction::DoubleToInt { src, .. }
                | Instruction::DoubleToUInt { src, .. }
                | Instruction::IntToDouble { src, .. }
                | Instruction::UIntToDouble { src, .. }
                | Instruction::Load { ptr: src, .. }
                | Instruction::Store { src, .. }
                | Instruction::CopyToOffset { src, .. } => {
                    *src = replace_operand(src.clone(), reaching_copies);
                }
                Instruction::Binary { src1, src2, .. } => {
                    *src1 = replace_operand(src1.clone(), reaching_copies);
                    *src2 = replace_operand(src2.clone(), reaching_copies);
                }
                Instruction::Return(Some(val)) => {
                    *val = replace_operand(val.clone(), reaching_copies);
                }
                Instruction::FnCall { args, .. } => {
                    for arg in args {
                        *arg = replace_operand(arg.clone(), reaching_copies);
                    }
                }
                Instruction::JumpIfZero { cond, .. } | Instruction::JumpIfNotZero { cond, .. } => {
                    *cond = replace_operand(cond.clone(), reaching_copies);
                }
                Instruction::AddPtr { ptr, index, .. } => {
                    *ptr = replace_operand(ptr.clone(), reaching_copies);
                    *index = replace_operand(index.clone(), reaching_copies);
                }
                Instruction::CopyFromOffset { src, .. } => {
                    let replaced_src = replace_operand(Val::Var(src.clone()), reaching_copies);
                    if let Val::Var(symbol) = replaced_src {
                        *src = symbol
                    }
                }
                _ => {}
            }
            new_instructions.push(new_instruction);
        }
        node.instructions = new_instructions;
    }
}

fn replace_operand(op: Val, copies: &Copies) -> Val {
    match &op {
        Val::Constant(_) => return op,
        Val::Var(_) => {
            for copy in copies.0.iter() {
                if let Instruction::Copy { src, dst } = copy
                    && *dst == op
                {
                    return src.clone();
                }
            }
        }
    };
    op
}

fn meet_operator(
    annotations: &mut ReachingCopies,
    cfg: &Cfg,
    node: &CfgNode,
    all_copies: &Copies,
) -> Copies {
    let mut incoming_copies = all_copies.clone();
    for pred_id in &node.predecessors {
        if pred_id == &cfg.entry_id() {
            return Copies::empty();
        }
        let pred_copies = annotations.get_block_annotation(pred_id);
        incoming_copies.intersect(pred_copies);
    }
    incoming_copies
}

fn transfer_function(
    annotations: &mut ReachingCopies,
    node: &CfgNode,
    initial_reaching_copies: &Copies,
    var_data: &VariableData,
) {
    let mut current_reaching_copies = initial_reaching_copies.clone();
    for (i, instruction) in node.instructions.iter().enumerate() {
        annotations.annotate_instruction(node.id, i, current_reaching_copies.clone());
        match instruction {
            Instruction::Copy { src, dst } => {
                if current_reaching_copies.contains(instruction) {
                    continue;
                }

                current_reaching_copies
                    .remove_if(|current_src, current_dst| current_src == dst || current_dst == dst);

                let src_ty = var_data.ty(src);
                let dst_ty = var_data.ty(dst);

                if src_ty == dst_ty || (src_ty.is_signed() == dst_ty.is_signed()) {
                    current_reaching_copies.add(instruction.clone())
                }
            }
            Instruction::FnCall { dst, .. } => {
                current_reaching_copies.remove_if(|current_src, current_dst| {
                    var_data.is_aliased_or_static(current_src)
                        || var_data.is_aliased_or_static(current_dst)
                        || Some(current_src) == dst.as_ref()
                        || Some(current_dst) == dst.as_ref()
                });
            }
            Instruction::Store { .. } => {
                current_reaching_copies.remove_if(|current_src, current_dst| {
                    var_data.is_aliased_or_static(current_src)
                        || var_data.is_aliased_or_static(current_dst)
                });
            }
            Instruction::Binary { dst, .. }
            | Instruction::Unary { dst, .. }
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
                current_reaching_copies
                    .remove_if(|current_src, current_dst| current_src == dst || current_dst == dst);
            }
            Instruction::CopyToOffset { dst, .. } => {
                current_reaching_copies.remove_if(|current_src, current_dst| {
                    let dst = &Val::Var(dst.clone());
                    current_src == dst || current_dst == dst
                });
            }
            _ => {}
        }
    }
    annotations.annotate_block(node.id, current_reaching_copies);
}
