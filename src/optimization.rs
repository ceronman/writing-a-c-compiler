pub mod cfg;
mod constant_folding;
mod unreachable_code;

use crate::optimization::constant_folding::constant_fold;
use crate::optimization::unreachable_code::remove_unreachable_code;
use crate::tacky;

#[derive(Default)]
pub struct OptimizationFlags {
    pub fold_constants: bool,
    pub propagate_copies: bool,
    pub eliminate_unreachable_code: bool,
    pub eliminate_dead_stores: bool,
    pub optimize: bool,
}

pub fn optimize(mut program: tacky::Program, flags: &OptimizationFlags) -> tacky::Program {
    for top_level in &mut program.top_level {
        if let tacky::TopLevel::Function(f) = top_level {
            loop {
                let mut optimized = f.body.clone();
                if flags.fold_constants || flags.optimize {
                    optimized = constant_fold(&optimized, &program.semantics);
                }
                if flags.eliminate_unreachable_code || flags.optimize {
                    optimized = remove_unreachable_code(&optimized);
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
