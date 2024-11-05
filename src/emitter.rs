use crate::asm::ir::{
    AsmType, BinaryOp, CondCode, Function, Instruction, Operand, Program, Reg, StaticConstant,
    StaticVariable, TopLevel, UnaryOp,
};
use crate::semantic::StaticInit;
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
    for (i, top_level) in program.top_level.iter().enumerate() {
        match top_level {
            TopLevel::Function(function) => emit_function(output, function)?,
            TopLevel::Variable(variable) => emit_variable(output, variable)?,
            TopLevel::Constant(constant) => emit_constant(output, constant)?,
        }
        if i < program.top_level.len() - 1 {
            writeln!(output)?;
        }
    }
    Ok(output_path)
}

// TODO: rewrite with an IR for simplicity
fn emit_function(output: &mut impl Write, function: &Function) -> Result<()> {
    if function.global {
        writeln!(output, "\t.globl _{name}", name = function.name)?;
    }
    writeln!(output, "\t.text")?;
    writeln!(output, "_{name}:", name = function.name)?;

    // Prologue
    emit_ins(output, "pushq")?;
    writeln!(output, "%rbp")?;
    emit_ins(output, "movq")?;
    writeln!(output, "%rsp, %rbp")?;

    for ins in &function.instructions {
        match ins {
            Instruction::Mov(ty, src, dst) => {
                let op = match ty {
                    AsmType::Longword => "movl",
                    AsmType::Quadword => "movq",
                    AsmType::Double => "movsd",
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty))?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::from_ty(ty))?;
            }
            Instruction::Unary(ty, op, src) => {
                let op = match (op, ty) {
                    (UnaryOp::Neg, AsmType::Longword) => "negl",
                    (UnaryOp::Neg, AsmType::Quadword) => "negq",
                    (UnaryOp::Neg, AsmType::Double) => "negsd",

                    (UnaryOp::Not, AsmType::Longword) => "notl",
                    (UnaryOp::Not, AsmType::Quadword) => "notq",
                    (UnaryOp::Not, AsmType::Double) => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty))?;
            }
            Instruction::Binary(ty, op, left, right) => {
                let op = match (op, ty) {
                    (BinaryOp::Add, AsmType::Longword) => "addl",
                    (BinaryOp::Add, AsmType::Quadword) => "addq",
                    (BinaryOp::Add, AsmType::Double) => "addsd",

                    (BinaryOp::Sub, AsmType::Longword) => "subl",
                    (BinaryOp::Sub, AsmType::Quadword) => "subq",
                    (BinaryOp::Sub, AsmType::Double) => "subsd",

                    (BinaryOp::Mul, AsmType::Longword) => "imull",
                    (BinaryOp::Mul, AsmType::Quadword) => "imulq",
                    (BinaryOp::Mul, AsmType::Double) => "mulsd",

                    (BinaryOp::And, AsmType::Longword) => "andl",
                    (BinaryOp::And, AsmType::Quadword) => "andq",
                    (BinaryOp::And, AsmType::Double) => unreachable!(),

                    (BinaryOp::Or, AsmType::Longword) => "orl",
                    (BinaryOp::Or, AsmType::Quadword) => "orq",
                    (BinaryOp::Or, AsmType::Double) => unreachable!(),

                    (BinaryOp::Xor, AsmType::Longword) => "xorl",
                    (BinaryOp::Xor, AsmType::Quadword) => "xorq",
                    (BinaryOp::Xor, AsmType::Double) => "xorpd",

                    (BinaryOp::DivDouble, AsmType::Double) => "divsd",
                    (BinaryOp::DivDouble, _) => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, left, RegSize::from_ty(ty))?;
                write!(output, ", ")?;
                emit_operand(output, right, RegSize::from_ty(ty))?;
            }

            Instruction::Sal(ty, dst) => {
                let op = match ty {
                    AsmType::Longword => "sall",
                    AsmType::Quadword => "salq",
                    AsmType::Double => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, &Operand::Reg(Reg::Cx), RegSize::Byte)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::from_ty(ty))?;
                writeln!(output)?;
            }

            Instruction::Shl(ty, dst) => {
                let op = match ty {
                    AsmType::Longword => "shll",
                    AsmType::Quadword => "shlq",
                    AsmType::Double => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, &Operand::Reg(Reg::Cx), RegSize::Byte)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::from_ty(ty))?;
                writeln!(output)?;
            }

            Instruction::Sar(ty, dst) => {
                let op = match ty {
                    AsmType::Longword => "sarl",
                    AsmType::Quadword => "sarq",
                    AsmType::Double => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, &Operand::Reg(Reg::Cx), RegSize::Byte)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::from_ty(ty))?;
            }

            Instruction::Shr(ty, dst) => {
                let op = match ty {
                    AsmType::Longword => "shrl",
                    AsmType::Quadword => "shrq",
                    AsmType::Double => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, &Operand::Reg(Reg::Cx), RegSize::Byte)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::from_ty(ty))?;
            }

            Instruction::Idiv(ty, src) => {
                let op = match ty {
                    AsmType::Longword => "idivl",
                    AsmType::Quadword => "idivq",
                    AsmType::Double => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty))?;
            }

            Instruction::Div(ty, src) => {
                let op = match ty {
                    AsmType::Longword => "divl",
                    AsmType::Quadword => "divq",
                    AsmType::Double => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty))?;
            }

            Instruction::Cdq(ty) => {
                let op = match ty {
                    AsmType::Longword => "cdq",
                    AsmType::Quadword => "cqo",
                    AsmType::Double => unreachable!(),
                };
                emit_ins(output, op)?;
            }

            Instruction::Ret => {
                // epilogue
                emit_ins(output, "movq")?;
                writeln!(output, "%rbp, %rsp")?;
                emit_ins(output, "popq")?;
                writeln!(output, "%rbp")?;
                emit_ins(output, "ret")?;
            }

            Instruction::Cmp(ty, left, right) => {
                let op = match ty {
                    AsmType::Longword => "cmpl",
                    AsmType::Quadword => "cmpq",
                    AsmType::Double => "comisd",
                };
                emit_ins(output, op)?;
                emit_operand(output, left, RegSize::from_ty(ty))?;
                write!(output, ", ")?;
                emit_operand(output, right, RegSize::from_ty(ty))?;
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
                    CondCode::A => emit_ins(output, "ja")?,
                    CondCode::AE => emit_ins(output, "jae")?,
                    CondCode::B => emit_ins(output, "jb")?,
                    CondCode::BE => emit_ins(output, "jbe")?,
                    CondCode::P => emit_ins(output, "jp")?,
                    CondCode::NP => emit_ins(output, "jnp")?,
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
                    CondCode::A => emit_ins(output, "seta")?,
                    CondCode::AE => emit_ins(output, "setae")?,
                    CondCode::B => emit_ins(output, "setb")?,
                    CondCode::BE => emit_ins(output, "setbe")?,
                    CondCode::P => emit_ins(output, "setp")?,
                    CondCode::NP => emit_ins(output, "setnp")?,
                }
                emit_operand(output, dst, RegSize::Byte)?;
            }
            Instruction::Label(label) => {
                emit_label(output, label)?;
                write!(output, ":")?;
            }
            Instruction::Push(operand) => {
                emit_ins(output, "pushq")?;
                emit_operand(output, operand, RegSize::Quad)?;
            }
            Instruction::Call(name) => {
                emit_ins(output, "call")?;
                write!(output, "_{name}")?;
            }
            Instruction::Movsx(src, dst) => {
                emit_ins(output, "movslq")?;
                emit_operand(output, src, RegSize::Long)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::Quad)?;
            }
            Instruction::MovZeroExtend(_, _) => {
                unreachable!("Should have been replaced in fixup instructions")
            }

            Instruction::Cvttsd2si(ty, src, dst) => {
                let op = match ty {
                    AsmType::Longword => "cvttsd2sil",
                    AsmType::Quadword => "cvttsd2siq",
                    AsmType::Double => panic!("Should never be called with double"),
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::Quad)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::from_ty(ty))?;
            }
            Instruction::Cvtsi2sd(ty, src, dst) => {
                let op = match ty {
                    AsmType::Longword => "cvtsi2sdl",
                    AsmType::Quadword => "cvtsi2sdq",
                    AsmType::Double => unreachable!("Should never be called with double"),
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty))?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::Quad)?;
            }
        }
        writeln!(output)?;
    }
    Ok(())
}

fn emit_variable(output: &mut impl Write, variable: &StaticVariable) -> Result<()> {
    if variable.global {
        writeln!(output, "\t.globl _{name}", name = variable.name)?;
    }
    if matches!(
        variable.init,
        StaticInit::Int(0) | StaticInit::Long(0) | StaticInit::UInt(0) | StaticInit::ULong(0)
    ) {
        writeln!(output, "\t.bss")?;
        emit_ins(output, ".balign")?;
        writeln!(output, "{}", variable.alignment)?;
        writeln!(output, "_{name}:", name = variable.name)?;
        emit_ins(output, ".zero")?;
        writeln!(output, "{}", variable.alignment)?;
    } else {
        writeln!(output, "\t.data")?;
        emit_ins(output, ".balign")?;
        writeln!(output, "{}", variable.alignment)?;
        writeln!(output, "_{name}:", name = variable.name)?;
        emit_static_init(output, &variable.init)?;
    }
    Ok(())
}

fn emit_static_init(output: &mut impl Write, init: &StaticInit) -> Result<()> {
    match init {
        StaticInit::Int(v) => {
            emit_ins(output, ".long")?;
            writeln!(output, "{}", v)?;
        }
        StaticInit::UInt(v) => {
            emit_ins(output, ".long")?;
            writeln!(output, "{}", v)?;
        }
        StaticInit::Long(v) => {
            emit_ins(output, ".quad")?;
            writeln!(output, "{}", v)?;
        }
        StaticInit::ULong(v) => {
            emit_ins(output, ".quad")?;
            writeln!(output, "{}", v)?;
        }
        StaticInit::Double(v) => {
            emit_ins(output, ".quad")?;
            writeln!(output, "{:#x} # {v}_f64", v.to_bits())?;
        }
    };
    Ok(())
}

fn emit_constant(output: &mut impl Write, constant: &StaticConstant) -> Result<()> {
    match constant.alignment {
        8 => {
            writeln!(output, "\t.literal8")?;
            writeln!(output, "\t.balign 8")?;
            writeln!(output, "L{name}:", name = constant.name)?;
            emit_static_init(output, &constant.init)?;
        }
        16 => {
            writeln!(output, "\t.literal16")?;
            writeln!(output, "\t.balign 16")?;
            writeln!(output, "L{name}:", name = constant.name)?;
            emit_static_init(output, &constant.init)?;
            emit_ins(output, ".quad")?;
            writeln!(output, "0")?;
        }
        _ => panic!("Invalid alignment"),
    };
    Ok(())
}

fn emit_ins(output: &mut impl Write, ins: &str) -> Result<()> {
    write!(output, "\t{ins:8} ")
}

enum RegSize {
    Byte,
    Long,
    Quad,
}

impl RegSize {
    fn from_ty(ty: &AsmType) -> RegSize {
        match ty {
            AsmType::Longword => RegSize::Long,
            AsmType::Quadword => RegSize::Quad,
            AsmType::Double => RegSize::Quad,
        }
    }
}

fn emit_operand(output: &mut impl Write, operand: &Operand, size: RegSize) -> Result<()> {
    match (operand, size) {
        (Operand::Reg(Reg::Ax), RegSize::Byte) => write!(output, "%al"),
        (Operand::Reg(Reg::Ax), RegSize::Long) => write!(output, "%eax"),
        (Operand::Reg(Reg::Ax), RegSize::Quad) => write!(output, "%rax"),

        (Operand::Reg(Reg::Cx), RegSize::Byte) => write!(output, "%cl"),
        (Operand::Reg(Reg::Cx), RegSize::Long) => write!(output, "%ecx"),
        (Operand::Reg(Reg::Cx), RegSize::Quad) => write!(output, "%rcx"),

        (Operand::Reg(Reg::Dx), RegSize::Byte) => write!(output, "%dl"),
        (Operand::Reg(Reg::Dx), RegSize::Long) => write!(output, "%edx"),
        (Operand::Reg(Reg::Dx), RegSize::Quad) => write!(output, "%rdx"),

        (Operand::Reg(Reg::Di), RegSize::Byte) => write!(output, "%dil"),
        (Operand::Reg(Reg::Di), RegSize::Long) => write!(output, "%edi"),
        (Operand::Reg(Reg::Di), RegSize::Quad) => write!(output, "%rdi"),

        (Operand::Reg(Reg::Si), RegSize::Byte) => write!(output, "%sil"),
        (Operand::Reg(Reg::Si), RegSize::Long) => write!(output, "%esi"),
        (Operand::Reg(Reg::Si), RegSize::Quad) => write!(output, "%rsi"),

        (Operand::Reg(Reg::R8), RegSize::Byte) => write!(output, "%r8b"),
        (Operand::Reg(Reg::R8), RegSize::Long) => write!(output, "%r8d"),
        (Operand::Reg(Reg::R8), RegSize::Quad) => write!(output, "%r8"),

        (Operand::Reg(Reg::R9), RegSize::Byte) => write!(output, "%r9b"),
        (Operand::Reg(Reg::R9), RegSize::Long) => write!(output, "%r9d"),
        (Operand::Reg(Reg::R9), RegSize::Quad) => write!(output, "%r9"),

        (Operand::Reg(Reg::R10), RegSize::Byte) => write!(output, "%r10b"),
        (Operand::Reg(Reg::R10), RegSize::Long) => write!(output, "%r10d"),
        (Operand::Reg(Reg::R10), RegSize::Quad) => write!(output, "%r10"),

        (Operand::Reg(Reg::R11), RegSize::Byte) => write!(output, "%r11b"),
        (Operand::Reg(Reg::R11), RegSize::Long) => write!(output, "%r11d"),
        (Operand::Reg(Reg::R11), RegSize::Quad) => write!(output, "%r11"),

        (Operand::Reg(Reg::SP), _) => write!(output, "%rsp"),

        (Operand::Reg(Reg::XMM0), _) => write!(output, "%xmm0"),
        (Operand::Reg(Reg::XMM1), _) => write!(output, "%xmm1"),
        (Operand::Reg(Reg::XMM2), _) => write!(output, "%xmm2"),
        (Operand::Reg(Reg::XMM3), _) => write!(output, "%xmm3"),
        (Operand::Reg(Reg::XMM4), _) => write!(output, "%xmm4"),
        (Operand::Reg(Reg::XMM5), _) => write!(output, "%xmm5"),
        (Operand::Reg(Reg::XMM6), _) => write!(output, "%xmm6"),
        (Operand::Reg(Reg::XMM7), _) => write!(output, "%xmm7"),
        (Operand::Reg(Reg::XMM14), _) => write!(output, "%xmm14"),
        (Operand::Reg(Reg::XMM15), _) => write!(output, "%xmm15"),

        (Operand::Imm(value), _) => write!(output, "${value}"),
        (Operand::Stack(offset), _) => write!(output, "{offset}(%rbp)"),
        (Operand::Data(true, name), _) => write!(output, "L{name}(%rip)"),
        (Operand::Data(_, name), _) => write!(output, "_{name}(%rip)"),
        (Operand::Pseudo(_), _) => unreachable!("Pseudo-registers should not appear here"),
    }
}

fn emit_label(output: &mut impl Write, label: &str) -> Result<()> {
    write!(output, "L{label}")
}
