pub mod cfg;
mod constant_folding;
mod copy_propagation;
mod unreachable_code;
mod dead_store_elimination;

use crate::optimization::constant_folding::constant_fold;
use crate::optimization::copy_propagation::copy_propagation;
use crate::optimization::unreachable_code::remove_unreachable_code;
use crate::semantic::{Attributes, SemanticData};
use crate::tacky;
use crate::tacky::{Instruction, Val};
use std::collections::HashSet;
use crate::optimization::dead_store_elimination::dead_store_elimination;

#[derive(Default)]
pub struct OptimizationFlags {
    pub fold_constants: bool,
    pub propagate_copies: bool,
    pub eliminate_unreachable_code: bool,
    pub eliminate_dead_stores: bool,
    pub optimize: bool,
    pub trace: bool,
}

pub fn optimize(mut program: tacky::Program, flags: &OptimizationFlags) -> tacky::Program {
    for top_level in &mut program.top_level {
        if let tacky::TopLevel::Function(f) = top_level {
            loop {
                if flags.trace {
                    println!();
                    println!(">>>> OPTIMIZATION ITERATION <<<<");
                    println!();
                }
                let mut optimized = f.body.clone();

                let aliased_vars = address_taken_analysis(&optimized);
                let static_vars = static_vars(&program.semantics);
                let aliased_and_static_vars: HashSet<Val> = aliased_vars.union(&static_vars).cloned().collect();
                if flags.fold_constants || flags.optimize {
                    optimized = constant_fold(&optimized, &program.semantics, false);
                }
                // TODO: pass same cfg to all passes
                if flags.eliminate_unreachable_code || flags.optimize {
                    optimized = remove_unreachable_code(&optimized, false);
                }
                if flags.propagate_copies || flags.optimize {
                    optimized = copy_propagation(
                        &optimized,
                        &aliased_and_static_vars,
                        &program.semantics,
                        flags.trace,
                    );
                }
                if flags.eliminate_dead_stores || flags.optimize {
                    optimized = dead_store_elimination(
                        &optimized,
                        &static_vars,
                        &aliased_vars,
                        flags.trace,
                    )
                }

                if optimized == f.body {
                    break;
                }
                f.body = optimized;
            }
        }
    }
    program
}

fn address_taken_analysis(instructions: &[tacky::Instruction]) -> HashSet<Val> {
    let mut result = HashSet::new();
    for instruction in instructions {
        if let Instruction::GetAddress { src, .. } = instruction {
            result.insert(src.clone());
        }
    }
    result
}

fn static_vars(semantics: &SemanticData) -> HashSet<Val> {
    let mut result = HashSet::new();
    for (name, data) in semantics.symbols.iter() {
        if let Attributes::Static { .. } = data.attrs {
            result.insert(Val::Var(name.clone()));
        }
    }
    result
}