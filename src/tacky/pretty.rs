use crate::semantic::{StaticInit, Type};
use crate::{ast, tacky};
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn pp(program: &tacky::Program) -> Result<String> {
    let mut buffer = Vec::new();
    let file = &mut buffer;
    for top_level in &program.top_level {
        match top_level {
            tacky::TopLevel::Function(f) => pp_function(file, f)?,
            tacky::TopLevel::Variable(v) => pp_static_variable(file, v)?,
            tacky::TopLevel::Constant(c) => pp_static_constant(file, c)?,
        }
    }
    Ok(String::from_utf8(buffer)?)
}

fn pp_static_constant(file: &mut impl Write, constant: &tacky::StaticConstant) -> Result<()> {
    write!(file, "constant ")?;
    write!(file, "{}", constant.name)?;
    write!(file, ": ")?;
    pp_type(file, &constant.ty)?;
    write!(file, " = ")?;
    pp_initializer(file, &constant.init)?;
    writeln!(file)?;
    Ok(())
}

fn pp_static_variable(file: &mut impl Write, variable: &tacky::StaticVariable) -> Result<()> {
    write!(file, "static ")?;
    if variable.global {
        write!(file, "global ")?;
    }
    write!(file, "{}", variable.name)?;
    write!(file, ": ")?;
    pp_type(file, &variable.ty)?;
    write!(file, " = ")?;

    // Shortcut for a nicer single initializer.
    if variable.init.len() == 1 {
        pp_initializer(file, &variable.init[0])?;
        writeln!(file)?;
        return Ok(());
    }

    write!(file, "[ ")?;
    for (i, init) in variable.init.iter().enumerate() {
        pp_initializer(file, init)?;
        if i != variable.init.len() - 1 {
            write!(file, ", ")?;
        }
    }
    writeln!(file, "]")?;
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

fn pp_function(file: &mut impl Write, function: &tacky::Function) -> Result<()> {
    let global = if function.global { "global " } else { "" };
    let params = function.params.to_vec().join(", ");
    writeln!(file, "{}function {}({}) {{ ", global, function.name, params)?;
    let indent = "    ";
    for item in &function.body {
        match item {
            tacky::Instruction::Return(val) => {
                write!(file, "{indent}return ")?;
                if let Some(val) = val {
                    pp_val(file, val)?;
                } else {
                    writeln!(file)?;
                }
            }
            tacky::Instruction::Unary { op, src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = ")?;
                write!(
                    file,
                    "{} ",
                    match op {
                        tacky::UnaryOp::Complement => "~",
                        tacky::UnaryOp::Negate => "-",
                        tacky::UnaryOp::Not => "!",
                        tacky::UnaryOp::Increment => "inc",
                        tacky::UnaryOp::Decrement => "dec",
                    }
                )?;
                pp_val(file, src)?;
            }
            tacky::Instruction::Binary {
                op,
                src1,
                src2,
                dst,
            } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = ")?;
                pp_val(file, src1)?;
                write!(
                    file,
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
                pp_val(file, src2)?;
            }
            tacky::Instruction::Copy { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::Jump { target } => {
                write!(file, "{indent}jump {target}")?;
            }
            tacky::Instruction::JumpIfZero { cond, target } => {
                write!(file, "{indent}if !")?;
                pp_val(file, cond)?;
                write!(file, " jump {target}")?;
            }
            tacky::Instruction::JumpIfNotZero { cond, target } => {
                write!(file, "{indent}if ")?;
                pp_val(file, cond)?;
                write!(file, " jump {target}")?;
            }
            tacky::Instruction::Label(name) => {
                writeln!(file)?;
                write!(file, "  {name}:")?;
            }
            tacky::Instruction::FnCall { name, args, dst } => {
                write!(file, "{indent}")?;
                if let Some(dst) = dst {
                    pp_val(file, dst)?;
                    write!(file, " = ")?;
                }
                write!(file, "{name}(")?;
                for (i, arg) in args.iter().enumerate() {
                    pp_val(file, arg)?;
                    if i != args.len() - 1 {
                        write!(file, ", ")?;
                    }
                }
                write!(file, ")")?;
            }
            tacky::Instruction::SignExtend { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = sign_extend ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::Truncate { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = truncate ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::ZeroExtend { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = zero_extend ")?;
                pp_val(file, src)?;
            }

            tacky::Instruction::DoubleToInt { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = double_to_int ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::DoubleToUInt { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = double_to_uint ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::IntToDouble { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = int_to_double ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::UIntToDouble { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = uint_to_double ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::GetAddress { src, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = &")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::Load { ptr, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = *")?;
                pp_val(file, ptr)?;
            }
            tacky::Instruction::Store { src, ptr } => {
                write!(file, "{indent}")?;
                write!(file, "*")?;
                pp_val(file, ptr)?;
                write!(file, " = ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::AddPtr {
                ptr,
                index,
                scale,
                dst,
            } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = add_ptr(")?;
                pp_val(file, ptr)?;
                write!(file, ", index=")?;
                pp_val(file, index)?;
                write!(file, ", scale=")?;
                write!(file, "{scale})")?;
            }
            tacky::Instruction::CopyToOffset { src, dst, offset } => {
                write!(file, "{indent}")?;
                write!(file, "{dst}[{offset}] = ")?;
                pp_val(file, src)?;
            }
        }
        writeln!(file)?;
    }
    writeln!(file, "}}")?;
    Ok(())
}

fn pp_val(file: &mut impl Write, val: &tacky::Val) -> Result<()> {
    match val {
        tacky::Val::Constant(ast::Constant::Char(value)) => {
            write!(file, "{:?}", (*value as u8) as char)?
        }
        tacky::Val::Constant(ast::Constant::UChar(value)) => write!(file, "{value}UC")?,
        tacky::Val::Constant(ast::Constant::Int(value)) => write!(file, "{value}")?,
        tacky::Val::Constant(ast::Constant::Long(value)) => write!(file, "{value}L")?,
        tacky::Val::Constant(ast::Constant::UInt(value)) => write!(file, "{value}U")?,
        tacky::Val::Constant(ast::Constant::ULong(value)) => write!(file, "{value}UL")?,
        tacky::Val::Constant(ast::Constant::Double(value)) => write!(file, "{value}D")?,
        tacky::Val::Var(name) => write!(file, "{name}")?,
    }
    Ok(())
}

fn pp_type(file: &mut impl Write, ty: &Type) -> Result<()> {
    match ty {
        Type::Char => write!(file, "Char"),
        Type::SChar => write!(file, "Signed Char"),
        Type::UChar => write!(file, "Unsigned Char"),
        Type::Int => write!(file, "Int"),
        Type::Long => write!(file, "Long"),
        Type::UInt => write!(file, "Unsigned Int"),
        Type::ULong => write!(file, "Unsigned Long"),
        Type::Function(_) => write!(file, "Function(...)"),
        Type::Double => write!(file, "Double"),
        Type::Void => write!(file, "Void"),
        Type::Pointer(referenced) => {
            write!(file, "Pointer(")?;
            pp_type(file, referenced.as_ref())?;
            write!(file, ")")
        }
        Type::Array(inner, size) => {
            write!(file, "Array(")?;
            write!(file, "{size},")?;
            pp_type(file, inner.as_ref())?;
            write!(file, ")")
        }
        Type::Struct(name) => {
            write!(file, "Struct({name})")
        }
    }?;
    Ok(())
}
