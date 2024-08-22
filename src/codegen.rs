use crate::ir;

use crate::ast::{Expression, Program, Statement};
use crate::ir::{Instruction, Operand};
use anyhow::Result;

pub fn generate_ir(program: Program) -> Result<ir::Program> {
    let mut instructions = Vec::new();
    let function = program.function_definition;
    match &function.body {
        Statement::Return { expr } => match expr {
            Expression::Constant(value) => {
                instructions.push(Instruction::Mov(Operand::Imm(*value), Operand::Reg));
                instructions.push(Instruction::Ret);
            }
        },
    }
    Ok(ir::Program {
        function_definition: ir::Function {
            name: function.name,
            instructions,
        },
    })
}
