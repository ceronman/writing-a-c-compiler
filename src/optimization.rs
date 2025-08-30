use crate::tacky::Program;

#[derive(Default)]
pub struct OptimizationFlags {
    pub fold_constants: bool,
    pub propagate_copies: bool,
    pub eliminate_unreachable_code: bool,
    pub eliminate_dead_stores: bool,
    pub optimize: bool,
}

pub fn optimize(program: Program, _flags: &OptimizationFlags) -> Program {
    program
}
