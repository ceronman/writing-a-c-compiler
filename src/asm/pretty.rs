use crate::asm::ir::{Function, Instruction, Program, StaticConstant, StaticVariable, TopLevel};
use crate::semantic::StaticInit;
use std::io::Write;

pub fn pp(program: &Program) -> anyhow::Result<String> {
    let mut buffer = Vec::new();
    let file = &mut buffer;
    for top_level in &program.top_level {
        match top_level {
            TopLevel::Function(f) => pp_function(file, f)?,
            TopLevel::Variable(v) => pp_static_variable(file, v)?,
            TopLevel::Constant(c) => pp_static_constant(file, c)?,
        }
    }
    Ok(String::from_utf8(buffer)?)
}

fn pp_function(out: &mut impl Write, f: &Function) -> anyhow::Result<()> {
    let global = if f.global { "global " } else { "" };
    writeln!(out, "{global}function {}", f.name)?;
    for ins in &f.instructions {
        pp_instruction(out, ins)?;
    }
    writeln!(out)?;
    Ok(())
}

fn pp_static_variable(out: &mut impl Write, var: &StaticVariable) -> anyhow::Result<()> {
    let global = if var.global { "global " } else { "" };
    writeln!(out, "{global}[{}] static {} = ", var.alignment, var.name)?;
    for init in &var.init {
        write!(out, "    ")?;
        pp_static_init(out, init)?;
    }
    writeln!(out)?;
    Ok(())
}

fn pp_static_init(out: &mut impl Write, init: &StaticInit) -> anyhow::Result<()> {
    match init {
        StaticInit::Char(v) => writeln!(out, "{:?}", (*v as u8) as char)?,
        StaticInit::UChar(v) => writeln!(out, "{:?}U", *v as char)?,
        StaticInit::Int(v) => writeln!(out, "{v}")?,
        StaticInit::UInt(v) => writeln!(out, "{v}U")?,
        StaticInit::Long(v) => writeln!(out, "{v}L")?,
        StaticInit::ULong(v) => writeln!(out, "{v}UL")?,
        StaticInit::Double(v) => writeln!(out, "{v}D")?,
        StaticInit::ZeroInit(v) => writeln!(out, "Zero({v})")?,
        StaticInit::String {
            symbol,
            null_terminated,
        } => writeln!(
            out,
            "\"{symbol}{}\"",
            if *null_terminated { "\\0" } else { "" }
        )?,
        StaticInit::Pointer(name) => writeln!(out, "&{name}")?,
    }
    Ok(())
}

fn pp_static_constant(out: &mut impl Write, c: &StaticConstant) -> anyhow::Result<()> {
    write!(out, "[{}] static {} = ", c.alignment, c.name)?;
    pp_static_init(out, &c.init)?;
    writeln!(out)?;
    Ok(())
}

fn pp_instruction(out: &mut impl Write, ins: &Instruction) -> anyhow::Result<()> {
    writeln!(out, "    {ins:?}")?;
    Ok(())
}
