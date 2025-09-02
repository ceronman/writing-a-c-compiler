mod constant_folding;

use crate::tacky::{Program, TopLevel};
use crate::optimization::constant_folding::constant_fold;

#[derive(Default)]
pub struct OptimizationFlags {
    pub fold_constants: bool,
    pub propagate_copies: bool,
    pub eliminate_unreachable_code: bool,
    pub eliminate_dead_stores: bool,
    pub optimize: bool,
}

pub fn optimize(mut program: Program, flags: &OptimizationFlags) -> Program {
    for top_level in &mut program.top_level {
        if let TopLevel::Function(f) = top_level {
            if flags.fold_constants || flags.optimize {
                f.body = constant_fold(&f.body, &program.semantics);
            }
        }
    }
    program
}

