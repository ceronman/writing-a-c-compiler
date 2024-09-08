use crate::symbol::Symbol;
use crate::tacky;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub function_definition: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Symbol,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Unary(UnaryOp, Operand),
    Binary(BinaryOp, Operand, Operand),
    Cmp(Operand, Operand),
    Idiv(Operand),
    Sal(Operand),
    Sar(Operand),
    Cdq,
    Jmp(Symbol),
    JmpCC(CondCode, Symbol),
    SetCC(CondCode, Operand),
    Label(Symbol),
    AllocateStack(u64),
    Ret,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
    Inc,
    Dec,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i64),
    Reg(Reg),
    Pseudo(Symbol),
    Stack(u64),
}

#[derive(Debug)]
pub enum Reg {
    Ax,
    Cx,
    Dx,
    R10,
    R11,
}

#[derive(Debug)]
pub enum CondCode {
    E,
    NE,
    G,
    GE,
    L,
    LE,
}

pub fn generate(program: &tacky::Program) -> Program {
    let mut instructions = Vec::new();
    let function = &program.function;

    for tacky_instruction in &program.function.body {
        match tacky_instruction {
            tacky::Instruction::Return(val) => {
                instructions.push(Instruction::Mov(val.to_asm(), Operand::Reg(Reg::Ax)));
                instructions.push(Instruction::Ret);
            }

            tacky::Instruction::Unary { op, src, dst } => match op {
                tacky::UnaryOp::Not => {
                    instructions.push(Instruction::Cmp(Operand::Imm(0), src.to_asm()));
                    instructions.push(Instruction::Mov(Operand::Imm(0), dst.to_asm()));
                    instructions.push(Instruction::SetCC(CondCode::E, dst.to_asm()));
                }
                _ => {
                    instructions.push(Instruction::Mov(src.to_asm(), dst.to_asm()));
                    instructions.push(Instruction::Unary(op.to_asm(), dst.to_asm()));
                }
            },

            tacky::Instruction::Binary {
                op,
                src1,
                src2,
                dst,
            } => match op {
                tacky::BinaryOp::Divide => {
                    instructions.push(Instruction::Mov(src1.to_asm(), Operand::Reg(Reg::Ax)));
                    instructions.push(Instruction::Cdq);
                    instructions.push(Instruction::Idiv(src2.to_asm()));
                    instructions.push(Instruction::Mov(Operand::Reg(Reg::Ax), dst.to_asm()));
                }
                tacky::BinaryOp::Reminder => {
                    instructions.push(Instruction::Mov(src1.to_asm(), Operand::Reg(Reg::Ax)));
                    instructions.push(Instruction::Cdq);
                    instructions.push(Instruction::Idiv(src2.to_asm()));
                    instructions.push(Instruction::Mov(Operand::Reg(Reg::Dx), dst.to_asm()));
                }
                tacky::BinaryOp::ShiftLeft => {
                    instructions.push(Instruction::Mov(src1.to_asm(), dst.to_asm()));
                    instructions.push(Instruction::Mov(src2.to_asm(), Operand::Reg(Reg::Cx)));
                    instructions.push(Instruction::Sal(dst.to_asm()));
                }
                tacky::BinaryOp::ShiftRight => {
                    instructions.push(Instruction::Mov(src1.to_asm(), dst.to_asm()));
                    instructions.push(Instruction::Mov(src2.to_asm(), Operand::Reg(Reg::Cx)));
                    instructions.push(Instruction::Sar(dst.to_asm()));
                }
                tacky::BinaryOp::Equal
                | tacky::BinaryOp::NotEqual
                | tacky::BinaryOp::GreaterThan
                | tacky::BinaryOp::GreaterOrEqual
                | tacky::BinaryOp::LessThan
                | tacky::BinaryOp::LessOrEqual => {
                    instructions.push(Instruction::Cmp(src2.to_asm(), src1.to_asm()));
                    instructions.push(Instruction::Mov(Operand::Imm(0), dst.to_asm()));
                    let cond = match op {
                        tacky::BinaryOp::Equal => CondCode::E,
                        tacky::BinaryOp::NotEqual => CondCode::NE,
                        tacky::BinaryOp::GreaterThan => CondCode::G,
                        tacky::BinaryOp::GreaterOrEqual => CondCode::GE,
                        tacky::BinaryOp::LessThan => CondCode::L,
                        tacky::BinaryOp::LessOrEqual => CondCode::LE,
                        _ => unreachable!(),
                    };
                    instructions.push(Instruction::SetCC(cond, dst.to_asm()));
                }
                _ => {
                    instructions.push(Instruction::Mov(src1.to_asm(), dst.to_asm()));
                    instructions.push(Instruction::Binary(
                        op.to_asm(),
                        src2.to_asm(),
                        dst.to_asm(),
                    ));
                }
            },

            tacky::Instruction::Jump { target } => {
                instructions.push(Instruction::Jmp(target.clone()));
            }
            tacky::Instruction::JumpIfZero { cond, target } => {
                instructions.push(Instruction::Cmp(Operand::Imm(0), cond.to_asm()));
                instructions.push(Instruction::JmpCC(CondCode::E, target.clone()));
            }
            tacky::Instruction::JumpIfNotZero { cond, target } => {
                instructions.push(Instruction::Cmp(Operand::Imm(0), cond.to_asm()));
                instructions.push(Instruction::JmpCC(CondCode::NE, target.clone()));
            }
            tacky::Instruction::Copy { src, dst } => {
                instructions.push(Instruction::Mov(src.to_asm(), dst.to_asm()));
            }
            tacky::Instruction::Label(l) => {
                instructions.push(Instruction::Label(l.clone()));
            }
        }
    }

    let stack_size = replace_pseudo_registers(&mut instructions);
    let instructions = fixup_instructions(instructions, stack_size);

    Program {
        function_definition: Function {
            name: function.name.clone(),
            instructions,
        },
    }
}

fn replace_pseudo_registers(instructions: &mut Vec<Instruction>) -> u64 {
    let mut stack_size = 0;
    let mut stack_vars = HashMap::new();

    let mut update_operand = |operand: &mut Operand| {
        if let Operand::Pseudo(name) = operand {
            let offset = if let Some(saved) = stack_vars.get(name).copied() {
                saved
            } else {
                stack_size += 4;
                stack_vars.insert(name.clone(), stack_size);
                stack_size
            };
            *operand = Operand::Stack(offset);
        }
    };

    for instruction in instructions {
        match instruction {
            Instruction::Mov(src, dst)
            | Instruction::Binary(_, src, dst)
            | Instruction::Cmp(src, dst) => {
                update_operand(src);
                update_operand(dst);
            }
            Instruction::Unary(_, src)
            | Instruction::Idiv(src)
            | Instruction::Sal(src)
            | Instruction::Sar(src)
            | Instruction::SetCC(_, src) => update_operand(src),
            _ => continue,
        }
    }

    stack_size
}

fn fixup_instructions(instructions: Vec<Instruction>, stack_size: u64) -> Vec<Instruction> {
    let mut fixed = Vec::with_capacity(instructions.len() + 1);

    fixed.push(Instruction::AllocateStack(stack_size));
    for instruction in instructions.into_iter() {
        match instruction {
            Instruction::Mov(Operand::Stack(src), Operand::Stack(dst)) => {
                fixed.push(Instruction::Mov(
                    Operand::Stack(src),
                    Operand::Reg(Reg::R10),
                ));
                fixed.push(Instruction::Mov(
                    Operand::Reg(Reg::R10),
                    Operand::Stack(dst),
                ));
            }
            Instruction::Binary(op, Operand::Stack(left), Operand::Stack(right))
                if matches!(
                    op,
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::And | BinaryOp::Or | BinaryOp::Xor
                ) =>
            {
                fixed.push(Instruction::Mov(
                    Operand::Stack(left),
                    Operand::Reg(Reg::R10),
                ));
                fixed.push(Instruction::Binary(
                    op,
                    Operand::Reg(Reg::R10),
                    Operand::Stack(right),
                ));
            }
            Instruction::Cmp(Operand::Stack(left), Operand::Stack(right)) => {
                fixed.push(Instruction::Mov(
                    Operand::Stack(left),
                    Operand::Reg(Reg::R10),
                ));
                fixed.push(Instruction::Cmp(
                    Operand::Reg(Reg::R10),
                    Operand::Stack(right),
                ));
            }
            Instruction::Cmp(left, Operand::Imm(value)) => {
                fixed.push(Instruction::Mov(
                    Operand::Imm(value),
                    Operand::Reg(Reg::R11),
                ));
                fixed.push(Instruction::Cmp(left, Operand::Reg(Reg::R11)));
            }
            Instruction::Binary(BinaryOp::Mul, left, Operand::Stack(right)) => {
                fixed.push(Instruction::Mov(
                    Operand::Stack(right),
                    Operand::Reg(Reg::R11),
                ));
                fixed.push(Instruction::Binary(
                    BinaryOp::Mul,
                    left,
                    Operand::Reg(Reg::R11),
                ));
                fixed.push(Instruction::Mov(
                    Operand::Reg(Reg::R11),
                    Operand::Stack(right),
                ));
            }
            Instruction::Idiv(Operand::Imm(value)) => {
                fixed.push(Instruction::Mov(
                    Operand::Imm(value),
                    Operand::Reg(Reg::R10),
                ));
                fixed.push(Instruction::Idiv(Operand::Reg(Reg::R10)));
            }
            other => fixed.push(other),
        }
    }
    fixed
}

impl tacky::Val {
    fn to_asm(&self) -> Operand {
        match self {
            tacky::Val::Constant(value) => Operand::Imm(*value),
            tacky::Val::Var(name) => Operand::Pseudo(name.clone()),
        }
    }
}

impl tacky::UnaryOp {
    fn to_asm(&self) -> UnaryOp {
        match self {
            tacky::UnaryOp::Complement => UnaryOp::Not,
            tacky::UnaryOp::Negate => UnaryOp::Neg,
            tacky::UnaryOp::Increment => UnaryOp::Inc,
            tacky::UnaryOp::Decrement => UnaryOp::Dec,

            tacky::UnaryOp::Not => unreachable!(), // `Not` does not have equivalent
        }
    }
}

impl tacky::BinaryOp {
    fn to_asm(&self) -> BinaryOp {
        match self {
            tacky::BinaryOp::Add => BinaryOp::Add,
            tacky::BinaryOp::Subtract => BinaryOp::Sub,
            tacky::BinaryOp::Multiply => BinaryOp::Mul,
            tacky::BinaryOp::BinAnd => BinaryOp::And,
            tacky::BinaryOp::BinOr => BinaryOp::Or,
            tacky::BinaryOp::BinXor => BinaryOp::Xor,

            _ => unreachable!(), // Other operators do not have equivalent
        }
    }
}
