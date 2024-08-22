use std::io::{BufWriter, Result, Write};

use crate::asm::{Instruction, Operand, Program};

pub fn emit_code(program: &Program) -> Result<tempfile::TempPath> {
    let file = tempfile::Builder::new().suffix(".s").tempfile()?;

    let mut output = BufWriter::new(file);

    let function = &program.function_definition;

    writeln!(output, "\t.globl _{name}", name = function.name)?;
    writeln!(output)?;
    writeln!(output, "_{name}:", name = function.name)?;

    for ins in &function.instructions {
        write!(output, "\t")?;
        match ins {
            Instruction::Mov(src, dst) => {
                write!(output, "movl ")?;
                write_operand(&mut output, src)?;
                write!(output, ", ")?;
                write_operand(&mut output, dst)?;
            }
            Instruction::Ret => write!(output, "ret")?,
        }
        writeln!(output)?;
    }

    Ok(output.into_inner()?.into_temp_path())
}

fn write_operand(output: &mut impl Write, operand: &Operand) -> Result<()> {
    match operand {
        Operand::Imm(value) => write!(output, "${value}"),
        Operand::Reg => write!(output, "%eax"),
    }
}
