use crate::asm::ir::{
    AsmType, BinaryOp, CondCode, Function, Instruction, Operand, Program, Reg, StaticConstant,
    StaticVariable, TopLevel, UnaryOp,
};
use crate::semantic::StaticInit;
use crate::symbol::Symbol;
use std::io::{Result, Write};

#[derive(Copy, Clone)]
pub enum TargetOs {
    MacOs,
    Linux,
}

pub fn emit_program(output: &mut impl Write, program: &Program, target_os: TargetOs) -> Result<()> {
    for (i, top_level) in program.top_level.iter().enumerate() {
        match top_level {
            TopLevel::Function(function) => emit_function(output, function, target_os)?,
            TopLevel::Variable(variable) => emit_variable(output, variable, target_os)?,
            TopLevel::Constant(constant) => emit_constant(output, constant, target_os)?,
        }
        if i < program.top_level.len() - 1 {
            writeln!(output)?;
        }
    }
    if let TargetOs::Linux = target_os {
        writeln!(output, ".section .note.GNU-stack,\"\",@progbits")?;
    }
    Ok(())
}

fn emit_function(output: &mut impl Write, function: &Function, target_os: TargetOs) -> Result<()> {
    if function.global {
        writeln!(
            output,
            "\t.globl {}",
            emit_symbol(&function.name, target_os)
        )?;
    }
    writeln!(output, "\t.text")?;
    writeln!(output, "{}:", emit_symbol(&function.name, target_os))?;

    // Prologue
    emit_ins(output, "pushq")?;
    writeln!(output, "%rbp")?;
    emit_ins(output, "movq")?;
    writeln!(output, "%rsp, %rbp")?;

    for ins in &function.instructions {
        match ins {
            Instruction::Mov(ty, src, dst) => {
                let op = match ty {
                    AsmType::Byte => "movb",
                    AsmType::Longword => "movl",
                    AsmType::Quadword | AsmType::ByteArray { .. } => "movq",
                    AsmType::Double => "movsd",
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty), target_os)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::from_ty(ty), target_os)?;
            }
            Instruction::Unary(ty, op, src) => {
                let op = match (op, ty) {
                    (UnaryOp::Neg, AsmType::Byte) => "negb",
                    (UnaryOp::Neg, AsmType::Longword) => "negl",
                    (UnaryOp::Neg, AsmType::Quadword) => "negq",
                    (UnaryOp::Neg, AsmType::Double) => "negsd",

                    (UnaryOp::Not, AsmType::Byte) => "notb",
                    (UnaryOp::Not, AsmType::Longword) => "notl",
                    (UnaryOp::Not, AsmType::Quadword) => "notq",
                    (UnaryOp::Not, AsmType::Double) => unreachable!(),
                    (_, AsmType::ByteArray { .. }) => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty), target_os)?;
            }
            Instruction::Binary(ty, op, left, right) => {
                let typed_instruction = match (op, ty) {
                    (BinaryOp::Add, AsmType::Byte) => "addb",
                    (BinaryOp::Add, AsmType::Longword) => "addl",
                    (BinaryOp::Add, AsmType::Quadword) => "addq",
                    (BinaryOp::Add, AsmType::Double) => "addsd",

                    (BinaryOp::Sub, AsmType::Byte) => "subb",
                    (BinaryOp::Sub, AsmType::Longword) => "subl",
                    (BinaryOp::Sub, AsmType::Quadword) => "subq",
                    (BinaryOp::Sub, AsmType::Double) => "subsd",

                    (BinaryOp::Mul, AsmType::Byte) => "imulb",
                    (BinaryOp::Mul, AsmType::Longword) => "imull",
                    (BinaryOp::Mul, AsmType::Quadword) => "imulq",
                    (BinaryOp::Mul, AsmType::Double) => "mulsd",

                    (BinaryOp::And, AsmType::Byte) => "andb",
                    (BinaryOp::And, AsmType::Longword) => "andl",
                    (BinaryOp::And, AsmType::Quadword) => "andq",
                    (BinaryOp::And, AsmType::Double) => unreachable!(),

                    (BinaryOp::Or, AsmType::Byte) => "orb",
                    (BinaryOp::Or, AsmType::Longword) => "orl",
                    (BinaryOp::Or, AsmType::Quadword) => "orq",
                    (BinaryOp::Or, AsmType::Double) => unreachable!(),

                    (BinaryOp::Xor, AsmType::Byte) => "xorb",
                    (BinaryOp::Xor, AsmType::Longword) => "xorl",
                    (BinaryOp::Xor, AsmType::Quadword) => "xorq",
                    (BinaryOp::Xor, AsmType::Double) => "xorpd",

                    (BinaryOp::Sal, AsmType::Byte) => "salb",
                    (BinaryOp::Sal, AsmType::Longword) => "sall",
                    (BinaryOp::Sal, AsmType::Quadword) => "salq",
                    (BinaryOp::Sal, AsmType::Double) => unreachable!(),

                    (BinaryOp::Shl, AsmType::Byte) => "shlb",
                    (BinaryOp::Shl, AsmType::Longword) => "shll",
                    (BinaryOp::Shl, AsmType::Quadword) => "shlq",
                    (BinaryOp::Shl, AsmType::Double) => unreachable!(),

                    (BinaryOp::Sar, AsmType::Byte) => "sarb",
                    (BinaryOp::Sar, AsmType::Longword) => "sarl",
                    (BinaryOp::Sar, AsmType::Quadword) => "sarq",
                    (BinaryOp::Sar, AsmType::Double) => unreachable!(),

                    (BinaryOp::Shr, AsmType::Byte) => "shrb",
                    (BinaryOp::Shr, AsmType::Longword) => "shrl",
                    (BinaryOp::Shr, AsmType::Quadword) => "shrq",
                    (BinaryOp::Shr, AsmType::Double) => unreachable!(),

                    (BinaryOp::DivDouble, AsmType::Double) => "divsd",
                    (BinaryOp::DivDouble, _) => unreachable!(),
                    (_, AsmType::ByteArray { .. }) => unreachable!(),
                };
                emit_ins(output, typed_instruction)?;
                let left_size = match op {
                    BinaryOp::Sar | BinaryOp::Shr | BinaryOp::Sal | BinaryOp::Shl => RegSize::Byte,
                    _ => RegSize::from_ty(ty),
                };
                emit_operand(output, left, left_size, target_os)?;
                write!(output, ", ")?;
                emit_operand(output, right, RegSize::from_ty(ty), target_os)?;
            }

            Instruction::Idiv(ty, src) => {
                let op = match ty {
                    AsmType::Byte => "idivb",
                    AsmType::Longword => "idivl",
                    AsmType::Quadword => "idivq",
                    AsmType::Double | AsmType::ByteArray { .. } => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty), target_os)?;
            }

            Instruction::Div(ty, src) => {
                let op = match ty {
                    AsmType::Byte => "divb",
                    AsmType::Longword => "divl",
                    AsmType::Quadword => "divq",
                    AsmType::Double | AsmType::ByteArray { .. } => unreachable!(),
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty), target_os)?;
            }

            Instruction::Cdq(ty) => {
                let op = match ty {
                    AsmType::Byte => "cdqb",
                    AsmType::Longword => "cdq",
                    AsmType::Quadword => "cqo",
                    AsmType::Double | AsmType::ByteArray { .. } => unreachable!(),
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
                    AsmType::Byte => "cmpb",
                    AsmType::Longword => "cmpl",
                    AsmType::Quadword | AsmType::ByteArray { .. } => "cmpq",
                    AsmType::Double => "comisd",
                };
                emit_ins(output, op)?;
                emit_operand(output, left, RegSize::from_ty(ty), target_os)?;
                write!(output, ", ")?;
                emit_operand(output, right, RegSize::from_ty(ty), target_os)?;
            }
            Instruction::Jmp(label) => {
                emit_ins(output, "jmp")?;
                emit_label(output, label, target_os)?;
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
                emit_label(output, target, target_os)?;
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
                emit_operand(output, dst, RegSize::Byte, target_os)?;
            }
            Instruction::Label(label) => {
                emit_label(output, label, target_os)?;
                write!(output, ":")?;
            }
            Instruction::Push(operand) => {
                emit_ins(output, "pushq")?;
                emit_operand(output, operand, RegSize::Quad, target_os)?;
            }
            Instruction::Pop(reg) => {
                emit_ins(output, "popq")?;
                emit_operand(output, &Operand::Reg(*reg), RegSize::Quad, target_os)?;
            }
            Instruction::Call(name) => {
                emit_ins(output, "call")?;
                write!(output, "{}", emit_symbol(name, target_os))?;
            }
            Instruction::Movsx(src_ty, src, dst_ty, dst) => {
                let s1 = RegSize::from_ty(src_ty);
                let s2 = RegSize::from_ty(dst_ty);
                emit_2sized(output, "movs", s1, s2)?;
                emit_operand(output, src, s1, target_os)?;
                write!(output, ", ")?;
                emit_operand(output, dst, s2, target_os)?;
            }
            Instruction::MovZeroExtend(src_ty, src, dst_ty, dst) => {
                let s1 = RegSize::from_ty(src_ty);
                let s2 = RegSize::from_ty(dst_ty);
                emit_2sized(output, "movz", s1, s2)?;
                emit_operand(output, src, s1, target_os)?;
                write!(output, ", ")?;
                emit_operand(output, dst, s2, target_os)?;
            }
            Instruction::Lea(src, dst) => {
                emit_ins(output, "leaq")?;
                emit_operand(output, src, RegSize::Quad, target_os)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::Quad, target_os)?;
            }
            Instruction::Cvttsd2si(ty, src, dst) => {
                let op = match ty {
                    AsmType::Byte => "cvttsd2sib",
                    AsmType::Longword => "cvttsd2sil",
                    AsmType::Quadword => "cvttsd2siq",
                    AsmType::Double | AsmType::ByteArray { .. } => {
                        panic!("Should never be called with double or bytearray")
                    }
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::Quad, target_os)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::from_ty(ty), target_os)?;
            }
            Instruction::Cvtsi2sd(ty, src, dst) => {
                let op = match ty {
                    AsmType::Byte => unreachable!("Can't convert char to double"),
                    AsmType::Longword => "cvtsi2sdl",
                    AsmType::Quadword => "cvtsi2sdq",
                    AsmType::Double | AsmType::ByteArray { .. } => {
                        unreachable!("Should never be called with double or bytearray")
                    }
                };
                emit_ins(output, op)?;
                emit_operand(output, src, RegSize::from_ty(ty), target_os)?;
                write!(output, ", ")?;
                emit_operand(output, dst, RegSize::Quad, target_os)?;
            }
        }
        writeln!(output)?;
    }
    Ok(())
}

fn emit_variable(
    output: &mut impl Write,
    variable: &StaticVariable,
    target_os: TargetOs,
) -> Result<()> {
    if variable.global {
        writeln!(
            output,
            "\t.globl {}",
            emit_symbol(&variable.name, target_os)
        )?;
    }

    if matches!(
        variable.init[..],
        [StaticInit::Int(0)]
            | [StaticInit::Long(0)]
            | [StaticInit::UInt(0)]
            | [StaticInit::ULong(0)]
            | [StaticInit::ZeroInit(0)]
    ) {
        writeln!(output, "\t.bss")?;
        emit_ins(output, ".balign")?;
        writeln!(output, "{}", variable.alignment)?;
        writeln!(output, "{}:", emit_symbol(&variable.name, target_os))?;
        emit_ins(output, ".zero")?;
        writeln!(output, "{}", variable.alignment)?;
    } else {
        writeln!(output, "\t.data")?;
        emit_ins(output, ".balign")?;
        writeln!(output, "{}", variable.alignment)?;
        writeln!(output, "{}:", emit_symbol(&variable.name, target_os))?;
        for init in &variable.init {
            emit_static_init(output, init, target_os)?;
        }
    }
    Ok(())
}

fn emit_static_init(output: &mut impl Write, init: &StaticInit, target_os: TargetOs) -> Result<()> {
    match init {
        StaticInit::String {
            symbol,
            null_terminated,
        } => {
            if *null_terminated {
                emit_ins(output, ".asciz")?;
            } else {
                emit_ins(output, ".ascii")?;
            }
            write!(output, "\"")?;
            for c in symbol.as_ref().as_bytes() {
                // Hack to avoid std::ascii::escape_default escaping the quote
                if *c as char != '\'' {
                    write!(output, "{}", std::ascii::escape_default(*c))?;
                } else {
                    write!(output, "\'")?;
                }
            }
            writeln!(output, "\"")?;
        }
        StaticInit::Pointer(label) => {
            emit_ins(output, ".quad")?;
            emit_label(output, label, target_os)?;
            writeln!(output)?;
        }
        StaticInit::Char(v) => {
            emit_ins(output, ".byte")?;
            writeln!(output, "{v}")?;
        }
        StaticInit::UChar(v) => {
            emit_ins(output, ".byte")?;
            writeln!(output, "{v}")?;
        }
        StaticInit::Int(v) => {
            emit_ins(output, ".long")?;
            writeln!(output, "{v}")?;
        }
        StaticInit::UInt(v) => {
            emit_ins(output, ".long")?;
            writeln!(output, "{v}")?;
        }
        StaticInit::Long(v) => {
            emit_ins(output, ".quad")?;
            writeln!(output, "{v}")?;
        }
        StaticInit::ULong(v) => {
            emit_ins(output, ".quad")?;
            writeln!(output, "{v}")?;
        }
        StaticInit::Double(v) => {
            emit_ins(output, ".quad")?;
            writeln!(output, "{:#x} # {v}_f64", v.to_bits())?;
        }
        StaticInit::ZeroInit(size) => {
            emit_ins(output, ".zero")?;
            writeln!(output, "{size}")?;
        }
    };
    Ok(())
}

fn emit_constant(
    output: &mut impl Write,
    constant: &StaticConstant,
    target_os: TargetOs,
) -> Result<()> {
    if let StaticInit::String { .. } = &constant.init {
        match target_os {
            TargetOs::MacOs => writeln!(output, "\t.cstring")?,
            TargetOs::Linux => writeln!(output, "\t.section .rodata")?,
        }
        emit_label(output, &constant.name, target_os)?;
        writeln!(output, ":")?;
        emit_static_init(output, &constant.init, target_os)?;
        return Ok(());
    }
    match constant.alignment {
        8 => {
            match target_os {
                TargetOs::MacOs => {
                    writeln!(output, "\t.literal8")?;
                    writeln!(output, "\t.balign 8")?;
                }
                TargetOs::Linux => {
                    writeln!(output, "\t.section .rodata")?;
                    writeln!(output, "\t.balign 8")?;
                }
            }
            emit_label(output, &constant.name, target_os)?;
            writeln!(output, ":")?;
            emit_static_init(output, &constant.init, target_os)?;
        }
        16 => {
            match target_os {
                TargetOs::MacOs => {
                    writeln!(output, "\t.literal16")?;
                    writeln!(output, "\t.balign 16")?;
                }
                TargetOs::Linux => {
                    writeln!(output, "\t.section .rodata")?;
                    writeln!(output, "\t.balign 16")?;
                }
            }
            emit_label(output, &constant.name, target_os)?;
            writeln!(output, ":")?;
            emit_static_init(output, &constant.init, target_os)?;
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

fn emit_2sized(output: &mut impl Write, ins: &str, s1: RegSize, s2: RegSize) -> Result<()> {
    let ins = format!("{ins}{}{}", s1.suffix(), s2.suffix());
    write!(output, "\t{ins:8} ")
}

#[derive(Copy, Clone)]
enum RegSize {
    Byte,
    Long,
    Quad,
}

impl RegSize {
    fn from_ty(ty: &AsmType) -> RegSize {
        match ty {
            AsmType::Byte => RegSize::Byte,
            AsmType::Longword => RegSize::Long,
            AsmType::Quadword => RegSize::Quad,
            AsmType::Double => RegSize::Quad,
            AsmType::ByteArray { .. } => RegSize::Quad,
        }
    }

    fn suffix(self) -> char {
        match self {
            RegSize::Byte => 'b',
            RegSize::Long => 'l',
            RegSize::Quad => 'q',
        }
    }
}

fn emit_operand(
    output: &mut impl Write,
    operand: &Operand,
    size: RegSize,
    target_os: TargetOs,
) -> Result<()> {
    match (operand, size) {
        (Operand::Reg(Reg::Ax), RegSize::Byte) => write!(output, "%al"),
        (Operand::Reg(Reg::Ax), RegSize::Long) => write!(output, "%eax"),
        (Operand::Reg(Reg::Ax), RegSize::Quad) => write!(output, "%rax"),

        (Operand::Reg(Reg::Bx), RegSize::Byte) => write!(output, "%bl"),
        (Operand::Reg(Reg::Bx), RegSize::Long) => write!(output, "%ebx"),
        (Operand::Reg(Reg::Bx), RegSize::Quad) => write!(output, "%rbx"),

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

        (Operand::Reg(Reg::R12), RegSize::Byte) => write!(output, "%r12b"),
        (Operand::Reg(Reg::R12), RegSize::Long) => write!(output, "%r12d"),
        (Operand::Reg(Reg::R12), RegSize::Quad) => write!(output, "%r12"),

        (Operand::Reg(Reg::R13), RegSize::Byte) => write!(output, "%r13b"),
        (Operand::Reg(Reg::R13), RegSize::Long) => write!(output, "%r13d"),
        (Operand::Reg(Reg::R13), RegSize::Quad) => write!(output, "%r13"),

        (Operand::Reg(Reg::R14), RegSize::Byte) => write!(output, "%r14b"),
        (Operand::Reg(Reg::R14), RegSize::Long) => write!(output, "%r14d"),
        (Operand::Reg(Reg::R14), RegSize::Quad) => write!(output, "%r14"),

        (Operand::Reg(Reg::R15), RegSize::Byte) => write!(output, "%r15b"),
        (Operand::Reg(Reg::R15), RegSize::Long) => write!(output, "%r15d"),
        (Operand::Reg(Reg::R15), RegSize::Quad) => write!(output, "%r15"),

        (Operand::Reg(Reg::SP), _) => write!(output, "%rsp"),
        (Operand::Reg(Reg::BP), _) => write!(output, "%rbp"),

        (Operand::Reg(Reg::XMM0), _) => write!(output, "%xmm0"),
        (Operand::Reg(Reg::XMM1), _) => write!(output, "%xmm1"),
        (Operand::Reg(Reg::XMM2), _) => write!(output, "%xmm2"),
        (Operand::Reg(Reg::XMM3), _) => write!(output, "%xmm3"),
        (Operand::Reg(Reg::XMM4), _) => write!(output, "%xmm4"),
        (Operand::Reg(Reg::XMM5), _) => write!(output, "%xmm5"),
        (Operand::Reg(Reg::XMM6), _) => write!(output, "%xmm6"),
        (Operand::Reg(Reg::XMM7), _) => write!(output, "%xmm7"),
        (Operand::Reg(Reg::XMM8), _) => write!(output, "%xmm8"),
        (Operand::Reg(Reg::XMM9), _) => write!(output, "%xmm9"),
        (Operand::Reg(Reg::XMM10), _) => write!(output, "%xmm10"),
        (Operand::Reg(Reg::XMM11), _) => write!(output, "%xmm11"),
        (Operand::Reg(Reg::XMM12), _) => write!(output, "%xmm12"),
        (Operand::Reg(Reg::XMM13), _) => write!(output, "%xmm13"),
        (Operand::Reg(Reg::XMM14), _) => write!(output, "%xmm14"),
        (Operand::Reg(Reg::XMM15), _) => write!(output, "%xmm15"),

        (Operand::Imm(value), _) => write!(output, "${value}"),
        (Operand::Memory(reg, offset), _) => {
            write!(output, "{offset}")?;
            write!(output, "(")?;
            emit_operand(output, &Operand::Reg(*reg), RegSize::Quad, target_os)?;
            write!(output, ")")
        }
        (Operand::Indexed(reg1, reg2, scale), _) => {
            write!(output, "(")?;
            emit_operand(output, &Operand::Reg(*reg1), RegSize::Quad, target_os)?;
            write!(output, ", ")?;
            emit_operand(output, &Operand::Reg(*reg2), RegSize::Quad, target_os)?;
            write!(output, ", ")?;
            write!(output, "{scale}")?;
            write!(output, ")")
        }
        (
            Operand::Data {
                is_static: true,
                name,
                offset,
            },
            _,
        ) => {
            emit_label(output, name, target_os)?;
            write!(output, "+{offset}(%rip)")
        }
        (
            Operand::Data {
                is_static: false,
                name,
                offset,
            },
            _,
        ) => write!(output, "{}+{offset}(%rip)", emit_symbol(name, target_os)),
        (Operand::Pseudo(..) | Operand::PseudoMem(..), _) => {
            unreachable!("Pseudo-registers should not appear here")
        }
    }
}

fn emit_label(output: &mut impl Write, label: &Symbol, target_os: TargetOs) -> Result<()> {
    match target_os {
        TargetOs::MacOs => write!(output, "L{label}"),
        TargetOs::Linux => write!(output, ".L{label}"),
    }
}

fn emit_symbol(name: &Symbol, target_os: TargetOs) -> String {
    match target_os {
        TargetOs::MacOs => format!("_{name}"),
        TargetOs::Linux => name.to_string(),
    }
}
