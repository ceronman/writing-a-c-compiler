use crate::ast;
use crate::parser;
use crate::semantic;
use crate::tacky;

use anyhow::Result;
use std::io::Write;

#[allow(dead_code)]
pub fn dump_ast(src: &str) -> String {
    let ast = parser::parse(src).unwrap();
    pretty_print_ast(&ast).unwrap()
}

#[allow(dead_code)]
pub fn dump_tacky(src: &str) -> String {
    let ast = parser::parse(src).unwrap();
    let ast = semantic::validate(ast).unwrap();
    let tacky = tacky::emit(&ast);
    pp_tacky(&tacky).unwrap().trim().to_owned()
}

pub fn pretty_print_ast(program: &ast::Program) -> Result<String> {
    let mut buffer = Vec::new();
    print_program(&mut buffer, program)?;
    Ok(String::from_utf8(buffer)?.trim().into())
}

pub fn pp_tacky(program: &tacky::Program) -> Result<String> {
    let mut buffer = Vec::new();
    let file = &mut buffer;
    for function in &program.functions {
        pp_function(file, function)?;
    }
    Ok(String::from_utf8(buffer)?)
}

pub fn pp_function(file: &mut impl Write, function: &tacky::Function) -> Result<()> {
    writeln!(file, "function {} {{ ", function.name)?;
    let indent = "    ";
    for item in &function.body {
        match item {
            tacky::Instruction::Return(val) => {
                write!(file, "{indent}return ")?;
                pp_val(file, val)?;
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
                pp_val(file, dst)?;
                write!(file, " = ")?;
                write!(file, "{name}(")?;
                for (i, arg) in args.iter().enumerate() {
                    pp_val(file, arg)?;
                    if i != args.len() - 1 {
                        write!(file, ", ")?;
                    }
                }
                write!(file, ")")?;
            }
        }
        writeln!(file)?;
    }
    writeln!(file, "}}")?;
    Ok(())
}

pub fn pp_val(file: &mut impl Write, val: &tacky::Val) -> Result<()> {
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

// TODO: refactor this with an homogeneous tree IR, right now it's too complicated.
fn print_program(file: &mut impl Write, program: &ast::Program) -> Result<()> {
    writeln!(file, "Program")?;
    for (i, function) in program.functions.iter().enumerate() {
        if i < program.functions.len() - 1 {
            print_function_declaration(file, function, "├──", 1, &[1])?;
        } else {
            print_function_declaration(file, function, "╰──", 1, &[])?;
        }
    }
    Ok(())
}

fn print_function_declaration(
    file: &mut impl Write,
    function: &ast::FunctionDeclaration,
    pipe: &str,
    level: usize,
    pipes: &[usize],
) -> Result<()> {
    let indent = make_indent(level, pipes);
    writeln!(file, "{indent}{pipe} Function [{}]", function.name.symbol)?;
    let inner_indent = make_indent(level + 1, pipes);
    if !function.params.is_empty() {
        let pipe = if function.body.is_some() {
            "├──"
        } else {
            "╰──"
        };
        writeln!(file, "{inner_indent}{pipe} Parameters")?;
        let param_indent = make_indent(level + 2, pipes);
        for (i, param) in function.params.iter().enumerate() {
            let pipe = if i < function.params.len() - 1 {
                "├──"
            } else {
                "╰──"
            };
            writeln!(file, "{param_indent}{pipe} {}", param.symbol)?;
        }
    }
    if let Some(body) = function.body.as_ref() {
        writeln!(file, "{inner_indent}╰── Body")?;
        for (i, block_item) in body.items.iter().enumerate() {
            if i < body.items.len() - 1 {
                print_block_item(
                    file,
                    block_item,
                    "├──",
                    level + 2,
                    &[pipes, &[level + 1]].concat(),
                )?
            } else {
                print_block_item(file, block_item, "╰──", level + 2, pipes)?
            };
        }
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
    match item {
        ast::BlockItem::Stmt(s) => print_statement(file, s, pipe, level, pipes),
        ast::BlockItem::Decl(d) => print_declaration(file, d, pipe, level, pipes),
    }
}

fn make_indent(level: usize, pipes: &[usize]) -> String {
    let mut indent = String::new();
    for l in 0..level {
        if pipes.contains(&l) {
            indent.push_str("│   ");
        } else {
            indent.push_str("    ");
        }
    }
    indent
}

fn print_declaration(
    file: &mut impl Write,
    declaration: &ast::Declaration,
    pipe: &str,
    level: usize,
    pipes: &[usize],
) -> Result<()> {
    match declaration {
        ast::Declaration::Var(decl) => print_var_declaration(file, decl, pipe, level, pipes),
        ast::Declaration::Function(func) => {
            print_function_declaration(file, func, pipe, level, pipes)
        }
    }
}

fn print_var_declaration(
    file: &mut impl Write,
    declaration: &ast::VarDeclaration,
    pipe: &str,
    level: usize,
    pipes: &[usize],
) -> Result<()> {
    let indent = make_indent(level, pipes);
    writeln!(file, "{indent}{pipe} Var [{}]", declaration.name.symbol)?;
    if let Some(init) = &declaration.init {
        print_expression(file, init, "╰──", level + 1, pipes)?;
    }
    Ok(())
}

fn print_named_statement(
    file: &mut impl Write,
    name: &str,
    stmt: &ast::Statement,
    pipe: &str,
    level: usize,
    pipes: &[usize],
) -> Result<()> {
    let indent = make_indent(level, pipes);
    writeln!(file, "{indent}{pipe} {name}")?;
    print_statement(file, stmt, "╰──", level + 1, pipes)
}

fn print_named_expression(
    file: &mut impl Write,
    name: &str,
    expr: &ast::Expression,
    pipe: &str,
    level: usize,
    pipes: &[usize],
) -> Result<()> {
    let indent = make_indent(level, pipes);
    writeln!(file, "{indent}{pipe} {name}")?;
    print_expression(file, expr, "╰──", level + 1, pipes)
}

fn print_named_var_declaration(
    file: &mut impl Write,
    name: &str,
    decl: &ast::VarDeclaration,
    pipe: &str,
    level: usize,
    pipes: &[usize],
) -> Result<()> {
    let indent = make_indent(level, pipes);
    writeln!(file, "{indent}{pipe} {name}")?;
    print_var_declaration(file, decl, "╰──", level + 1, pipes)
}

fn print_statement(
    file: &mut impl Write,
    statement: &ast::Statement,
    pipe: &str,
    level: usize,
    pipes: &[usize],
) -> Result<()> {
    let indent = make_indent(level, pipes);
    match statement {
        ast::Statement::Return { expr } => {
            print_named_expression(file, "Return", expr, "╰──", level, pipes)?;
        }
        ast::Statement::If {
            cond,
            then_stmt,
            else_stmt,
        } => {
            writeln!(file, "{indent}{pipe} If")?;
            print_named_expression(
                file,
                "Condition",
                cond,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            if let Some(else_stmt) = else_stmt {
                print_named_statement(
                    file,
                    "Then",
                    then_stmt,
                    "├──",
                    level + 1,
                    &[pipes, &[level + 1]].concat(),
                )?;
                print_named_statement(file, "Else", else_stmt, "╰──", level + 1, pipes)?;
            } else {
                print_named_statement(file, "Then", then_stmt, "╰──", level + 1, pipes)?;
            }
        }
        ast::Statement::Switch {
            expr: cond, body, ..
        } => {
            writeln!(file, "{indent}{pipe} Switch")?;
            print_named_expression(
                file,
                "Condition",
                cond,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            print_statement(file, body, "╰──", level + 1, pipes)?;
        }
        ast::Statement::Expression(expr) => {
            print_expression(file, expr, pipe, level, pipes)?;
        }
        ast::Statement::Goto(label) => {
            writeln!(file, "{indent}{pipe} Goto [{}]", label.symbol)?;
        }
        ast::Statement::Labeled { name, body: stmt } => {
            writeln!(file, "{indent}{pipe} Label [{}]", name.symbol)?;
            print_statement(file, stmt, "╰──", level + 1, pipes)?;
        }
        ast::Statement::Case {
            value, body: stmt, ..
        } => {
            writeln!(file, "{indent}{pipe} Case")?;
            print_expression(
                file,
                value,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            print_statement(file, stmt, "╰──", level + 1, pipes)?;
        }
        ast::Statement::Default { body: stmt, .. } => {
            print_named_statement(file, "Default", stmt, "╰──", level, pipes)?;
        }
        ast::Statement::Compound(block) => {
            writeln!(file, "{indent}{pipe} Block")?;
            for (i, block_item) in block.items.iter().enumerate() {
                if i < block.items.len() - 1 {
                    print_block_item(
                        file,
                        block_item,
                        "├──",
                        level + 1,
                        &[pipes, &[level + 1]].concat(),
                    )?
                } else {
                    print_block_item(file, block_item, "╰──", level + 1, pipes)?
                };
            }
        }
        ast::Statement::While { cond, body, .. } => {
            writeln!(file, "{indent}{pipe} While")?;
            print_named_expression(
                file,
                "Condition",
                cond,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            print_statement(file, body, "╰──", level + 1, pipes)?;
        }
        ast::Statement::DoWhile { cond, body, .. } => {
            writeln!(file, "{indent}{pipe} DoWhile")?;
            print_statement(
                file,
                body,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
            print_named_expression(file, "Condition", cond, "╰──", level + 1, pipes)?;
        }
        ast::Statement::For {
            init,
            cond,
            post,
            body,
            ..
        } => {
            writeln!(file, "{indent}{pipe} For")?;
            match init {
                ast::ForInit::Decl(d) => {
                    print_named_var_declaration(
                        file,
                        "Initial",
                        d,
                        "├──",
                        level + 1,
                        &[pipes, &[level + 1]].concat(),
                    )?;
                }
                ast::ForInit::Expr(e) => {
                    print_named_expression(
                        file,
                        "Initial",
                        e,
                        "├──",
                        level + 1,
                        &[pipes, &[level + 1]].concat(),
                    )?;
                }
                ast::ForInit::None => {}
            }
            if let Some(cond) = cond {
                print_named_expression(
                    file,
                    "Condition",
                    cond,
                    "├──",
                    level + 1,
                    &[pipes, &[level + 1]].concat(),
                )?;
            }
            if let Some(post) = post {
                print_named_expression(
                    file,
                    "Post",
                    post,
                    "├──",
                    level + 1,
                    &[pipes, &[level + 1]].concat(),
                )?;
            }
            print_statement(file, body, "╰──", level + 1, pipes)?;
        }
        ast::Statement::Break(..) => {
            writeln!(file, "{indent}{pipe} Break")?;
        }
        ast::Statement::Continue(..) => {
            writeln!(file, "{indent}{pipe} Continue")?;
        }
        ast::Statement::Null => {
            writeln!(file, "{indent}{pipe} Empty")?;
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
    let indent = make_indent(level, pipes);
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
            print_expression(
                file,
                left,
                "├──",
                level + 1,
                &[pipes, &[level + 1]].concat(),
            )?;
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
        ast::Expression::FunctionCall { name, args } => {
            writeln!(file, "{indent}{pipe} FunctionCall [{}]", name.symbol)?;
            for (i, arg) in args.iter().enumerate() {
                if i < args.len() - 1 {
                    print_expression(file, arg, "├──", level + 1, &[pipes, &[level + 1]].concat())?;
                } else {
                    print_expression(file, arg, "╰──", level + 1, pipes)?;
                }
            }
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
