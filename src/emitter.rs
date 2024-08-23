use crate::asm::{Instruction, Operand, Program};
use crate::tempfile::TempPath;
use std::fs::OpenOptions;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

pub fn emit_code(filename: &Path, program: &Program) -> Result<TempPath> {
    let output_path = TempPath::new(filename.with_extension("s"));
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path.as_path())?;

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
    Ok(output_path)
}

fn write_operand(output: &mut impl Write, operand: &Operand) -> Result<()> {
    match operand {
        Operand::Imm(value) => write!(output, "${value}"),
        Operand::Reg => write!(output, "%eax"),
    }
}
