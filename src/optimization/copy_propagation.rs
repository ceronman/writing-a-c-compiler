use std::collections::{HashSet, VecDeque};
use crate::optimization::cfg::{Annotation, Cfg, NodeId, TackyCfg, TackyNode};
use crate::semantic::{Attributes, SemanticData};
use crate::tacky;
use crate::tacky::{Instruction, Val};

#[derive(Clone, PartialEq, Eq)]
struct Copies(HashSet<Instruction>);

impl Copies {
    fn empty() -> Self {
        Self(HashSet::new())
    }

    fn from_cfg(cfg: &TackyCfg) -> Self {
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

    fn remove_if(&mut self, condition: impl Fn(&Val, &Val) -> bool) {
        self.0.retain(|copy| {
            let Instruction::Copy { src, dst } = copy
            else {
                panic!("Expected copy instruction")
            };
            !condition(src, dst)
        });
    }

    fn contains(&self, instruction: &Instruction) -> bool {
        self.0.contains(instruction)
    }
}

type ReachingCopies = Annotation<Copies>;

// TODO: Take Cfg instead?
pub fn find_reaching_copies(instructions: &[tacky::Instruction], semantics: &SemanticData, trace: bool) {
    let mut cfg = Cfg::new(instructions);
    let all_copies = Copies::from_cfg(&cfg);
    let mut annotations = ReachingCopies::new();

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
        let incoming_copies = meet_operator(&mut annotations, &cfg, node, &all_copies);
        transfer_function(&mut annotations, node, &incoming_copies, semantics);
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
}

fn rewrite_instructions(cfg: &mut TackyCfg, annotations: &ReachingCopies) {
    for node_id in cfg.all_ids() {
        let node = cfg.get_node(node_id);
        let mut new_instructions = Vec::new();
        for (i, instruction) in node.instructions.iter().enumerate() {
            let reaching_copies = annotations.get_instruction_annotation(node.id, i);
            let mut new_instruction = instruction.clone();
            match &mut new_instruction {
                Instruction::Copy { src, dst } => {
                    if reaching_copies.contains(instruction) {
                        continue; // Do not add instruction to new instructions
                    }
                    *src = replace_operand(src.clone(), reaching_copies);
                    new_instructions.push(new_instruction);
                }
                _ => todo!()
            }
        }
    }
}

fn replace_operand(op: Val, copies: &Copies) -> Val {
    match &op {
        Val::Constant(_) => return op,
        Val::Var(_) => {
            for copy in copies.0.iter() {
                if let Instruction::Copy { src, dst } = copy {
                    if *dst == op {
                        return src.clone();
                    }
                }
            }
        }
    };
    op
}

fn meet_operator(
    annotations: &mut ReachingCopies,
    cfg: &TackyCfg,
    node: &TackyNode,
    all_copies: &Copies,
) -> Copies {
    let mut incoming_copies = all_copies.clone();
    for pred_id in &node.predecessors {
        if pred_id == &cfg.entry_id() {
            return Copies::empty()
        }
        let pred_copies = annotations.get_block_annotation(pred_id);
        incoming_copies.intersect(pred_copies);
    }
    incoming_copies
}

fn transfer_function(
    annotations: &mut ReachingCopies,
    node: &TackyNode,
    initial_reaching_copies: &Copies,
    semantics: &SemanticData,
) {
    let mut current_reaching_copies = initial_reaching_copies.clone();
    for (i, instruction) in node.instructions.iter().enumerate() {
        annotations.annotate_instruction(node.id, i, current_reaching_copies.clone());
        match instruction {
            Instruction::Copy { src, dst } => {
                if current_reaching_copies.contains(instruction) {
                    continue;
                }

                current_reaching_copies.remove_if(|current_src, current_dst| {
                    current_src == dst || current_dst == dst
                });
            }
            Instruction::FnCall { dst: Some(dst), .. } => {
                current_reaching_copies.remove_if(|current_src, current_dst| {
                    current_dst.is_static(semantics)
                        || current_src.is_static(semantics)
                        || current_src == dst
                        || current_dst == dst
                });
            }

            Instruction::Binary { dst, .. }
            | Instruction::Unary { dst, .. } => {
                current_reaching_copies.remove_if(|current_src, current_dst| {
                    current_src == dst || current_dst == dst
                });
            }
            _ => continue,
        }
    }
    annotations.annotate_block(node.id, current_reaching_copies);
}

impl Val {
    fn is_static(&self, semantics: &SemanticData) -> bool {
        match self {
            Val::Constant(_) => false,
            Val::Var(name) => {
                let symbol_data = semantics
                    .symbols
                    .get(name)
                    .expect("Variable without symbol data");
                matches!(symbol_data.attrs, Attributes::Static { .. })
            }
        }
    }
}
