use crate::semantic::{StaticInit, Type};
use crate::{ast, tacky};
use std::fmt::Write;

type Result<T> = std::result::Result<T, std::fmt::Error>;

pub fn pp(program: &tacky::Program) -> Result<String> {
    let mut buffer = String::new();
    let stream = &mut buffer;
    for top_level in &program.top_level {
        match top_level {
            tacky::TopLevel::Function(f) => pp_function(stream, f)?,
            tacky::TopLevel::Variable(v) => pp_static_variable(stream, v)?,
            tacky::TopLevel::Constant(c) => pp_static_constant(stream, c)?,
        }
    }
    Ok(buffer)
}

fn pp_static_constant(stream: &mut impl Write, constant: &tacky::StaticConstant) -> Result<()> {
    write!(stream, "constant ")?;
    write!(stream, "{}", constant.name)?;
    write!(stream, ": ")?;
    pp_type(stream, &constant.ty)?;
    write!(stream, " = ")?;
    pp_initializer(stream, &constant.init)?;
    writeln!(stream)?;
    Ok(())
}

fn pp_static_variable(stream: &mut impl Write, variable: &tacky::StaticVariable) -> Result<()> {
    write!(stream, "static ")?;
    if variable.global {
        write!(stream, "global ")?;
    }
    write!(stream, "{}", variable.name)?;
    write!(stream, ": ")?;
    pp_type(stream, &variable.ty)?;
    write!(stream, " = ")?;

    // Shortcut for a nicer single initializer.
    if variable.init.len() == 1 {
        pp_initializer(stream, &variable.init[0])?;
        writeln!(stream)?;
        return Ok(());
    }

    write!(stream, "[ ")?;
    for (i, init) in variable.init.iter().enumerate() {
        pp_initializer(stream, init)?;
        if i != variable.init.len() - 1 {
            write!(stream, ", ")?;
        }
    }
    writeln!(stream, "]")?;
    Ok(())
}

fn pp_initializer(out: &mut impl Write, init: &StaticInit) -> Result<()> {
    match init {
        StaticInit::Char(v) => write!(out, "{:?}", (*v as u8) as char)?,
        StaticInit::UChar(v) => write!(out, "{}UC", *v)?,
        StaticInit::Int(v) => write!(out, "{v}")?,
        StaticInit::UInt(v) => write!(out, "{v}U")?,
        StaticInit::Long(v) => write!(out, "{v}L")?,
        StaticInit::ULong(v) => write!(out, "{v}UL")?,
        StaticInit::Double(v) => write!(out, "{v}D")?,
        StaticInit::ZeroInit(v) => write!(out, "zero[{v}]")?,
        StaticInit::String {
            symbol,
            null_terminated,
        } => {
            let s = if *null_terminated {
                format!("{symbol}\\0")
            } else {
                symbol.clone()
            };
            write!(out, "{s:?}")?
        }
        StaticInit::Pointer(name) => write!(out, "&{name}")?,
    }
    Ok(())
}

pub(super) fn pp_function(stream: &mut impl Write, function: &tacky::Function) -> Result<()> {
    let global = if function.global { "global " } else { "" };
    let params = function.params.to_vec().join(", ");
    writeln!(
        stream,
        "{}function {}({}) {{ ",
        global, function.name, params
    )?;
    for instruction in &function.body {
        pp_instruction(stream, instruction)?;
    }
    writeln!(stream, "}}")?;
    Ok(())
}

pub(crate) fn pp_instruction(
    stream: &mut impl Write,
    instruction: &tacky::Instruction,
) -> Result<()> {
    let indent = "    ";
    match instruction {
        tacky::Instruction::Return(val) => {
            write!(stream, "{indent}return ")?;
            if let Some(val) = val {
                pp_val(stream, val)?;
            } else {
                writeln!(stream)?;
            }
        }
        tacky::Instruction::Unary { op, src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = ")?;
            write!(
                stream,
                "{} ",
                match op {
                    tacky::UnaryOp::Complement => "~",
                    tacky::UnaryOp::Negate => "-",
                    tacky::UnaryOp::Not => "!",
                    tacky::UnaryOp::Increment => "inc",
                    tacky::UnaryOp::Decrement => "dec",
                }
            )?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::Binary {
            op,
            src1,
            src2,
            dst,
        } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = ")?;
            pp_val(stream, src1)?;
            write!(
                stream,
                " {} ",
                match op {
                    tacky::BinaryOp::Add => "+",
                    tacky::BinaryOp::Subtract => "-",
                    tacky::BinaryOp::Multiply => "*",
                    tacky::BinaryOp::Divide => "/",
                    tacky::BinaryOp::Reminder => "%",
                    tacky::BinaryOp::BinAnd => "&",
                    tacky::BinaryOp::BinOr => "|",
                    tacky::BinaryOp::BinXor => "^",
                    tacky::BinaryOp::ShiftLeft => "<<",
                    tacky::BinaryOp::ShiftRight => ">>",
                    tacky::BinaryOp::Equal => "==",
                    tacky::BinaryOp::NotEqual => "!=",
                    tacky::BinaryOp::LessThan => "<",
                    tacky::BinaryOp::LessOrEqual => "<=",
                    tacky::BinaryOp::GreaterThan => ">",
                    tacky::BinaryOp::GreaterOrEqual => ">=",
                }
            )?;
            pp_val(stream, src2)?;
        }
        tacky::Instruction::Copy { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::Jump { target } => {
            write!(stream, "{indent}jump {target}")?;
        }
        tacky::Instruction::JumpIfZero { cond, target } => {
            write!(stream, "{indent}if !")?;
            pp_val(stream, cond)?;
            write!(stream, " jump {target}")?;
        }
        tacky::Instruction::JumpIfNotZero { cond, target } => {
            write!(stream, "{indent}if ")?;
            pp_val(stream, cond)?;
            write!(stream, " jump {target}")?;
        }
        tacky::Instruction::Label(name) => {
            writeln!(stream)?;
            write!(stream, "  {name}:")?;
        }
        tacky::Instruction::FnCall { name, args, dst } => {
            write!(stream, "{indent}")?;
            if let Some(dst) = dst {
                pp_val(stream, dst)?;
                write!(stream, " = ")?;
            }
            write!(stream, "{name}(")?;
            for (i, arg) in args.iter().enumerate() {
                pp_val(stream, arg)?;
                if i != args.len() - 1 {
                    write!(stream, ", ")?;
                }
            }
            write!(stream, ")")?;
        }
        tacky::Instruction::SignExtend { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = sign_extend ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::Truncate { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = truncate ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::ZeroExtend { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = zero_extend ")?;
            pp_val(stream, src)?;
        }

        tacky::Instruction::DoubleToInt { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = double_to_int ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::DoubleToUInt { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = double_to_uint ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::IntToDouble { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = int_to_double ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::UIntToDouble { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = uint_to_double ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::GetAddress { src, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = &")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::Load { ptr, dst } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = *")?;
            pp_val(stream, ptr)?;
        }
        tacky::Instruction::Store { src, ptr } => {
            write!(stream, "{indent}")?;
            write!(stream, "*")?;
            pp_val(stream, ptr)?;
            write!(stream, " = ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::AddPtr {
            ptr,
            index,
            scale,
            dst,
        } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = add_ptr(")?;
            pp_val(stream, ptr)?;
            write!(stream, ", index=")?;
            pp_val(stream, index)?;
            write!(stream, ", scale=")?;
            write!(stream, "{scale})")?;
        }
        tacky::Instruction::CopyToOffset { src, dst, offset } => {
            write!(stream, "{indent}")?;
            write!(stream, "{dst}[{offset}] = ")?;
            pp_val(stream, src)?;
        }
        tacky::Instruction::CopyFromOffset { src, dst, offset } => {
            write!(stream, "{indent}")?;
            pp_val(stream, dst)?;
            write!(stream, " = {src}[{offset}]")?;
        }
    }
    writeln!(stream)?;
    Ok(())
}

fn pp_val(stream: &mut impl Write, val: &tacky::Val) -> Result<()> {
    match val {
        tacky::Val::Constant(ast::Constant::Char(value)) => {
            write!(stream, "{:?}", (*value as u8) as char)?
        }
        tacky::Val::Constant(ast::Constant::UChar(value)) => write!(stream, "{value}UC")?,
        tacky::Val::Constant(ast::Constant::Int(value)) => write!(stream, "{value}")?,
        tacky::Val::Constant(ast::Constant::Long(value)) => write!(stream, "{value}L")?,
        tacky::Val::Constant(ast::Constant::UInt(value)) => write!(stream, "{value}U")?,
        tacky::Val::Constant(ast::Constant::ULong(value)) => write!(stream, "{value}UL")?,
        tacky::Val::Constant(ast::Constant::Double(value)) => write!(stream, "{value}D")?,
        tacky::Val::Var(name) => write!(stream, "{name}")?,
    }
    Ok(())
}

fn pp_type(stream: &mut impl Write, ty: &Type) -> Result<()> {
    match ty {
        Type::Char => write!(stream, "Char"),
        Type::SChar => write!(stream, "Signed Char"),
        Type::UChar => write!(stream, "Unsigned Char"),
        Type::Int => write!(stream, "Int"),
        Type::Long => write!(stream, "Long"),
        Type::UInt => write!(stream, "Unsigned Int"),
        Type::ULong => write!(stream, "Unsigned Long"),
        Type::Function(_) => write!(stream, "Function(...)"),
        Type::Double => write!(stream, "Double"),
        Type::Void => write!(stream, "Void"),
        Type::Pointer(referenced) => {
            write!(stream, "Pointer(")?;
            pp_type(stream, referenced.as_ref())?;
            write!(stream, ")")
        }
        Type::Array(inner, size) => {
            write!(stream, "Array(")?;
            write!(stream, "{size},")?;
            pp_type(stream, inner.as_ref())?;
            write!(stream, ")")
        }
        Type::Struct(name) => {
            write!(stream, "Struct({name})")
        }
        Type::Union(name) => {
            write!(stream, "Union({name})")
        }
    }?;
    Ok(())
}
