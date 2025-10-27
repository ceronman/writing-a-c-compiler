use crate::asm::ir::{Function, Instruction, Program, StaticConstant, StaticVariable, TopLevel};
use crate::semantic::StaticInit;
use std::fmt::Write;

type Result<T> = std::result::Result<T, std::fmt::Error>;

pub fn pp(program: &Program) -> Result<String> {
    let mut buffer = String::new();
    let stream = &mut buffer;
    for top_level in &program.top_level {
        match top_level {
            TopLevel::Function(f) => pp_function(stream, f)?,
            TopLevel::Variable(v) => pp_static_variable(stream, v)?,
            TopLevel::Constant(c) => pp_static_constant(stream, c)?,
        }
    }
    Ok(buffer)
}

pub(super) fn pp_function(stream: &mut impl Write, f: &Function) -> Result<()> {
    let global = if f.global { "global " } else { "" };
    writeln!(stream, "{global}function {}", f.name)?;
    for ins in &f.instructions {
        pp_instruction(stream, ins)?;
    }
    writeln!(stream)?;
    Ok(())
}

fn pp_static_variable(stream: &mut impl Write, var: &StaticVariable) -> Result<()> {
    let global = if var.global { "global " } else { "" };
    writeln!(stream, "{global}[{}] static {} = ", var.alignment, var.name)?;
    for init in &var.init {
        write!(stream, "    ")?;
        pp_static_init(stream, init)?;
    }
    writeln!(stream)?;
    Ok(())
}

fn pp_static_init(stream: &mut impl Write, init: &StaticInit) -> Result<()> {
    match init {
        StaticInit::Char(v) => writeln!(stream, "{:?}", (*v as u8) as char)?,
        StaticInit::UChar(v) => writeln!(stream, "{}UC", *v)?,
        StaticInit::Int(v) => writeln!(stream, "{v}")?,
        StaticInit::UInt(v) => writeln!(stream, "{v}U")?,
        StaticInit::Long(v) => writeln!(stream, "{v}L")?,
        StaticInit::ULong(v) => writeln!(stream, "{v}UL")?,
        StaticInit::Double(v) => writeln!(stream, "{v}D")?,
        StaticInit::ZeroInit(v) => writeln!(stream, "zero[{v}]")?,
        StaticInit::String {
            symbol,
            null_terminated,
        } => {
            let s = if *null_terminated {
                format!("{symbol}\\0")
            } else {
                symbol.clone()
            };
            writeln!(stream, "{:?}", s)?
        }
        StaticInit::Pointer(name) => writeln!(stream, "&{name}")?,
    }
    Ok(())
}

fn pp_static_constant(stream: &mut impl Write, c: &StaticConstant) -> Result<()> {
    write!(stream, "[{}] static {} = ", c.alignment, c.name)?;
    pp_static_init(stream, &c.init)?;
    writeln!(stream)?;
    Ok(())
}

fn pp_instruction(stream: &mut impl Write, ins: &Instruction) -> Result<()> {
    writeln!(stream, "    {ins:?}")?;
    Ok(())
}
