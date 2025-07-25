use crate::ast::{
    AssignOp, BinaryOp, BlockItem, Constant, Declaration, Expression, ForInit, FunctionDeclaration,
    Identifier, Initializer, Node, PostfixOp, Program, Statement, StorageClass, Type, UnaryOp,
    VarDeclaration,
};
use std::io::Write;

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
    fn from_program(program: &Program) -> Self {
        Self::new(
            "Program",
            program
                .declarations
                .iter()
                .map(|f| Self::from_declaration(f)),
        )
    }
    fn from_declaration(declaration: &Declaration) -> PrettyAst {
        match declaration {
            Declaration::Var(d) => Self::from_var_declaration(d),
            Declaration::Function(d) => Self::from_function_declaration(d),
        }
    }
    fn from_function_declaration(function: &FunctionDeclaration) -> PrettyAst {
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
    fn from_var_declaration(declaration: &VarDeclaration) -> PrettyAst {
        let mut children = vec![
            Self::new("Name", vec![Self::from_identifier(&declaration.name)]),
            Self::new("Type", vec![Self::from_type(&declaration.ty)]),
        ];
        if let Some(init) = &declaration.init {
            children.push(Self::new("Initializer", vec![Self::from_initializer(init)]));
        }
        if let Some(s) = &declaration.storage_class {
            match s.as_ref() {
                StorageClass::Static => {
                    children.push(Self::new("Static", vec![]));
                }
                StorageClass::Extern => {
                    children.push(Self::new("Extern", vec![]));
                }
            }
        }
        Self::new("VarDeclaration", children)
    }
    fn from_initializer(initializer: &Node<Initializer>) -> PrettyAst {
        match initializer.as_ref() {
            Initializer::Single(expr) => Self::from_expression(expr),
            Initializer::Compound(elements) => {
                Self::new("Compound", elements.iter().map(Self::from_initializer))
            }
        }
    }
    fn storage_class(storage: &Option<Node<StorageClass>>) -> &str {
        if let Some(s) = storage {
            match s.as_ref() {
                StorageClass::Static => "static ",
                StorageClass::Extern => "extern ",
            }
        } else {
            ""
        }
    }
    fn from_statement(statement: &Statement) -> PrettyAst {
        match statement {
            Statement::Return(expr) => {
                let children = if let Some(expr) = expr {
                    vec![Self::from_expression(expr)]
                } else {
                    vec![]
                };
                Self::new("Return", children)
            }
            Statement::If {
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
            Statement::Switch { expr, body, .. } => {
                let children = vec![
                    Self::new("Expression", vec![Self::from_expression(expr)]),
                    Self::from_statement(body),
                ];
                Self::new("Switch", children)
            }
            Statement::Expression(expr) => Self::from_expression(expr),
            Statement::Labeled { name, body } => Self::new(
                format!("Label [{}]", name.symbol),
                vec![Self::from_statement(body)],
            ),
            Statement::Default { body, .. } => {
                Self::new("Default", vec![Self::from_statement(body)])
            }
            Statement::Case { value, body, .. } => {
                if let Expression::Constant(value) = value.as_ref() {
                    let value = match value {
                        Constant::UChar(v) => *v as i64,
                        Constant::Char(v) => *v as i64,
                        Constant::Int(v) => *v as i64,
                        Constant::Long(v) => *v,
                        Constant::UInt(v) => *v as i64,
                        Constant::ULong(v) => *v as i64,
                        Constant::Double(v) => *v as i64,
                    };
                    Self::new(format!("Case [{value}]"), vec![Self::from_statement(body)])
                } else {
                    let children = vec![
                        Self::new("Value", vec![Self::from_expression(value)]),
                        Self::from_statement(body),
                    ];
                    Self::new("Case [invalid]", children)
                }
            }
            Statement::Goto(identifier) => {
                Self::new(format!("Goto [{}]", identifier.symbol), vec![])
            }
            Statement::Compound(block) => {
                Self::new("Block", block.items.iter().map(Self::from_block_item))
            }
            Statement::While { cond, body, .. } => {
                let children = vec![
                    Self::new("Condition", vec![Self::from_expression(cond)]),
                    Self::new("Body", vec![Self::from_statement(body)]),
                ];
                Self::new("While", children)
            }
            Statement::DoWhile { cond, body, .. } => {
                let children = vec![
                    Self::new("Body", vec![Self::from_statement(body)]),
                    Self::new("Condition", vec![Self::from_expression(cond)]),
                ];
                Self::new("DoWhile", children)
            }
            Statement::For {
                init,
                cond,
                post,
                body,
                ..
            } => {
                let mut children = Vec::new();
                match init {
                    ForInit::None => {}
                    ForInit::Decl(decl) => {
                        children.push(Self::new("Init", vec![Self::from_var_declaration(decl)]));
                    }
                    ForInit::Expr(expr) => {
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
            Statement::Break(_) => Self::new("Break", vec![]),
            Statement::Continue(_) => Self::new("Continue", vec![]),
            Statement::Null => Self::new("Empty", vec![]),
        }
    }
    fn from_expression(expression: &Node<Expression>) -> PrettyAst {
        let node_id = expression.id;
        match expression.as_ref() {
            Expression::Constant(value) => Self::from_constant(value),
            Expression::String(symbol) => Self::new(format!("<{node_id}> \"{symbol}\""), vec![]),
            Expression::Var(name) => Self::new(format!("<{node_id}> Var [{name}]"), vec![]),
            Expression::Unary { op, expr } => Self::new(
                format!("<{node_id}> Unary [{}]", Self::unary_op(op)),
                vec![Self::from_expression(expr)],
            ),
            Expression::Postfix { op, expr } => Self::new(
                format!("<{node_id}> Postfix [{}]", Self::postfix_op(op)),
                vec![Self::from_expression(expr)],
            ),
            Expression::Binary { op, left, right } => Self::new(
                format!("<{node_id}>  [{}]", Self::binary_op(op)),
                vec![Self::from_expression(left), Self::from_expression(right)],
            ),
            Expression::Assignment { op, left, right } => Self::new(
                format!("<{node_id}> Assign [{}]", Self::assign_op(op)),
                vec![Self::from_expression(left), Self::from_expression(right)],
            ),
            Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                let children = vec![
                    Self::from_expression(cond),
                    Self::new("Then", vec![Self::from_expression(then_expr)]),
                    Self::new("Else", vec![Self::from_expression(else_expr)]),
                ];
                Self::new("<{node_id}> Conditional [?]", children)
            }
            Expression::FunctionCall { name, args } => Self::new(
                format!("<{node_id}> FunctionCall [{}]", name.symbol),
                args.iter().map(Self::from_expression),
            ),
            Expression::Cast { target, expr } => Self::new(
                format!("<{node_id}> Cast"),
                vec![
                    Self::new("Target", vec![Self::from_type(target)]),
                    Self::new("Expression", vec![Self::from_expression(expr)]),
                ],
            ),
            Expression::AddressOf(expr) => Self::new(
                format!("<{node_id}> AddressOf"),
                vec![Self::from_expression(expr)],
            ),
            Expression::Dereference(expr) => Self::new(
                format!("<{node_id}> Dereference"),
                vec![Self::from_expression(expr)],
            ),
            Expression::Subscript(expr, index) => Self::new(
                format!("<{node_id}> Subscript"),
                vec![Self::from_expression(expr), Self::from_expression(index)],
            ),
            Expression::SizeOfType(ty) => {
                Self::new(format!("<{node_id}> SizeOfType"), vec![Self::from_type(ty)])
            }
            Expression::SizeOfExpr(ty) => Self::new(
                format!("<{node_id}> SizeOfExpr"),
                vec![Self::from_expression(ty)],
            ),
        }
    }
    fn from_block_item(item: &BlockItem) -> PrettyAst {
        match item {
            BlockItem::Stmt(s) => Self::from_statement(s),
            BlockItem::Decl(d) => Self::from_declaration(d),
        }
    }
    fn from_identifier(identifier: &Identifier) -> PrettyAst {
        Self::new(&identifier.symbol, vec![])
    }

    fn from_constant(constant: &Constant) -> PrettyAst {
        match constant {
            Constant::Int(v) => Self::new(format!("Constant Int [{}]", *v), vec![]),
            Constant::Long(v) => Self::new(format!("Constant Long [{}]", *v), vec![]),
            Constant::UInt(v) => Self::new(format!("Constant UInt [{}]", *v), vec![]),
            Constant::ULong(v) => Self::new(format!("Constant ULong [{}]", *v), vec![]),
            Constant::Double(v) => Self::new(format!("Constant Double [{:+e}]", *v), vec![]),
            Constant::Char(v) => Self::new(format!("Constant Char [{}]", *v), vec![]),
            Constant::UChar(v) => Self::new(format!("Constant UChar [{}]", *v), vec![]),
        }
    }

    fn from_type(ty: &Type) -> PrettyAst {
        match ty {
            Type::Int => Self::new("Int", vec![]),
            Type::Long => Self::new("Long", vec![]),
            Type::Char => Self::new("Char", vec![]),
            Type::SChar => Self::new("Signed Char", vec![]),
            Type::UChar => Self::new("Unsigned Char", vec![]),
            Type::ULong => Self::new("Unsigned Long", vec![]),
            Type::UInt => Self::new("Unsigned Int", vec![]),
            Type::Double => Self::new("Double", vec![]),
            Type::Void => Self::new("Void", vec![]),
            Type::Function(f) => Self::new(
                "FunctionType",
                vec![
                    Self::new("Return", vec![Self::from_type(&f.ret)]),
                    Self::new("Params", f.params.iter().map(Self::from_type)),
                ],
            ),
            Type::Pointer(t) => Self::new("Pointer", vec![Self::from_type(t.as_ref())]),
            Type::Array(t, size) => Self::new(
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
    ) -> Result<(), std::io::Error> {
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

    fn binary_op(op: &BinaryOp) -> &str {
        use BinaryOp::*;
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

    fn assign_op(op: &AssignOp) -> &str {
        use AssignOp::*;
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

    fn unary_op(op: &UnaryOp) -> &str {
        use UnaryOp::*;
        match op {
            Complement => "~",
            Negate => "-",
            Not => "!",
            Increment => "++",
            Decrement => "--",
        }
    }

    fn postfix_op(op: &PostfixOp) -> &str {
        use PostfixOp::*;
        match op {
            Increment => "++",
            Decrement => "--",
        }
    }
}

pub fn dump(program: &Program) -> Result<String, std::io::Error> {
    let mut buffer = Vec::new();
    PrettyAst::from_program(program).write(&mut buffer, "", 0, &[])?;
    Ok(String::from_utf8(buffer).unwrap().trim().into())
}
