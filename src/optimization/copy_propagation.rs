use std::collections::HashSet;
use crate::optimization::cfg::{Annotation, TackyCfg, TackyNode};
use crate::semantic::{Attributes, SemanticData};
use crate::tacky::{Instruction, Val};

#[derive(Clone)]
struct Copies(HashSet<Instruction>);

impl Copies {
    fn empty() -> Self {
        Self(HashSet::new())
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

fn meet_operator(
    annotations: &mut ReachingCopies,
    cfg: &TackyCfg,
    node: &TackyNode,
    all_copies: Copies,
) -> Copies {
    let mut incoming_copies = all_copies.clone();
    for pred_id in &node.predecessors {
        if pred_id == &cfg.entry_id() {
            return Copies::empty()
        }
        if pred_id == &cfg.exit_id() {
            panic!("Malformed cfg")
        }
        let pred_copies = annotations.get_block(pred_id);
        incoming_copies.intersect(pred_copies);
    }
    incoming_copies
}

fn transfer_function(
    annotations: &mut ReachingCopies,
    node: &TackyNode,
    initial_reaching_copies: Copies,
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
