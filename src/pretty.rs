use crate::ast;
use crate::tacky;
use anyhow::Result;
use std::io::Write;

#[allow(dead_code)]
pub fn dump_ast(src: &str) -> String {
    let ast = crate::parser::parse(src).unwrap();
    pretty_print_ast(&ast).unwrap()
}

#[allow(dead_code)]
pub fn dump_tacky(src: &str) -> String {
    let ast = crate::parser::parse(src).unwrap();
    let ast = crate::resolver::resolve(ast).unwrap();
    let tacky = crate::tacky::emit(&ast);
    pretty_print_tacky(tacky).unwrap()
}

pub fn pretty_print_ast(program: &ast::Program) -> Result<String> {
    let mut buffer = Vec::new();
    print_program(&mut buffer, program)?;
    Ok(String::from_utf8(buffer)?.trim().into())
}

pub fn pretty_print_tacky(program: tacky::Program) -> Result<String> {
    let mut buffer = Vec::new();
    writeln!(&mut buffer, "function {} {{ ", program.function.name)?;
    let file = &mut buffer;
    for item in &program.function.body {
        match item {
            tacky::Instruction::Return(val) => {
                write!(file, "  return ")?;
                print_val(file, val)?;
            }
            tacky::Instruction::Unary { op, src, dst } => {
                write!(file, "  ")?;
                print_val(file, dst)?;
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
                print_val(file, src)?;
            }
            tacky::Instruction::Binary {
                op,
                src1,
                src2,
                dst,
            } => {
                write!(file, "  ")?;
                print_val(file, dst)?;
                write!(file, " = ")?;
                print_val(file, src1)?;
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
                print_val(file, src2)?;
            }
            tacky::Instruction::Copy { src, dst } => {
                write!(file, "  ")?;
                print_val(file, dst)?;
                write!(file, " = ")?;
                print_val(file, src)?;
            }
            tacky::Instruction::Jump { target } => {
                write!(file, "  jump {target}")?;
            }
            tacky::Instruction::JumpIfZero { cond, target } => {
                write!(file, "  jump {target}")?;
                write!(file, " if !")?;
                print_val(file, cond)?;
            }
            tacky::Instruction::JumpIfNotZero { cond, target } => {
                write!(file, "  jump {target}")?;
                write!(file, "if ")?;
                print_val(file, cond)?;
            }
            tacky::Instruction::Label(name) => {
                write!(file, "{name}:")?;
            }
        }
        writeln!(file)?;
    }
    writeln!(&mut buffer, "}}")?;
    Ok(String::from_utf8(buffer)?.trim().into())
}

pub fn print_val(file: &mut impl Write, val: &tacky::Val) -> Result<()> {
    match val {
        tacky::Val::Constant(value) => write!(file, "{value}")?,
        tacky::Val::Var(name) => write!(file, "{name}")?,
    }
    Ok(())
}

#[allow(dead_code)]
pub fn dedent(tree: &str) -> String {
    tree.trim()
        .lines()
        .map(|l| l.strip_prefix("        ").unwrap_or(l))
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
pub fn remove_annotation(src: &str) -> String {
    src.lines()
        .filter(|l| !l.trim().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
pub fn annotate(src: &str, error: &crate::error::CompilerError) -> String {
    let mut result = String::new();
    let mut offset = 0;
    let mut annotated = false;
    for line in src.split_inclusive('\n') {
        result.push_str(line);
        if !annotated && offset + line.len() > error.span.0 {
            let start = error.span.0 - offset - 2;
            let len = error.span.1 - error.span.0;
            let annotation = format!("{}//{} {}\n", " ".repeat(start), "^".repeat(len), error.msg);
            result.push_str(&annotation);
            annotated = true
        }
        offset += line.len();
    }
    if !annotated {
        result.push_str(&format!("\n// {}", error.msg));
    }
    result
}

fn print_program(file: &mut impl Write, program: &ast::Program) -> Result<()> {
    writeln!(file, "Program")?;
    print_function(file, &program.function_definition)?;
    Ok(())
}

fn print_function(file: &mut impl Write, function: &ast::Function) -> Result<()> {
    writeln!(file, "╰── Function [{}]", function.name.symbol)?;
    let body = &function.body;
    for (i, block_item) in body.items.iter().enumerate() {
        if i == body.items.len() - 1 {
            print_block_item(file, block_item, "╰──", 1, &[])?
        } else {
            print_block_item(file, block_item, "├──", 1, &[1])?
        };
    }
    Ok(())
}

fn print_block_item(
    file: &mut impl Write,
    item: &ast::BlockItem,
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
    match item {
        ast::BlockItem::Stmt(s) => print_statement(file, s, pipe, level, pipes)?,
        ast::BlockItem::Decl(d) => {
            writeln!(file, "{indent}{pipe} Declaration [{}]", d.name.symbol)?;
            if let Some(init) = &d.init {
                print_expression(file, init, "╰──", level + 1, pipes)?;
            }
        }
    }
    Ok(())
}

fn print_statement(
    file: &mut impl Write,
    statement: &ast::Statement,
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
    match statement {
        ast::Statement::Return { expr } => {
            writeln!(file, "{indent}{pipe} Return")?;
            print_expression(file, expr, "╰──", level + 1, pipes)?;
        }
        ast::Statement::If {
            cond,
            then_stmt,
            else_stmt,
        } => {
            writeln!(file, "{indent}{pipe} If")?;
            print_expression(
                file,
                cond,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            if let Some(else_stmt) = else_stmt {
                print_statement(
                    file,
                    then_stmt,
                    "├──",
                    level + 1,
                    &[pipes, &[level + 1]].concat(),
                )?;
                print_statement(file, else_stmt, "╰──", level + 1, pipes)?;
            } else {
                print_statement(file, then_stmt, "╰──", level + 1, pipes)?;
            }
            writeln!(file, "{indent}{pipe} If")?;
        }
        ast::Statement::Expression(expr) => {
            print_expression(file, expr, pipe, level, pipes)?;
        }
        ast::Statement::Goto(label) => {
            writeln!(file, "{indent}{pipe} Goto [{}]", label.symbol)?;
        }
        ast::Statement::Labeled { name, stmt } => {
            writeln!(file, "{indent}{pipe} Label [{}]", name.symbol)?;
            print_statement(file, stmt, "╰──", level + 1, pipes)?;
        }
        ast::Statement::Compound(block) => {
            writeln!(file, "{indent}{pipe} Block")?;
            for (i, block_item) in block.items.iter().enumerate() {
                if i == block.items.len() - 1 {
                    print_block_item(file, block_item, "╰──", level + 1, pipes)?
                } else {
                    print_block_item(file, block_item, "├──", level + 1, &[pipes, &[level + 1]].concat())?
                };
            }
        }
        ast::Statement::Null => {}
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
        ast::Expression::Var(value) => {
            writeln!(file, "{indent}{pipe} Var [{value}]")?;
        }
        ast::Expression::Unary { op, expr } => {
            writeln!(file, "{indent}{pipe} Unary [{}]", unary_op(op))?;
            print_expression(file, expr, "╰──", level + 1, pipes)?;
        }
        ast::Expression::Postfix { op, expr } => {
            writeln!(file, "{indent}{pipe} Postfix [{}]", postfix_op(op))?;
            print_expression(file, expr, "╰──", level + 1, pipes)?;
        }
        ast::Expression::Binary { op, left, right } => {
            writeln!(file, "{indent}{pipe} Binary [{}]", binary_op(op))?;
            print_expression(
                file,
                left,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            print_expression(file, right, "╰──", level + 1, pipes)?;
        }
        ast::Expression::Assignment { op, left, right } => {
            writeln!(file, "{indent}{pipe} Assign [{}]", assign_op(op))?;
            let mut added = pipes.to_vec();
            added.push(level + 1);
            print_expression(file, left, "├──", level + 1, &added)?;
            print_expression(file, right, "╰──", level + 1, pipes)?;
        }
        ast::Expression::Conditional {
            cond,
            then_expr,
            else_expr,
        } => {
            writeln!(file, "{indent}{pipe} Cond [?]")?;
            print_expression(
                file,
                cond,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            print_expression(
                file,
                then_expr,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            print_expression(file, else_expr, "╰──", level + 1, pipes)?;
        }
    }
    Ok(())
}

fn binary_op(op: &ast::BinaryOp) -> &str {
    use ast::BinaryOp::*;
    match op {
        Add => "+",
        Subtract => "-",
        Multiply => "*",
        Divide => "/",
        Reminder => "%",
        BinAnd => "&",
        BinOr => "|",
        BinXor => "^",
        ShiftLeft => "<<",
        ShiftRight => ">>",
        And => "&&",
        Or => "||",
        Equal => "==",
        NotEqual => "!=",
        LessThan => "<",
        LessOrEqualThan => "<=",
        GreaterThan => ">",
        GreaterOrEqualThan => ">=",
    }
}

fn assign_op(op: &ast::AssignOp) -> &str {
    use ast::AssignOp::*;
    match op {
        Equal => "=",
        AddEqual => "+=",
        SubEqual => "-=",
        MulEqual => "*=",
        DivEqual => "/=",
        ModEqual => "&=",
        BitAndEqual => "&=",
        BitOrEqual => "|=",
        BitXorEqual => "^=",
        ShiftLeftEqual => "<<=",
        ShiftRightEqual => ">>=",
    }
}

fn unary_op(op: &ast::UnaryOp) -> &str {
    use ast::UnaryOp::*;
    match op {
        Complement => "~",
        Negate => "-",
        Not => "!",
        Increment => "++",
        Decrement => "--",
    }
}

fn postfix_op(op: &ast::PostfixOp) -> &str {
    use ast::PostfixOp::*;
    match op {
        Increment => "++",
        Decrement => "--",
    }
}
