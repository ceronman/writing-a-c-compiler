pub mod cfg;
mod constant_folding;
mod unreachable_code;
mod copy_propagation;

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
    pub trace: bool,
}

pub fn optimize(mut program: tacky::Program, flags: &OptimizationFlags) -> tacky::Program {
    for top_level in &mut program.top_level {
        if let tacky::TopLevel::Function(f) = top_level {
            loop {
                if flags.trace {
                    println!();
                    println!("OPTIMIZATION ITERATION");
                    println!();
                }
                let mut optimized = f.body.clone();
                if flags.fold_constants || flags.optimize {
                    optimized = constant_fold(&optimized, &program.semantics, flags.trace);
                }
                if flags.eliminate_unreachable_code || flags.optimize {
                    optimized = remove_unreachable_code(&optimized, flags.trace);
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
