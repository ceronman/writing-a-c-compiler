use crate::ast::Constant::{Char, Double, Int, Long, UChar, UInt, ULong};
use crate::semantic::{SemanticData, Type};
use crate::tacky::pretty::pp_instruction;
use crate::tacky::{BinaryOp, Instruction, UnaryOp, Val};
use Instruction::{
    Binary, DoubleToInt, DoubleToUInt, IntToDouble, Jump, JumpIfNotZero, JumpIfZero, SignExtend,
    Truncate, UIntToDouble, Unary, ZeroExtend,
};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

pub fn constant_fold(
    old: &[Instruction],
    semantics: &SemanticData,
    trace: bool,
) -> Vec<Instruction> {
    if trace {
        println!("=======================");
        println!("Constant Folding");
        println!("=======================");
        dump_instructions("INITIAL", old);
    }

    let mut new = Vec::with_capacity(old.len());

    for instruction in old {
        match instruction {
            Unary {
                op: UnaryOp::Complement,
                src: Val::Constant(src),
                dst,
            } => {
                let complement = match src {
                    Int(value) => Int(value.not()),
                    UInt(value) => UInt(value.not()),
                    Long(value) => Long(value.not()),
                    ULong(value) => ULong(value.not()),
                    Char(value) => Char(value.not()),
                    UChar(value) => UChar(value.not()),

                    Double(_) => unreachable!("Type checker should prevent complement of doubles"),
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(complement),
                    dst: dst.clone(),
                });
            }
            Unary {
                op: UnaryOp::Negate,
                src: Val::Constant(src),
                dst,
            } => {
                let negated = match src {
                    Int(value) => Int(-value),
                    UInt(value) => UInt(value.wrapping_neg()),
                    Long(value) => Long(value.neg()),
                    ULong(value) => ULong(value.wrapping_neg()),
                    Char(value) => Char(value.neg()),
                    UChar(value) => UChar(value.wrapping_neg()),
                    Double(value) => Double(value.neg()),
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(negated),
                    dst: dst.clone(),
                });
            }
            Unary {
                op: UnaryOp::Not,
                src: Val::Constant(src),
                dst,
            } => {
                let is_zero = match src {
                    Int(value) => *value == 0,
                    UInt(value) => *value == 0,
                    Long(value) => *value == 0,
                    ULong(value) => *value == 0,
                    Char(value) => *value == 0,
                    UChar(value) => *value == 0,
                    Double(value) => *value == 0.0, // TODO: there is more to this
                };
                if is_zero {
                    new.push(Instruction::Copy {
                        src: Val::Constant(Int(1)),
                        dst: dst.clone(),
                    });
                } else {
                    new.push(Instruction::Copy {
                        src: Val::Constant(Int(0)),
                        dst: dst.clone(),
                    });
                }
            }
            Binary {
                op: BinaryOp::Add,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.wrapping_add(*right)),
                    (UInt(left), UInt(right)) => UInt(left.wrapping_add(*right)),
                    (Long(left), Long(right)) => Long(left.wrapping_add(*right)),
                    (ULong(left), ULong(right)) => ULong(left.wrapping_add(*right)),
                    (Char(left), Char(right)) => Char(left.wrapping_add(*right)),
                    (UChar(left), UChar(right)) => UChar(left.wrapping_add(*right)),
                    (Double(left), Double(right)) => Double(left.add(*right)),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }
            Binary {
                op: BinaryOp::Subtract,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.wrapping_sub(*right)),
                    (UInt(left), UInt(right)) => UInt(left.wrapping_sub(*right)),
                    (Long(left), Long(right)) => Long(left.wrapping_sub(*right)),
                    (ULong(left), ULong(right)) => ULong(left.wrapping_sub(*right)),
                    (Char(left), Char(right)) => Char(left.wrapping_sub(*right)),
                    (UChar(left), UChar(right)) => UChar(left.wrapping_sub(*right)),
                    (Double(left), Double(right)) => Double(left.sub(*right)),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }
            Binary {
                op: BinaryOp::Multiply,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.wrapping_mul(*right)),
                    (UInt(left), UInt(right)) => UInt(left.wrapping_mul(*right)),
                    (Long(left), Long(right)) => Long(left.wrapping_mul(*right)),
                    (ULong(left), ULong(right)) => ULong(left.wrapping_mul(*right)),
                    (Char(left), Char(right)) => Char(left.wrapping_mul(*right)),
                    (UChar(left), UChar(right)) => UChar(left.wrapping_mul(*right)),
                    (Double(left), Double(right)) => Double(left.mul(*right)),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }
            Binary {
                op: BinaryOp::Divide,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                if right.is_int() && right.is_zero() {
                    new.push(instruction.clone())
                } else {
                    let result = match (left, right) {
                        (Int(left), Int(right)) => Int(left.div(*right)),
                        (UInt(left), UInt(right)) => UInt(left.wrapping_div(*right)),
                        (Long(left), Long(right)) => Long(left.div(*right)),
                        (ULong(left), ULong(right)) => ULong(left.wrapping_div(*right)),
                        (Char(left), Char(right)) => Char(left.div(*right)),
                        (UChar(left), UChar(right)) => UChar(left.wrapping_div(*right)),
                        (Double(left), Double(right)) => Double(left.div(*right)),
                        _ => unreachable!(
                            "Type checker should prevent binary ops with different types"
                        ),
                    };
                    new.push(Instruction::Copy {
                        src: Val::Constant(result),
                        dst: dst.clone(),
                    });
                }
            }

            Binary {
                op: BinaryOp::Reminder,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                if right.is_int() && right.is_zero() {
                    new.push(instruction.clone())
                } else {
                    let result = match (left, right) {
                        (Int(left), Int(right)) => Int(left.rem(*right)),
                        (UInt(left), UInt(right)) => UInt(left.wrapping_rem(*right)),
                        (Long(left), Long(right)) => Long(left.rem(*right)),
                        (ULong(left), ULong(right)) => ULong(left.wrapping_rem(*right)),
                        (Char(left), Char(right)) => Char(left.rem(*right)),
                        (UChar(left), UChar(right)) => UChar(left.wrapping_rem(*right)),
                        (Double(left), Double(right)) => Double(left.rem(*right)),
                        _ => unreachable!(
                            "Type checker should prevent binary ops with different types"
                        ),
                    };
                    new.push(Instruction::Copy {
                        src: Val::Constant(result),
                        dst: dst.clone(),
                    });
                }
            }

            Binary {
                op: BinaryOp::BinAnd,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.bitand(*right)),
                    (UInt(left), UInt(right)) => UInt(left.bitand(*right)),
                    (Long(left), Long(right)) => Long(left.bitand(*right)),
                    (ULong(left), ULong(right)) => ULong(left.bitand(*right)),
                    (Char(left), Char(right)) => Char(left.bitand(*right)),
                    (UChar(left), UChar(right)) => UChar(left.bitand(*right)),
                    (Double(_), Double(_)) => {
                        unreachable!("Type checker should prevent binary ops with doubles")
                    }
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::BinOr,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.bitor(*right)),
                    (UInt(left), UInt(right)) => UInt(left.bitor(*right)),
                    (Long(left), Long(right)) => Long(left.bitor(*right)),
                    (ULong(left), ULong(right)) => ULong(left.bitor(*right)),
                    (Char(left), Char(right)) => Char(left.bitor(*right)),
                    (UChar(left), UChar(right)) => UChar(left.bitor(*right)),
                    (Double(_), Double(_)) => {
                        unreachable!("Type checker should prevent binary ops with doubles")
                    }
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::BinXor,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.bitxor(*right)),
                    (UInt(left), UInt(right)) => UInt(left.bitxor(*right)),
                    (Long(left), Long(right)) => Long(left.bitxor(*right)),
                    (ULong(left), ULong(right)) => ULong(left.bitxor(*right)),
                    (Char(left), Char(right)) => Char(left.bitxor(*right)),
                    (UChar(left), UChar(right)) => UChar(left.bitxor(*right)),
                    (Double(_), Double(_)) => {
                        unreachable!("Type checker should prevent binary ops with doubles")
                    }
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::ShiftLeft,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), right) if right.is_int() => Int(left.shl(right.as_u64())),
                    (UInt(left), right) if right.is_int() => UInt(left.shl(right.as_u64())),
                    (Long(left), right) if right.is_int() => Long(left.shl(right.as_u64())),
                    (ULong(left), right) if right.is_int() => ULong(left.shl(right.as_u64())),
                    (Char(left), right) if right.is_int() => Char(left.shl(right.as_u64())),
                    (UChar(left), right) if right.is_int() => UChar(left.shl(right.as_u64())),
                    (Double(_), Double(_)) => {
                        unreachable!("Type checker should prevent binary ops with doubles")
                    }
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::ShiftRight,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), right) if right.is_int() => Int(left.shr(right.as_u64())),
                    (UInt(left), right) if right.is_int() => UInt(left.shr(right.as_u64())),
                    (Long(left), right) if right.is_int() => Long(left.shr(right.as_u64())),
                    (ULong(left), right) if right.is_int() => ULong(left.shr(right.as_u64())),
                    (Char(left), right) if right.is_int() => Char(left.shr(right.as_u64())),
                    (UChar(left), right) if right.is_int() => UChar(left.shr(right.as_u64())),
                    (Double(_), Double(_)) => {
                        unreachable!("Type checker should prevent binary ops with doubles")
                    }
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::Equal,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.eq(right) as i32),
                    (UInt(left), UInt(right)) => Int(left.eq(right) as i32),
                    (Long(left), Long(right)) => Int(left.eq(right) as i32),
                    (ULong(left), ULong(right)) => Int(left.eq(right) as i32),
                    (Char(left), Char(right)) => Int(left.eq(right) as i32),
                    (UChar(left), UChar(right)) => Int(left.eq(right) as i32),
                    (Double(left), Double(right)) => Int(left.eq(right) as i32),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::NotEqual,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.ne(right) as i32),
                    (UInt(left), UInt(right)) => Int(left.ne(right) as i32),
                    (Long(left), Long(right)) => Int(left.ne(right) as i32),
                    (ULong(left), ULong(right)) => Int(left.ne(right) as i32),
                    (Char(left), Char(right)) => Int(left.ne(right) as i32),
                    (UChar(left), UChar(right)) => Int(left.ne(right) as i32),
                    (Double(left), Double(right)) => Int(left.ne(right) as i32),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::LessThan,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.lt(right) as i32),
                    (UInt(left), UInt(right)) => Int(left.lt(right) as i32),
                    (Long(left), Long(right)) => Int(left.lt(right) as i32),
                    (ULong(left), ULong(right)) => Int(left.lt(right) as i32),
                    (Char(left), Char(right)) => Int(left.lt(right) as i32),
                    (UChar(left), UChar(right)) => Int(left.lt(right) as i32),
                    (Double(left), Double(right)) => Int(left.lt(right) as i32),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::LessOrEqual,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.le(right) as i32),
                    (UInt(left), UInt(right)) => Int(left.le(right) as i32),
                    (Long(left), Long(right)) => Int(left.le(right) as i32),
                    (ULong(left), ULong(right)) => Int(left.le(right) as i32),
                    (Char(left), Char(right)) => Int(left.le(right) as i32),
                    (UChar(left), UChar(right)) => Int(left.le(right) as i32),
                    (Double(left), Double(right)) => Int(left.le(right) as i32),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::GreaterThan,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.gt(right) as i32),
                    (UInt(left), UInt(right)) => Int(left.gt(right) as i32),
                    (Long(left), Long(right)) => Int(left.gt(right) as i32),
                    (ULong(left), ULong(right)) => Int(left.gt(right) as i32),
                    (Char(left), Char(right)) => Int(left.gt(right) as i32),
                    (UChar(left), UChar(right)) => Int(left.gt(right) as i32),
                    (Double(left), Double(right)) => Int(left.gt(right) as i32),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            Binary {
                op: BinaryOp::GreaterOrEqual,
                src1: Val::Constant(left),
                src2: Val::Constant(right),
                dst,
            } => {
                let result = match (left, right) {
                    (Int(left), Int(right)) => Int(left.ge(right) as i32),
                    (UInt(left), UInt(right)) => Int(left.ge(right) as i32),
                    (Long(left), Long(right)) => Int(left.ge(right) as i32),
                    (ULong(left), ULong(right)) => Int(left.ge(right) as i32),
                    (Char(left), Char(right)) => Int(left.ge(right) as i32),
                    (UChar(left), UChar(right)) => Int(left.ge(right) as i32),
                    (Double(left), Double(right)) => Int(left.ge(right) as i32),
                    _ => {
                        unreachable!("Type checker should prevent binary ops with different types")
                    }
                };
                new.push(Instruction::Copy {
                    src: Val::Constant(result),
                    dst: dst.clone(),
                });
            }

            JumpIfZero {
                cond: Val::Constant(cond),
                target,
            } => {
                if cond.is_zero() {
                    new.push(Jump {
                        target: target.clone(),
                    });
                }
            }
            JumpIfNotZero {
                cond: Val::Constant(cond),
                target,
            } => {
                if !cond.is_zero() {
                    new.push(Jump {
                        target: target.clone(),
                    });
                }
            }
            Truncate {
                src: Val::Constant(c),
                dst,
            }
            | SignExtend {
                src: Val::Constant(c),
                dst,
            }
            | ZeroExtend {
                src: Val::Constant(c),
                dst,
            }
            | DoubleToInt {
                src: Val::Constant(c),
                dst,
            }
            | DoubleToUInt {
                src: Val::Constant(c),
                dst,
            }
            | IntToDouble {
                src: Val::Constant(c),
                dst,
            }
            | UIntToDouble {
                src: Val::Constant(c),
                dst,
            }
            | Instruction::Copy {
                src: Val::Constant(c),
                dst,
            } => {
                let dst_ty = semantics.val_ty(dst);
                let dst_ty = if dst_ty.is_pointer() {
                    Type::ULong
                } else {
                    dst_ty
                }; // Hacks for now
                let new_constant = c
                    .cast(&dst_ty)
                    .expect("Type checker should prevent casts with incorrect types");
                new.push(Instruction::Copy {
                    src: Val::Constant(new_constant),
                    dst: dst.clone(),
                });
            }
            _ => new.push(instruction.clone()),
        }
    }

    if trace {
        dump_instructions("AFTER FOLD", old);
    }

    new
}

fn dump_instructions(version: &str, instructions: &[Instruction]) {
    println!("{version} instructions:");
    for instruction in instructions {
        let mut result = String::new();
        pp_instruction(&mut result, instruction).unwrap();
        println!(" {}", result.trim());
    }
}
