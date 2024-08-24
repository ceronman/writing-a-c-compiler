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
    AllocateStack(u64),
    Ret,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
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
    R10,
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
            tacky::Instruction::Unary { op, src, dst } => {
                instructions.push(Instruction::Mov(src.to_asm(), dst.to_asm()));
                instructions.push(Instruction::Unary(op.to_asm(), dst.to_asm()));
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
            Instruction::Mov(src, dst) => {
                update_operand(src);
                update_operand(dst);
            }
            Instruction::Unary(_, src) => update_operand(src),
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
        }
    }
}
