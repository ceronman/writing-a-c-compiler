use crate::asm::{BinaryOp, CondCode, Instruction, Operand, Program, Reg, UnaryOp};
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

    let output = &mut BufWriter::new(file);

    let function = &program.function_definition;

    writeln!(output, "\t.globl _{name}", name = function.name)?;
    writeln!(output, "_{name}:", name = function.name)?;

    // Prologue
    writeln!(output, "\tpushq    %rbp")?;
    writeln!(output, "\tmovq     %rsp, %rbp")?;

    for ins in &function.instructions {
        match ins {
            Instruction::Mov(src, dst) => {
                emit_ins(output, "movl")?;
                emit_operand(output, src, 4)?;
                write!(output, ", ")?;
                emit_operand(output, dst, 4)?;
            }
            Instruction::Unary(op, src) => {
                let op = match op {
                    UnaryOp::Neg => "negl",
                    UnaryOp::Not => "notl",
                    UnaryOp::Inc => "incl",
                    UnaryOp::Dec => "decl",
                };
                emit_ins(output, op)?;
                emit_operand(output, src, 4)?;
            }
            Instruction::Binary(op, left, right) => {
                let op = match op {
                    BinaryOp::Add => "addl",
                    BinaryOp::Sub => "subl",
                    BinaryOp::Mul => "imull",
                    BinaryOp::And => "andl",
                    BinaryOp::Or => "orl",
                    BinaryOp::Xor => "xorl",
                };
                emit_ins(output, op)?;
                emit_operand(output, left, 4)?;
                write!(output, ", ")?;
                emit_operand(output, right, 4)?;
            }
            Instruction::AllocateStack(offset) => {
                emit_ins(output, "subq")?;
                write!(output, "${offset}, %rsp")?;
            }

            Instruction::Sal(dst) => {
                emit_ins(output, "sall")?;
                emit_operand(output, &Operand::Reg(Reg::Cx), 1)?;
                write!(output, ", ")?;
                emit_operand(output, dst, 4)?;
                writeln!(output)?;
            }

            Instruction::Sar(dst) => {
                emit_ins(output, "sarl")?;
                emit_operand(output, &Operand::Reg(Reg::Cx), 1)?;
                write!(output, ", ")?;
                emit_operand(output, dst, 4)?;
            }

            Instruction::Idiv(src) => {
                emit_ins(output, "idivl")?;
                emit_operand(output, src, 4)?;
            }

            Instruction::Cdq => write!(output, "\tcdq")?,

            Instruction::Ret => {
                // epilogue
                writeln!(output, "\tmovq     %rbp, %rsp")?;
                writeln!(output, "\tpopq     %rbp")?;
                write!(output, "\tret")?;
            }

            Instruction::Cmp(left, right) => {
                emit_ins(output, "cmpl")?;
                emit_operand(output, left, 4)?;
                write!(output, ", ")?;
                emit_operand(output, right, 4)?;
            }
            Instruction::Jmp(label) => {
                emit_ins(output, "jmp")?;
                emit_label(output, label)?;
            }
            Instruction::JmpCC(cond, target) => {
                match cond {
                    CondCode::E => emit_ins(output, "je")?,
                    CondCode::NE => emit_ins(output, "jne")?,
                    CondCode::G => emit_ins(output, "jg")?,
                    CondCode::GE => emit_ins(output, "jge")?,
                    CondCode::L => emit_ins(output, "jl")?,
                    CondCode::LE => emit_ins(output, "jle")?,
                }
                emit_label(output, target)?;
            }
            Instruction::SetCC(cond, dst) => {
                match cond {
                    CondCode::E => emit_ins(output, "sete")?,
                    CondCode::NE => emit_ins(output, "setne")?,
                    CondCode::G => emit_ins(output, "setg")?,
                    CondCode::GE => emit_ins(output, "setge")?,
                    CondCode::L => emit_ins(output, "setl")?,
                    CondCode::LE => emit_ins(output, "setle")?,
                }
                emit_operand(output, dst, 1)?;
            }
            Instruction::Label(label) => {
                emit_label(output, label)?;
                write!(output, ":")?;
            }
        }
        writeln!(output)?;
    }

    Ok(output_path)
}

fn emit_ins(output: &mut impl Write, ins: &str) -> Result<()> {
    write!(output, "\t{ins:8} ")
}

fn emit_operand(output: &mut impl Write, operand: &Operand, size: u8) -> Result<()> {
    match (operand, size) {
        (Operand::Imm(value), _) => write!(output, "${value}"),
        (Operand::Reg(Reg::Ax), 1) => write!(output, "%al"),
        (Operand::Reg(Reg::Ax), 4) => write!(output, "%eax"),
        (Operand::Reg(Reg::Cx), 1) => write!(output, "%cl"),
        (Operand::Reg(Reg::Cx), 4) => write!(output, "%ecx"),
        (Operand::Reg(Reg::Dx), 1) => write!(output, "%dl"),
        (Operand::Reg(Reg::Dx), 4) => write!(output, "%edx"),
        (Operand::Reg(Reg::R10), 1) => write!(output, "%r10b"),
        (Operand::Reg(Reg::R10), 4) => write!(output, "%r10d"),
        (Operand::Reg(Reg::R11), 1) => write!(output, "%r11b"),
        (Operand::Reg(Reg::R11), 4) => write!(output, "%r11d"),
        (Operand::Stack(offset), _) => write!(output, "-{offset}(%rbp)"),
        _ => unreachable!(),
    }
}

fn emit_label(output: &mut impl Write, label: &str) -> Result<()> {
    write!(output, "L{label}")
}
