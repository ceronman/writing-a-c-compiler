use crate::asm;

use crate::asm::{Instruction, Operand};
use crate::ast::{Expression, Program, Statement};

pub fn generate(program: &Program) -> asm::Program {
    let mut instructions = Vec::new();
    let function = &program.function_definition;
    match &function.body {
        Statement::Return { expr } => match expr {
            Expression::Constant(value) => {
                instructions.push(Instruction::Mov(Operand::Imm(*value), Operand::Reg));
                instructions.push(Instruction::Ret);
            }
            _ => todo!(),
        },
    }
    asm::Program {
        function_definition: asm::Function {
            name: function.name.clone(),
            instructions,
        },
    }
}
