use crate::ast;
use crate::parser;
use crate::semantic;
use crate::tacky;

use crate::semantic::StaticInit;
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
    let (ast, semantic_data) = semantic::validate(ast).unwrap();
    let tacky = tacky::emit(&ast, semantic_data);
    pp_tacky(&tacky).unwrap().trim().to_owned()
}

pub fn pretty_print_ast(program: &ast::Program) -> Result<String> {
    let mut buffer = Vec::new();
    PrettyAst::from_program(program).write(&mut buffer, "", 0, &[])?;
    Ok(String::from_utf8(buffer)?.trim().into())
}

pub fn pp_tacky(program: &tacky::Program) -> Result<String> {
    let mut buffer = Vec::new();
    let file = &mut buffer;
    for top_level in &program.top_level {
        match top_level {
            tacky::TopLevel::Function(f) => pp_function(file, f)?,
            tacky::TopLevel::Variable(v) => pp_static_variable(file, v)?,
        }
    }
    Ok(String::from_utf8(buffer)?)
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
    match variable.init {
        StaticInit::Int(v) => writeln!(file, "{v}")?,
        StaticInit::Long(v) => writeln!(file, "{v}L")?,
        StaticInit::UInt(v) => writeln!(file, "{v}U")?,
        StaticInit::ULong(v) => writeln!(file, "{v}UL")?,
        StaticInit::Double(v) => writeln!(file, "{v}D")?,
        StaticInit::ZeroInit(_) => todo!(),
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
                write!(file, " = get_address ")?;
                pp_val(file, src)?;
            }
            tacky::Instruction::Load { ptr, dst } => {
                write!(file, "{indent}")?;
                pp_val(file, dst)?;
                write!(file, " = load ")?;
                pp_val(file, ptr)?;
            }
            tacky::Instruction::Store { src, ptr } => {
                write!(file, "{indent}")?;
                pp_val(file, ptr)?;
                write!(file, " = store ")?;
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
        tacky::Val::Constant(ast::Constant::Int(value)) => write!(file, "{value}")?,
        tacky::Val::Constant(ast::Constant::Long(value)) => write!(file, "{value}L")?,
        tacky::Val::Constant(ast::Constant::UInt(value)) => write!(file, "{value}U")?,
        tacky::Val::Constant(ast::Constant::ULong(value)) => write!(file, "{value}UL")?,
        tacky::Val::Constant(ast::Constant::Double(value)) => write!(file, "{value}D")?,
        tacky::Val::Var(name) => write!(file, "{name}")?,
    }
    Ok(())
}

fn pp_type(file: &mut impl Write, ty: &ast::Type) -> Result<()> {
    match ty {
        ast::Type::Int => write!(file, "Int"),
        ast::Type::Long => write!(file, "Long"),
        ast::Type::UInt => write!(file, "Unsigned Int"),
        ast::Type::ULong => write!(file, "Unsigned Long"),
        ast::Type::Function(_) => write!(file, "Function(...)"),
        ast::Type::Double => write!(file, "Double"),
        ast::Type::Pointer(referenced) => {
            write!(file, "Pointer(")?;
            pp_type(file, referenced.as_ref())?;
            write!(file, ")")
        }
        ast::Type::Array(inner, size) => {
            write!(file, "Array(")?;
            write!(file, "{size},")?;
            pp_type(file, inner.as_ref())?;
            write!(file, ")")
        }
    }?;
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
            let start = error.span.0 - offset;
            let start = if start > 2 { start - 2 } else { 0 };
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

// TODO: Add support for including node ids (useful for debugging semantic data)
struct PrettyAst {
    label: String,
    children: Vec<PrettyAst>,
}

impl PrettyAst {
    fn new(label: impl Into<String>, children: impl IntoIterator<Item = PrettyAst>) -> Self {
        Self {
            label: label.into(),
            children: children.into_iter().collect(),
        }
    }
    fn from_program(program: &ast::Program) -> Self {
        Self::new(
            "Program",
            program
                .declarations
                .iter()
                .map(|f| Self::from_declaration(f)),
        )
    }
    fn from_declaration(declaration: &ast::Declaration) -> PrettyAst {
        match declaration {
            ast::Declaration::Var(d) => Self::from_var_declaration(d),
            ast::Declaration::Function(d) => Self::from_function_declaration(d),
        }
    }
    fn from_function_declaration(function: &ast::FunctionDeclaration) -> PrettyAst {
        let mut children = Vec::new();

        if !function.params.is_empty() {
            children.push(Self::new(
                "Parameters",
                function
                    .params
                    .iter()
                    .zip(function.ty.params.iter())
                    .map(|(name, ty)| {
                        Self::new(
                            "Param",
                            vec![
                                Self::new("Name", vec![Self::from_identifier(name)]),
                                Self::new("Type", vec![Self::from_type(ty)]),
                            ],
                        )
                    }),
            ))
        }
        if let Some(body) = &function.body {
            children.push(Self::new(
                "Body",
                body.items.iter().map(Self::from_block_item),
            ))
        }
        Self::new(
            format!(
                "Function [{}{}]",
                Self::storage_class(&function.storage_class),
                &function.name.symbol
            ),
            children,
        )
    }
    fn from_var_declaration(declaration: &ast::VarDeclaration) -> PrettyAst {
        let mut children = vec![
            Self::new("Name", vec![Self::from_identifier(&declaration.name)]),
            Self::new("Type", vec![Self::from_type(&declaration.ty)]),
        ];
        if let Some(init) = &declaration.init {
            children.push(Self::new("Initializer", vec![Self::from_initializer(init)]));
        }
        if let Some(s) = &declaration.storage_class {
            match s.as_ref() {
                ast::StorageClass::Static => {
                    children.push(Self::new("Static", vec![]));
                }
                ast::StorageClass::Extern => {
                    children.push(Self::new("Extern", vec![]));
                }
            }
        }
        Self::new("VarDeclaration", children)
    }
    fn from_initializer(initializer: &ast::Node<ast::Initializer>) -> PrettyAst {
        match initializer.as_ref() {
            ast::Initializer::Single(expr) => Self::from_expression(expr),
            ast::Initializer::Compound(elements) => {
                Self::new("Compound", elements.iter().map(Self::from_initializer))
            }
        }
    }
    fn storage_class(storage: &Option<ast::Node<ast::StorageClass>>) -> &str {
        if let Some(s) = storage {
            match s.as_ref() {
                ast::StorageClass::Static => "static ",
                ast::StorageClass::Extern => "extern ",
            }
        } else {
            ""
        }
    }
    fn from_statement(statement: &ast::Statement) -> PrettyAst {
        match statement {
            ast::Statement::Return(expr) => Self::new("Return", vec![Self::from_expression(expr)]),
            ast::Statement::If {
                cond,
                then_stmt,
                else_stmt,
            } => {
                let mut children = vec![
                    Self::new("Condition", vec![Self::from_expression(cond)]),
                    Self::new("Then", vec![Self::from_statement(then_stmt)]),
                ];
                if let Some(else_stmt) = else_stmt {
                    children.push(Self::new("Else", vec![Self::from_statement(else_stmt)]))
                }
                Self::new("If", children)
            }
            ast::Statement::Switch { expr, body, .. } => {
                let children = vec![
                    Self::new("Expression", vec![Self::from_expression(expr)]),
                    Self::from_statement(body),
                ];
                Self::new("Switch", children)
            }
            ast::Statement::Expression(expr) => Self::from_expression(expr),
            ast::Statement::Labeled { name, body } => Self::new(
                format!("Label [{}]", name.symbol),
                vec![Self::from_statement(body)],
            ),
            ast::Statement::Default { body, .. } => {
                Self::new("Default", vec![Self::from_statement(body)])
            }
            ast::Statement::Case { value, body, .. } => {
                if let ast::Expression::Constant(value) = value.as_ref() {
                    let value = match value {
                        ast::Constant::Int(v) => *v as i64,
                        ast::Constant::Long(v) => *v,
                        ast::Constant::UInt(v) => *v as i64,
                        ast::Constant::ULong(v) => *v as i64,
                        ast::Constant::Double(v) => *v as i64,
                    };
                    Self::new(
                        format!("Case [{}]", value),
                        vec![Self::from_statement(body)],
                    )
                } else {
                    let children = vec![
                        Self::new("Value", vec![Self::from_expression(value)]),
                        Self::from_statement(body),
                    ];
                    Self::new("Case [invalid]", children)
                }
            }
            ast::Statement::Goto(identifier) => {
                Self::new(format!("Goto [{}]", identifier.symbol), vec![])
            }
            ast::Statement::Compound(block) => {
                Self::new("Block", block.items.iter().map(Self::from_block_item))
            }
            ast::Statement::While { cond, body, .. } => {
                let children = vec![
                    Self::new("Condition", vec![Self::from_expression(cond)]),
                    Self::new("Body", vec![Self::from_statement(body)]),
                ];
                Self::new("While", children)
            }
            ast::Statement::DoWhile { cond, body, .. } => {
                let children = vec![
                    Self::new("Body", vec![Self::from_statement(body)]),
                    Self::new("Condition", vec![Self::from_expression(cond)]),
                ];
                Self::new("DoWhile", children)
            }
            ast::Statement::For {
                init,
                cond,
                post,
                body,
                ..
            } => {
                let mut children = Vec::new();
                match init {
                    ast::ForInit::None => {}
                    ast::ForInit::Decl(decl) => {
                        children.push(Self::new("Init", vec![Self::from_var_declaration(decl)]));
                    }
                    ast::ForInit::Expr(expr) => {
                        children.push(Self::new("Init", vec![Self::from_expression(expr)]));
                    }
                }
                if let Some(cond) = cond {
                    children.push(Self::new("Condition", vec![Self::from_expression(cond)]))
                }
                if let Some(post) = post {
                    children.push(Self::new("Condition", vec![Self::from_expression(post)]))
                }
                children.push(Self::from_statement(body));
                Self::new("For", children)
            }
            ast::Statement::Break(_) => Self::new("Break", vec![]),
            ast::Statement::Continue(_) => Self::new("Continue", vec![]),
            ast::Statement::Null => Self::new("Empty", vec![]),
        }
    }
    fn from_expression(expression: &ast::Node<ast::Expression>) -> PrettyAst {
        match expression.as_ref() {
            ast::Expression::Constant(value) => Self::from_constant(value),
            ast::Expression::Var(name) => Self::new(format!("Var [{name}]"), vec![]),
            ast::Expression::Unary { op, expr } => Self::new(
                format!("Unary [{}]", Self::unary_op(op)),
                vec![Self::from_expression(expr)],
            ),
            ast::Expression::Postfix { op, expr } => Self::new(
                format!("Postfix [{}]", Self::postfix_op(op)),
                vec![Self::from_expression(expr)],
            ),
            ast::Expression::Binary { op, left, right } => Self::new(
                format!("Binary [{}]", Self::binary_op(op)),
                vec![Self::from_expression(left), Self::from_expression(right)],
            ),
            ast::Expression::Assignment { op, left, right } => Self::new(
                format!("Assign [{}]", Self::assign_op(op)),
                vec![Self::from_expression(left), Self::from_expression(right)],
            ),
            ast::Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                let children = vec![
                    Self::from_expression(cond),
                    Self::new("Then", vec![Self::from_expression(then_expr)]),
                    Self::new("Else", vec![Self::from_expression(else_expr)]),
                ];
                Self::new("Conditional [?]", children)
            }
            ast::Expression::FunctionCall { name, args } => Self::new(
                format!("FunctionCall [{}]", name.symbol),
                args.iter().map(Self::from_expression),
            ),
            ast::Expression::Cast { target, expr } => Self::new(
                "Cast",
                vec![
                    Self::new("Target", vec![Self::from_type(target)]),
                    Self::new("Expression", vec![Self::from_expression(expr)]),
                ],
            ),
            ast::Expression::AddressOf(expr) => {
                Self::new("AddressOf", vec![Self::from_expression(expr)])
            }
            ast::Expression::Dereference(expr) => {
                Self::new("Dereference", vec![Self::from_expression(expr)])
            }
            ast::Expression::Subscript(expr, index) => Self::new(
                "Subscript",
                vec![Self::from_expression(expr), Self::from_expression(index)],
            ),
        }
    }
    fn from_block_item(item: &ast::BlockItem) -> PrettyAst {
        match item {
            ast::BlockItem::Stmt(s) => Self::from_statement(s),
            ast::BlockItem::Decl(d) => Self::from_declaration(d),
        }
    }
    fn from_identifier(identifier: &ast::Identifier) -> PrettyAst {
        Self::new(&identifier.symbol, vec![])
    }

    fn from_constant(constant: &ast::Constant) -> PrettyAst {
        match constant {
            ast::Constant::Int(v) => Self::new(format!("Constant Int [{}]", *v), vec![]),
            ast::Constant::Long(v) => Self::new(format!("Constant Long [{}]", *v), vec![]),
            ast::Constant::UInt(v) => Self::new(format!("Constant UInt [{}]", *v), vec![]),
            ast::Constant::ULong(v) => Self::new(format!("Constant ULong [{}]", *v), vec![]),
            ast::Constant::Double(v) => Self::new(format!("Constant Double [{:+e}]", *v), vec![]),
        }
    }

    fn from_type(ty: &ast::Type) -> PrettyAst {
        match ty {
            ast::Type::Int => Self::new("Int", vec![]),
            ast::Type::Long => Self::new("Long", vec![]),
            ast::Type::ULong => Self::new("Unsigned Long", vec![]),
            ast::Type::UInt => Self::new("Unsigned Int", vec![]),
            ast::Type::Double => Self::new("Double", vec![]),
            ast::Type::Function(f) => Self::new(
                "FunctionType",
                vec![
                    Self::new("Return", vec![Self::from_type(&f.ret)]),
                    Self::new("Params", f.params.iter().map(Self::from_type)),
                ],
            ),
            ast::Type::Pointer(t) => Self::new("Pointer", vec![Self::from_type(t.as_ref())]),
            ast::Type::Array(t, size) => Self::new(
                "Array",
                vec![
                    Self::new(format!("{size}"), vec![]),
                    Self::from_type(t.as_ref()),
                ],
            ),
        }
    }

    fn write(
        &self,
        file: &mut impl Write,
        pipe: &str,
        level: usize,
        pipes: &[usize],
    ) -> Result<()> {
        let indent = Self::make_indent(level, pipes);
        writeln!(file, "{indent}{pipe}{}", self.label)?;
        for (i, child) in self.children.iter().enumerate() {
            if i < self.children.len() - 1 {
                child.write(file, "├── ", level + 1, &[pipes, &[level + 1]].concat())?;
            } else {
                child.write(file, "╰── ", level + 1, pipes)?;
            }
        }
        Ok(())
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
}
