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
        match init {
            StaticInit::Int(v) => writeln!(out, "{v}")?,
            StaticInit::UInt(v) => writeln!(out, "{v}U")?,
            StaticInit::Long(v) => writeln!(out, "{v}L")?,
            StaticInit::ULong(v) => writeln!(out, "{v}UL")?,
            StaticInit::Double(v) => writeln!(out, "{v}D")?,
            StaticInit::ZeroInit(v) => writeln!(out, "Zero({v})")?,
        }
    }
    writeln!(out)?;
    Ok(())
}

fn pp_static_constant(out: &mut impl Write, c: &StaticConstant) -> anyhow::Result<()> {
    write!(out, "[{}] static {} = ", c.alignment, c.name)?;
    match c.init {
        StaticInit::Int(v) => writeln!(out, "{v}")?,
        StaticInit::UInt(v) => writeln!(out, "{v}U")?,
        StaticInit::Long(v) => writeln!(out, "{v}L")?,
        StaticInit::ULong(v) => writeln!(out, "{v}UL")?,
        StaticInit::Double(v) => writeln!(out, "{v}D")?,
        StaticInit::ZeroInit(v) => writeln!(out, "Zero({v})")?,
    }
    writeln!(out)?;
    Ok(())
}

fn pp_instruction(out: &mut impl Write, ins: &Instruction) -> anyhow::Result<()> {
    writeln!(out, "    {ins:?}")?;
    Ok(())
}
