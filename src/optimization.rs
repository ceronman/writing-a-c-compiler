pub mod cfg;
mod constant_folding;
mod copy_propagation;
mod dead_store_elimination;
mod unreachable_code;

use crate::optimization::cfg::Cfg;
use crate::optimization::constant_folding::constant_fold;
use crate::optimization::copy_propagation::copy_propagation;
use crate::optimization::dead_store_elimination::dead_store_elimination;
use crate::optimization::unreachable_code::remove_unreachable_code;
use crate::semantic::{Attributes, SemanticData, Type};
use crate::tacky;
use crate::tacky::{Instruction, Val};
use std::collections::HashSet;

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
                let var_data = VariableData::new(&optimized, &program.semantics);

                if flags.fold_constants || flags.optimize {
                    optimized = constant_fold(&optimized, &var_data, flags.trace);
                }

                let mut cfg = Cfg::new(&optimized);
                if flags.eliminate_unreachable_code || flags.optimize {
                    remove_unreachable_code(&mut cfg, flags.trace);
                }
                if flags.propagate_copies || flags.optimize {
                    copy_propagation(&mut cfg, &var_data, flags.trace);
                }
                if flags.eliminate_dead_stores || flags.optimize {
                    dead_store_elimination(&mut cfg, &var_data, flags.trace)
                }

                optimized = cfg.dump();

                if optimized == f.body {
                    break;
                }
                f.body = optimized;
            }
        }
    }
    program
}

struct VariableData<'a> {
    aliased_vars: HashSet<Val>,
    static_vars: HashSet<Val>,
    semantics: &'a SemanticData,
}

impl<'a> VariableData<'a> {
    fn new(instructions: &[Instruction], semantic_data: &'a SemanticData) -> Self {
        VariableData {
            aliased_vars: Self::find_aliased_vars(instructions),
            static_vars: Self::find_static_vars(semantic_data),
            semantics: semantic_data,
        }
    }

    fn find_aliased_vars(instructions: &[Instruction]) -> HashSet<Val> {
        let mut result = HashSet::new();
        for instruction in instructions {
            if let Instruction::GetAddress { src, .. } = instruction {
                result.insert(src.clone());
            }
        }
        result
    }

    fn find_static_vars(semantics: &SemanticData) -> HashSet<Val> {
        let mut result = HashSet::new();
        for (name, data) in semantics.symbols.iter() {
            if let Attributes::Static { .. } = data.attrs {
                result.insert(Val::Var(name.clone()));
            }
        }
        result
    }

    pub fn is_aliased(&self, val: &Val) -> bool {
        self.aliased_vars.contains(val)
    }

    pub fn is_static(&self, val: &Val) -> bool {
        self.static_vars.contains(val)
    }

    pub fn is_aliased_or_static(&self, val: &Val) -> bool {
        self.is_aliased(val) || self.is_static(val)
    }

    pub fn ty(&self, val: &Val) -> Type {
        self.semantics.val_ty(val)
    }
}
