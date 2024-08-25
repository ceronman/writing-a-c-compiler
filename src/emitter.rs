use crate::asm::{Instruction, Operand, Program, Reg, UnaryOp};
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
    writeln!(output, "_{name}:", name = function.name)?;

    // Prologue
    writeln!(output, "\tpushq %rbp")?;
    writeln!(output, "\tmovq %rsp, %rbp")?;

    for ins in &function.instructions {
        match ins {
            Instruction::Mov(src, dst) => {
                write!(output, "\tmovl ")?;
                emit_operand(&mut output, src)?;
                write!(output, ", ")?;
                emit_operand(&mut output, dst)?;
                writeln!(output)?;
            }
            Instruction::Unary(op, src) => {
                let op = match op {
                    UnaryOp::Neg => "negl",
                    UnaryOp::Not => "notl",
                };
                write!(output, "\t{op} ")?;
                emit_operand(&mut output, src)?;
                writeln!(output)?;
            }
            Instruction::AllocateStack(offset) => {
                writeln!(output, "\tsubq ${offset}, %rsp")?;
            }
            Instruction::Ret => {
                // epilogue
                writeln!(output, "\tmovq %rbp, %rsp")?;
                writeln!(output, "\tpopq %rbp")?;
                writeln!(output, "\tret")?;
            }
        }
    }

    Ok(output_path)
}

fn emit_operand(output: &mut impl Write, operand: &Operand) -> Result<()> {
    match operand {
        Operand::Imm(value) => write!(output, "${value}"),
        Operand::Reg(Reg::Ax) => write!(output, "%eax"),
        Operand::Reg(Reg::R10) => write!(output, "%r10d"),
        Operand::Stack(offset) => write!(output, "-{offset}(%rbp)"),
        Operand::Pseudo(_) => unreachable!(),
    }
}
