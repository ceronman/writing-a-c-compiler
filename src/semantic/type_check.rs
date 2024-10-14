use crate::ast::{
    BinaryOp, Block, BlockItem, Constant, Declaration, Expression, ForInit, FunctionDeclaration,
    FunctionType, InnerRef, Node, NodeId, Program, Statement, StorageClass, Type, UnaryOp,
    VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::semantic::{
    Attributes, InitialValue, SemanticData, StaticInit, SwitchCases, SymbolData,
};
use std::collections::{BTreeMap, HashMap, VecDeque};

impl SymbolData {
    fn local(ty: Type) -> Self {
        SymbolData {
            ty,
            attrs: Attributes::Local,
        }
    }
    fn global(ty: Type) -> Self {
        SymbolData {
            ty,
            attrs: Attributes::Static {
                initial_value: InitialValue::NoInitializer,
                global: true,
            },
        }
    }
}

#[derive(Default)]
struct TypeChecker {
    symbols: BTreeMap<String, SymbolData>,
    expression_types: HashMap<NodeId, Type>,
    implicit_casts: HashMap<NodeId, Type>,
    switch_stack: VecDeque<SwitchCases>,
    switch_cases: HashMap<NodeId, SwitchCases>,
}

impl TypeChecker {
    fn check(&mut self, program: &Program) -> Result<()> {
        for decl in &program.declarations {
            match decl.as_ref() {
                Declaration::Var(v) => self.check_file_var_declaration(v)?,
                Declaration::Function(f) => self.check_function_declaration(f, true)?,
            }
        }
        Ok(())
    }

    fn check_local_var_declaration(&mut self, decl: &VarDeclaration) -> Result<()> {
        let name = decl.name.symbol.clone();
        if let Some(StorageClass::Extern) = decl.storage_class.inner_ref() {
            if let Some(init) = &decl.init {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: "Initializers are not allowed in local extern variable declarations"
                        .into(),
                    span: init.span,
                });
            }
            if let Some(data) = self.symbols.get(&name) {
                if data.ty != decl.ty {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: format!("Name '{name}' was already declared"),
                        span: decl.name.span,
                    });
                }
            } else {
                self.symbols.insert(
                    decl.name.symbol.clone(),
                    SymbolData::global(decl.ty.clone()),
                );
            }
        } else if let Some(StorageClass::Static) = decl.storage_class.inner_ref() {
            let initial_value = if let Some(init) = &decl.init {
                if let Expression::Constant(c) = init.as_ref() {
                    // TODO: this is duplicated in multiple places
                    InitialValue::Initial(match c {
                        Constant::Int(v) => StaticInit::Int(*v),
                        Constant::Long(v) => StaticInit::Long(*v),
                    })
                } else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: format!("Non-constant initializer on local static variable '{name}'"),
                        span: init.span,
                    });
                }
            } else {
                match decl.ty {
                    Type::Int => InitialValue::Initial(StaticInit::Int(0)),
                    Type::Long => InitialValue::Initial(StaticInit::Long(0)),
                    Type::Function(_) => unreachable!(),
                }
            };
            if let Some(data) = self.symbols.get(&name) {
                if data.ty != decl.ty {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: format!("Name '{name}' was previously declared with a different type"),
                        span: decl.name.span,
                    });
                }
            } else {
                self.symbols.insert(
                    decl.name.symbol.clone(),
                    SymbolData {
                        ty: decl.ty.clone(),
                        attrs: Attributes::Static {
                            initial_value,
                            global: false,
                        },
                    },
                );
            }
        } else {
            self.symbols
                .insert(decl.name.symbol.clone(), SymbolData::local(decl.ty.clone()));
            if let Some(init) = &decl.init {
                let init_ty = self.check_expression(init)?;
                self.cast_if_needed(init, &init_ty, &decl.ty);
            }
        }
        Ok(())
    }

    fn check_file_var_declaration(&mut self, decl: &VarDeclaration) -> Result<()> {
        let mut initial_value = if let Some(init) = &decl.init {
            if let Expression::Constant(c) = init.as_ref() {
                InitialValue::Initial(match c {
                    Constant::Int(v) => StaticInit::Int(*v),
                    Constant::Long(v) => StaticInit::Long(*v),
                })
            } else {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: "Non-constant initializer".into(),
                    span: init.span,
                });
            }
        } else if let Some(StorageClass::Extern) = decl.storage_class.inner_ref() {
            InitialValue::NoInitializer
        } else {
            InitialValue::Tentative
        };
        let mut global = !matches!(decl.storage_class.inner_ref(), Some(StorageClass::Static));
        let name = decl.name.symbol.clone();
        if let Some(data) = self.symbols.get(&name) {
            if data.ty != decl.ty {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Variable '{name}' is already declared with a different type"),
                    span: decl.name.span,
                });
            }
            let Attributes::Static {
                initial_value: old_initial,
                global: old_global,
            } = &data.attrs
            else {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Variable '{name}' does not have variable attributes"),
                    span: decl.name.span,
                });
            };
            if matches!(decl.storage_class.inner_ref(), Some(StorageClass::Extern)) {
                global = *old_global
            } else if *old_global != global {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Variable '{name}' has conflicting linkage"),
                    span: decl.name.span,
                });
            }
            initial_value = match (old_initial, initial_value) {
                (InitialValue::Initial(_), InitialValue::Initial(_)) => {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: format!("Variable '{name}' has conflicting definitions"),
                        span: decl.name.span,
                    });
                }
                (InitialValue::Initial(_), _) => *old_initial,
                (
                    InitialValue::Tentative,
                    InitialValue::NoInitializer | InitialValue::Tentative,
                ) => InitialValue::Tentative,
                _ => initial_value,
            };
        }
        let data = SymbolData {
            ty: decl.ty.clone(),
            attrs: Attributes::Static {
                initial_value,
                global,
            },
        };
        self.symbols.insert(name, data);
        Ok(())
    }

    fn check_function_declaration(
        &mut self,
        decl: &FunctionDeclaration,
        top_level: bool,
    ) -> Result<()> {
        let name = decl.name.symbol.clone();
        let this_ty = Type::Function(decl.ty.clone());
        let has_body = decl.body.is_some();
        let mut already_defined = false;
        let is_static = matches!(decl.storage_class.inner_ref(), Some(StorageClass::Static));

        if !top_level && is_static {
            return Err(CompilerError {
                kind: ErrorKind::Type,
                msg: "Block scoped function declarations can't be static".into(),
                span: decl.storage_class.as_ref().unwrap().span,
            });
        }

        let mut is_global = !is_static;

        if let Some(data) = self.symbols.get(&name) {
            if data.ty != this_ty {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Conflicting declaration types for '{name}'"),
                    span: decl.name.span,
                });
            };
            let Attributes::Function { defined, global } = data.attrs else {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Function '{name}' does not have function attributes"),
                    span: decl.name.span,
                });
            };
            already_defined = defined;
            if already_defined && has_body {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Function '{name}' is defined more than once"),
                    span: decl.name.span,
                });
            }
            if global && is_static {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Function '{name}' was previously declared as non-static"),
                    span: decl.name.span,
                });
            }
            is_global = global
        }
        let data = SymbolData {
            ty: this_ty,
            attrs: Attributes::Function {
                defined: already_defined || has_body,
                global: is_global,
            },
        };
        self.symbols.insert(name, data);
        if let Some(body) = &decl.body {
            if !top_level {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: "Nested function definitions are not allowed ".to_owned(),
                    span: decl.name.span,
                });
            }
            for (param_name, param_ty) in decl.params.iter().zip(decl.ty.params.iter()) {
                self.symbols.insert(
                    param_name.symbol.clone(),
                    SymbolData::local(param_ty.clone()),
                );
            }
            self.check_block(&decl.ty, body)?;
        }
        Ok(())
    }

    fn check_block(&mut self, function: &FunctionType, block: &Block) -> Result<()> {
        for item in &block.items {
            match item {
                BlockItem::Stmt(stmt) => self.check_statement(function, stmt)?,
                BlockItem::Decl(decl) => self.check_declaration(decl)?,
            }
        }
        Ok(())
    }

    fn check_statement(&mut self, function: &FunctionType, stmt: &Node<Statement>) -> Result<()> {
        match stmt.as_ref() {
            Statement::Return(expr) => {
                let expr_ty = self.check_expression(expr)?;
                self.cast_if_needed(expr, &expr_ty, &function.ret);
            }
            Statement::Expression(expr) => {
                self.check_expression(expr)?;
            }
            Statement::If {
                cond,
                then_stmt,
                else_stmt,
            } => {
                self.check_expression(cond)?;
                self.check_statement(function, then_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.check_statement(function, else_stmt)?;
                }
            }
            Statement::Switch { expr, body, .. } => {
                let expr_ty = self.check_expression(expr)?;
                self.switch_stack.push_front(SwitchCases {
                    expr_ty,
                    values: vec![],
                    default: None,
                });
                self.check_statement(function, body)?;
                self.switch_cases
                    .insert(expr.id, self.switch_stack.pop_front().unwrap());
            }
            Statement::Case { value, body, label } => {
                let switch_cases = self.switch_stack.front_mut().expect("Case without switch");
                let Expression::Constant(constant) = value.as_ref() else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "case label does not reduce to an integer constant".to_owned(),
                        span: value.span,
                    });
                };
                let case_value = match (constant, &switch_cases.expr_ty) {
                    (Constant::Int(v), Type::Int) => Constant::Int(*v),
                    (Constant::Long(v), Type::Long) => Constant::Long(*v),
                    (Constant::Int(v), Type::Long) => Constant::Long(*v as i64),
                    (Constant::Long(v), Type::Int) => Constant::Int(*v as i32),
                    _ => unreachable!(),
                };

                if switch_cases.values.iter().any(|(v, _)| *v == case_value) {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "duplicate case value".to_owned(),
                        span: value.span,
                    });
                }
                switch_cases.values.push((case_value, label.clone()));
                self.check_statement(function, body)?;
            }
            Statement::Default { label, body } => {
                let switch_cases = self.switch_stack.front_mut().expect("Case without switch");
                if switch_cases.default.is_some() {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "multiple default labels in one switch".to_owned(),
                        span: stmt.span,
                    });
                }
                switch_cases.default = Some(label.clone());
                self.check_statement(function, body)?
            }
            Statement::While {
                cond: expr, body, ..
            }
            | Statement::DoWhile {
                cond: expr, body, ..
            } => {
                self.check_expression(expr)?;
                self.check_statement(function, body)?;
            }
            Statement::Labeled { body, .. } => self.check_statement(function, body)?,
            Statement::Compound(block) => self.check_block(function, block)?,
            Statement::For {
                init,
                cond,
                post,
                body,
                ..
            } => {
                match init {
                    ForInit::Decl(d) => {
                        if let Some(storage) = &d.storage_class {
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Declarations inside for loops can't have storage class"
                                    .into(),
                                span: storage.span,
                            });
                        }
                        self.check_local_var_declaration(d)?
                    }
                    ForInit::Expr(e) => {
                        self.check_expression(e)?;
                    }
                    ForInit::None => {}
                }
                if let Some(cond) = cond {
                    self.check_expression(cond)?;
                }
                if let Some(post) = post {
                    self.check_expression(post)?;
                }
                self.check_statement(function, body)?;
            }
            Statement::Break(_) | Statement::Continue(_) | Statement::Goto(_) | Statement::Null => {
            }
        }
        Ok(())
    }

    fn check_declaration(&mut self, decl: &Node<Declaration>) -> Result<()> {
        match decl.as_ref() {
            Declaration::Var(d) => self.check_local_var_declaration(d),
            Declaration::Function(d) => self.check_function_declaration(d, false),
        }
    }

    fn check_expression(&mut self, expr: &Node<Expression>) -> Result<Type> {
        let ty = match expr.as_ref() {
            Expression::Constant(c) => match c {
                Constant::Int(_) => Type::Int,
                Constant::Long(_) => Type::Long,
            },
            Expression::Var(name) => {
                let Some(data) = self.symbols.get(name) else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Unknown type of expression".to_string(),
                        span: expr.span,
                    });
                };
                if let Type::Function(_) = data.ty {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Function used as variable".to_string(),
                        span: expr.span,
                    });
                };
                data.ty.clone()
            }
            Expression::Unary { op, expr } => {
                let operand_ty = self.check_expression(expr)?;
                match op.as_ref() {
                    UnaryOp::Not => Type::Int,
                    _ => operand_ty,
                }
            }
            Expression::Postfix { expr, .. } => self.check_expression(expr)?,
            Expression::Binary { left, right, op } => {
                let left_ty = self.check_expression(left)?;
                let right_ty = self.check_expression(right)?;

                match op.as_ref() {
                    BinaryOp::And | BinaryOp::Or => Type::Int,
                    BinaryOp::ShiftRight | BinaryOp::ShiftLeft => {
                        left_ty // TODO: Investigate this properly!
                    }
                    _ => {
                        let common = Self::common_type(&left_ty, &right_ty);
                        self.cast_if_needed(left, &left_ty, &common);
                        self.cast_if_needed(right, &right_ty, &common);
                        match op.as_ref() {
                            BinaryOp::Equal
                            | BinaryOp::NotEqual
                            | BinaryOp::LessThan
                            | BinaryOp::LessOrEqualThan
                            | BinaryOp::GreaterThan
                            | BinaryOp::GreaterOrEqualThan => Type::Int,
                            _ => common,
                        }
                    }
                }
            }

            Expression::Assignment { left, right, .. } => {
                let left_ty = self.check_expression(left)?;
                let right_ty = self.check_expression(right)?;
                self.cast_if_needed(right, &right_ty, &left_ty);
                left_ty
            }

            Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                self.check_expression(cond)?;
                let then_ty = self.check_expression(then_expr)?;
                let else_ty = self.check_expression(else_expr)?;
                let common = Self::common_type(&then_ty, &else_ty);
                self.cast_if_needed(then_expr, &then_ty, &common);
                self.cast_if_needed(else_expr, &else_ty, &common);
                common
            }

            Expression::FunctionCall { name, args } => {
                let symbol = name.symbol.clone();
                let Some(data) = self.symbols.get(&symbol) else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Unknown type of expression".to_string(),
                        span: expr.span,
                    });
                };
                let Type::Function(function_ty) = data.ty.clone() else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Variable used as function name".to_string(),
                        span: expr.span,
                    });
                };
                if function_ty.params.len() != args.len() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Function called with the wrong number of arguments".to_string(),
                        span: expr.span,
                    });
                }
                for (arg, param_ty) in args.iter().zip(&function_ty.params) {
                    let arg_ty = self.check_expression(arg)?;
                    self.cast_if_needed(arg, &arg_ty, param_ty);
                }
                *function_ty.ret.clone()
            }
            Expression::Cast { target, expr } => {
                self.check_expression(expr)?;
                *target.data.clone()
            }
        };
        self.expression_types.insert(expr.id, ty.clone());
        Ok(ty)
    }

    fn cast_if_needed(&mut self, expr: &Node<Expression>, ty: &Type, expected: &Type) -> Type {
        if ty == expected {
            expected.clone()
        } else {
            self.implicit_casts.insert(expr.id, expected.clone());
            expected.clone()
        }
    }

    fn common_type(ty1: &Type, ty2: &Type) -> Type {
        if ty1 == ty2 {
            ty1.clone()
        } else {
            Type::Long
        }
    }
}

pub fn check(program: &Node<Program>) -> Result<SemanticData> {
    let mut type_checker = TypeChecker::default();
    type_checker.check(program)?;
    Ok(SemanticData {
        symbols: type_checker.symbols,
        expression_types: type_checker.expression_types,
        implicit_casts: type_checker.implicit_casts,
        switch_cases: type_checker.switch_cases,
    })
}
