pub mod cfg;
mod constant_folding;
mod unreachable_code;

use crate::optimization::cfg::Cfg;
use crate::optimization::constant_folding::constant_fold;
use crate::optimization::unreachable_code::{
    remove_unreachable_blocks, remove_unreachable_code, remove_useless_jumps, remove_useless_labels,
};
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

pub fn debug_cfg(program: tacky::Program) {
    for program in program.top_level {
        if let tacky::TopLevel::Function(f) = program {
            let cfg = Cfg::new(&f.body);
            println!("[{}] initial:\n {cfg:?}", f.name);

            let cfg = remove_unreachable_blocks(cfg);
            println!("[{}] unreachable blocks:\n {cfg:?}", f.name);

            let cfg = remove_useless_jumps(cfg);
            println!("[{}] useless jumps:\n {cfg:?}", f.name);

            let cfg = remove_useless_labels(cfg);
            println!("[{}] useless labels:\n {cfg:?}", f.name);
        }
    }
}
