use crate::optimization::cfg::{Annotation, TackyNode};
use crate::semantic::{Attributes, SemanticData};
use crate::tacky::{Instruction, Val};

// TODO: maybe use set instead?
type ReachingCopies = Vec<Instruction>;

type ReachingCopiesAnnotations = Annotation<ReachingCopies>;

fn transfer_function(
    annotations: &mut ReachingCopiesAnnotations,
    node: &TackyNode,
    initial_reaching_copies: Vec<Instruction>,
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

                remove_if(&mut current_reaching_copies, |current_src, current_dst|{
                    current_src == dst || current_dst == dst
                });
            }
            Instruction::FnCall { dst: Some(dst), .. } => {
                remove_if(&mut current_reaching_copies, |current_src, current_dst|{
                    current_dst.is_static(semantics)
                        || current_src.is_static(semantics)
                        || current_src == dst
                        || current_dst == dst
                });
            }

            Instruction::Binary { dst, .. }
            | Instruction::Unary { dst, .. } => {
                remove_if(&mut current_reaching_copies, |current_src, current_dst|{
                    current_src == dst || current_dst == dst
                });
            }
            _ => continue,
        }
    }
    annotations.annotate_block(node.id, current_reaching_copies);
}

fn remove_if(copies: &mut ReachingCopies, condition: impl Fn(&Val, &Val) -> bool) {
    copies.retain(|copy| {
        let Instruction::Copy { src, dst } = copy
        else {
            panic!("Expected copy instruction")
        };
        !condition(src, dst)
    });
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
