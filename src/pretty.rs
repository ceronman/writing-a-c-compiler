use crate::ast;
use anyhow::Result;
use std::io::Write;

pub fn print_program(file: &mut impl Write, program: &ast::Program) -> Result<()> {
    writeln!(file, "Program")?;
    print_function(file, &program.function_definition)?;
    Ok(())
}

fn print_function(file: &mut impl Write, function: &ast::Function) -> Result<()> {
    writeln!(file, "╰── Function [{}]", function.name)?;
    print_statement(file, &function.body)?;
    Ok(())
}

fn print_statement(file: &mut impl Write, statement: &ast::Statement) -> Result<()> {
    let level = 1;
    let ident = "    ".repeat(level);
    match statement {
        ast::Statement::Return { expr } => {
            writeln!(file, "{ident}╰── return")?;
            print_expression(file, expr, "╰──", level + 1, &[])?;
        }
    }
    Ok(())
}

fn print_expression(
    file: &mut impl Write,
    expression: &ast::Expression,
    pipe: &str,
    level: usize,
    pipes: &[usize],
) -> Result<()> {
    let mut indent = String::new();
    for l in 0..level {
        if pipes.contains(&l) {
            indent.push_str("│   ");
        } else {
            indent.push_str("    ");
        }
    }
    match expression {
        ast::Expression::Constant(value) => {
            writeln!(file, "{indent}{pipe} Constant [{value}]")?;
        }
        ast::Expression::Unary { op, expr } => {
            writeln!(file, "{indent}{pipe} Unary [{}]", unary_op(op))?;
            print_expression(file, expr, "╰──", level + 1, pipes)?;
        }
        ast::Expression::Binary { op, left, right } => {
            writeln!(file, "{indent}{pipe} Binary [{}]", binary_op(op))?;
            let mut added = pipes.to_vec();
            added.push(level + 1);
            print_expression(file, left, "├──", level + 1, &added)?;
            print_expression(file, right, "╰──", level + 1, pipes)?;
        }
    }
    Ok(())
}

fn binary_op(op: &ast::BinaryOp) -> &str {
    match op {
        ast::BinaryOp::Add => "+",
        ast::BinaryOp::Subtract => "-",
        ast::BinaryOp::Multiply => "*",
        ast::BinaryOp::Divide => "/",
        ast::BinaryOp::Reminder => "%",
        ast::BinaryOp::BinAnd => "&",
        ast::BinaryOp::BinOr => "|",
        ast::BinaryOp::BinXor => "^",
        ast::BinaryOp::ShiftLeft => "<<",
        ast::BinaryOp::ShiftRight => ">>",
    }
}

fn unary_op(op: &ast::UnaryOp) -> &str {
    match op {
        ast::UnaryOp::Complement => "~",
        ast::UnaryOp::Negate => "-",
    }
}
