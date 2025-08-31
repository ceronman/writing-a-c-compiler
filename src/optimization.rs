use std::ops::{Neg, Not};
use Instruction::*;
use crate::tacky::{Constant, Instruction, Program, TopLevel, UnaryOp, Val};

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
                f.body = constant_fold(&f.body);
            }
        }
    }
    program
}

fn constant_fold(old: &[Instruction]) -> Vec<Instruction> {
    let mut new = Vec::with_capacity(old.len());

    for instruction in old {
        match instruction {
            Unary { op: UnaryOp::Complement, src: Val::Constant(src), dst } => {
                let complement = match src {
                    Constant::Int(value) => Constant::Int(value.not()),
                    Constant::UInt(value) => Constant::UInt(value.not()),
                    Constant::Long(value) => Constant::Long(value.not()),
                    Constant::ULong(value) => Constant::ULong(value.not()),
                    Constant::Char(value) => Constant::Char(value.not()),
                    Constant::UChar(value) => Constant::UChar(value.not()),

                    Constant::Double(_) => unreachable!("Type checker should prevent complement of doubles"),
                };
                new.push(Copy { src: Val::Constant(complement), dst: dst.clone()});
            }
            Unary { op: UnaryOp::Negate, src: Val::Constant(src), dst } => {
                let negated = match src {
                    Constant::Int(value) => Constant::Int(-value),
                    Constant::UInt(value) => Constant::UInt(value.wrapping_neg()),
                    Constant::Long(value) => Constant::Long(value.neg()),
                    Constant::ULong(value) => Constant::ULong(value.wrapping_neg()),
                    Constant::Char(value) => Constant::Char(value.neg()),
                    Constant::UChar(value) => Constant::UChar(value.wrapping_neg()),
                    Constant::Double(value) => Constant::Double(value.neg()),
                };
                new.push(Copy { src: Val::Constant(negated), dst: dst.clone()});
            }
            Unary { op: UnaryOp::Not, src: Val::Constant(src), dst } => {
                let is_zero = match src {
                    Constant::Int(value) => *value == 0,
                    Constant::UInt(value) => *value == 0,
                    Constant::Long(value) => *value == 0,
                    Constant::ULong(value) => *value == 0,
                    Constant::Char(value) => *value == 0,
                    Constant::UChar(value) => *value == 0,
                    Constant::Double(value) => *value == 0.0, // TODO: there is more to this
                };
                if is_zero {
                    new.push(Copy { src: Val::Constant(Constant::Int(1)), dst: dst.clone() });
                } else {
                    new.push(Copy { src: Val::Constant(Constant::Int(0)), dst: dst.clone() });
                }
            }
            // Binary { op, src1, src2, dst } => {}
            // Jump { target } => {}
            // JumpIfZero { cond, target } => {}
            // JumpIfNotZero { .. } => {}
            _ => new.push(instruction.clone()),
        }
    }

    new
}
