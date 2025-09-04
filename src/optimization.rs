mod constant_folding;
pub mod cfg;
mod unreachable_code;

use crate::optimization::constant_folding::constant_fold;
use crate::optimization::unreachable_code::remove_unreachable_blocks;
use crate::tacky::{Program, TopLevel};
use crate::tacky;

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

pub fn debug_cfg(program: Program) {
    for program in program.top_level {
        if let tacky::TopLevel::Function(f) = program {
            let cfg = cfg::tacky_to_cfg(&f.body);
            println!("function {} initial:\n {cfg:?}", f.name);
            let cfg = remove_unreachable_blocks(cfg);
            println!("function {} unreachable blocks:\n {cfg:?}", f.name);
        }
    }
}
