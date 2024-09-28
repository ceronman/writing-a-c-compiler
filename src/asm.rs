use crate::symbol::Symbol;
use crate::tacky;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
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
    AllocateStack(i64),
    DeallocateStack(i64),
    Push(Operand),
    Call(Symbol),
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
    Stack(i64),
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    Ax,
    Cx,
    Dx,
    Di,
    Si,
    R8,
    R9,
    R10,
    R11,
}

const ARG_REGISTERS: [Reg; 6] = [Reg::Di, Reg::Si, Reg::Dx, Reg::Cx, Reg::R8, Reg::R9];

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
    Program {
        functions: program.functions.iter().map(generate_function).collect(),
    }
}

fn generate_function(function: &tacky::Function) -> Function {
    let mut instructions = Vec::new();

    for (i, param) in function.params.iter().cloned().enumerate() {
        let src = if i < ARG_REGISTERS.len() {
            Operand::Reg(ARG_REGISTERS[i])
        } else {
            let offset = 16 + 8 * (i - ARG_REGISTERS.len());
            Operand::Stack(offset as i64)
        };
        instructions.push(Instruction::Mov(src, Operand::Pseudo(param)))
    }

    for tacky_instruction in &function.body {
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
            tacky::Instruction::FnCall { name, args, dst } => {
                generate_call(&mut instructions, name, args, dst);
            }
        }
    }

    let stack_size = replace_pseudo_registers(&mut instructions);
    let instructions = fixup_instructions(instructions, stack_size);

    Function {
        name: function.name.clone(),
        instructions,
    }
}

fn generate_call(
    instructions: &mut Vec<Instruction>,
    name: &Symbol,
    args: &[tacky::Val],
    dst: &tacky::Val,
) {
    let padding = if args.len() > 6 && args.len() % 2 != 0 {
        8
    } else {
        0
    };
    if padding != 0 {
        instructions.push(Instruction::AllocateStack(padding))
    }

    let register_args = args.iter().take(6);
    for (reg, val) in ARG_REGISTERS.iter().zip(register_args) {
        instructions.push(Instruction::Mov(val.to_asm(), Operand::Reg(*reg)));
    }

    let stack_args = args.iter().skip(6);
    for val in stack_args.clone().rev() {
        let operand = val.to_asm();
        if let Operand::Imm(_) | Operand::Reg(_) = operand {
            instructions.push(Instruction::Push(operand))
        } else {
            instructions.push(Instruction::Mov(operand, Operand::Reg(Reg::Ax)));
            instructions.push(Instruction::Push(Operand::Reg(Reg::Ax)));
        }
    }

    instructions.push(Instruction::Call(name.clone()));

    let bytes_to_remove = 8 * stack_args.count() as i64 + padding;
    if bytes_to_remove != 0 {
        instructions.push(Instruction::DeallocateStack(bytes_to_remove));
    }

    instructions.push(Instruction::Mov(Operand::Reg(Reg::Ax), dst.to_asm()));
}

fn replace_pseudo_registers(instructions: &mut Vec<Instruction>) -> i64 {
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
            *operand = Operand::Stack(-offset);
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
            | Instruction::Push(src)
            | Instruction::Idiv(src)
            | Instruction::Sal(src)
            | Instruction::Sar(src)
            | Instruction::SetCC(_, src) => update_operand(src),
            _ => continue,
        }
    }

    stack_size
}

fn fixup_instructions(instructions: Vec<Instruction>, stack_size: i64) -> Vec<Instruction> {
    let mut fixed = Vec::with_capacity(instructions.len() + 1);

    // Round stack_size to the closest next multiple of 16
    fixed.push(Instruction::AllocateStack(((stack_size / 16) + 1) * 16));

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
