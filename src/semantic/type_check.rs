use crate::ast::{
    AssignOp, BinaryOp, Block, BlockItem, Constant, Declaration, Expression, ForInit,
    FunctionDeclaration, FunctionType, Initializer, InnerRef, Node, NodeId, Program, Statement,
    StorageClass, Type, UnaryOp, VarDeclaration,
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
    pointer_decays: HashMap<NodeId, Type>,
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
                InitialValue::Initial(self.check_static_initializer(init, &decl.ty)?)
            } else {
                InitialValue::single(StaticInit::ZeroInit(decl.ty.size()))
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
                self.check_initializer(init, &decl.ty)?;
            }
        }
        Ok(())
    }

    fn check_static_initializer(
        &mut self,
        init: &Node<Initializer>,
        target: &Type,
    ) -> Result<Vec<StaticInit>> {
        match init.as_ref() {
            Initializer::Single(expr) => {
                if let Expression::Constant(c) = expr.as_ref() {
                    let static_init = match (c, target) {
                        (c, Type::Int) if c.is_int() => StaticInit::Int(c.as_u64() as i32),
                        (c, Type::UInt) if c.is_int() => StaticInit::UInt(c.as_u64() as u32),
                        (c, Type::Long) if c.is_int() => StaticInit::Long(c.as_u64() as i64),
                        (c, Type::ULong) if c.is_int() => StaticInit::ULong(c.as_u64()),
                        (c, Type::Double) if c.is_int() => StaticInit::Double(c.as_u64() as f64),
                        (Constant::Double(value), Type::Int) => StaticInit::Int(*value as i32),
                        (Constant::Double(value), Type::UInt) => StaticInit::UInt(*value as u32),
                        (Constant::Double(value), Type::Long) => StaticInit::Long(*value as i64),
                        (Constant::Double(value), Type::ULong) => StaticInit::ULong(*value as u64),
                        (Constant::Double(value), Type::Double) => StaticInit::Double(*value),
                        (
                            Constant::Int(0)
                            | Constant::Long(0)
                            | Constant::UInt(0)
                            | Constant::ULong(0),
                            Type::Pointer(_),
                        ) => StaticInit::ULong(0),
                        _ => {
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Invalid type of static declaration".into(),
                                span: init.span,
                            });
                        }
                    };
                    Ok(vec![static_init])
                } else {
                    Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Non-constant initializer on local static variable".to_string(),
                        span: init.span,
                    })
                }
            }
            Initializer::Compound(initializers) => {
                let Type::Array(inner_ty, size) = target else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Cannot initialize a scalar value with a compound initializer".into(),
                        span: init.span,
                    });
                };
                let size = *size;
                if initializers.len() > size {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Wrong number of element in the initializer".into(),
                        span: init.span,
                    });
                }
                let mut static_inits = Vec::with_capacity(size);
                for initializer in initializers {
                    let inner_inits = self.check_static_initializer(initializer, inner_ty)?;
                    static_inits.extend(inner_inits);
                }
                let padding = (size - initializers.len()) * inner_ty.size();
                if padding > 0 {
                    static_inits.push(StaticInit::ZeroInit(padding));
                }
                Ok(static_inits)
            }
        }
    }

    fn check_initializer(&mut self, init: &Node<Initializer>, target: &Type) -> Result<Type> {
        match init.as_ref() {
            Initializer::Single(expr) => {
                let init_ty = self.check_and_convert_expr(expr)?;
                self.convert_by_assignment(expr, &init_ty, target)
            }
            Initializer::Compound(initializers) => {
                let Type::Array(inner_ty, size) = target else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Cannot initialize a scalar value with a compound initializer".into(),
                        span: init.span,
                    });
                };
                if initializers.len() > *size {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Wrong number of element in the initializer".into(),
                        span: init.span,
                    });
                }
                for inner_innit in initializers {
                    self.check_initializer(inner_innit, inner_ty)?;
                }
                Ok(target.clone())
            }
        }
    }

    fn check_file_var_declaration(&mut self, decl: &VarDeclaration) -> Result<()> {
        let mut initial_value = if let Some(init) = &decl.init {
            InitialValue::Initial(self.check_static_initializer(init, &decl.ty)?)
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
                (InitialValue::Initial(_), _) => old_initial.clone(),
                (
                    InitialValue::Tentative,
                    InitialValue::NoInitializer | InitialValue::Tentative,
                ) => InitialValue::Tentative,
                (_, initial_value) => initial_value,
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
        let mut function_ty = decl.ty.clone();

        if function_ty.ret.is_array() {
            // TODO: Make this error point to the span of the actual return type
            return Err(CompilerError {
                kind: ErrorKind::Type,
                msg: "A function cannot return array".into(),
                span: decl.name.span,
            });
        }

        // Replace array params with pointers
        for param in &mut function_ty.params {
            if let Type::Array(inner, _) = param {
                *param = Type::Pointer(inner.clone());
            }
        }

        let this_ty = Type::Function(function_ty.clone());
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
            for (param_name, param_ty) in decl.params.iter().zip(function_ty.params.iter()) {
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
                let expr_ty = self.check_and_convert_expr(expr)?;
                self.convert_by_assignment(expr, &expr_ty, &function.ret)?;
            }
            Statement::Expression(expr) => {
                self.check_and_convert_expr(expr)?;
            }
            Statement::If {
                cond,
                then_stmt,
                else_stmt,
            } => {
                self.check_and_convert_expr(cond)?;
                self.check_statement(function, then_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.check_statement(function, else_stmt)?;
                }
            }
            Statement::Switch { expr, body, .. } => {
                let expr_ty = self.check_and_convert_expr(expr)?;

                if !expr_ty.is_int() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Switch statement requires an integer expression".to_owned(),
                        span: expr.span,
                    });
                }

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
                let constant = match value.as_ref() {
                    Expression::Constant(constant) if constant.ty().is_int() => constant,
                    _ => {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "case label does not reduce to an integer constant".to_owned(),
                            span: value.span,
                        })
                    }
                };
                let case_constant = constant.as_u64();
                let case_value = match &switch_cases.expr_ty {
                    Type::Int => Constant::Int(case_constant as i32),
                    Type::UInt => Constant::UInt(case_constant as u32),
                    Type::Long => Constant::Long(case_constant as i64),
                    Type::ULong => Constant::ULong(case_constant),
                    Type::Double | Type::Function(_) => unreachable!(),
                    Type::Pointer(_) => Constant::ULong(case_constant),
                    Type::Array(_, _) => todo!(),
                };

                if switch_cases.values.iter().any(|(v, _)| *v == case_value) {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
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
                self.check_and_convert_expr(expr)?;
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
                        self.check_and_convert_expr(e)?;
                    }
                    ForInit::None => {}
                }
                if let Some(cond) = cond {
                    self.check_and_convert_expr(cond)?;
                }
                if let Some(post) = post {
                    self.check_and_convert_expr(post)?;
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

    fn check_and_convert_expr(&mut self, expr: &Node<Expression>) -> Result<Type> {
        let ty = self.check_expression(expr)?;
        if let Type::Array(inner, ..) = ty {
            // Pointer decay
            let pointer_ty = Type::Pointer(inner);
            self.pointer_decays.insert(expr.id, pointer_ty.clone());
            self.expression_types.insert(expr.id, pointer_ty.clone());
            Ok(pointer_ty)
        } else {
            Ok(ty)
        }
    }

    fn check_expression(&mut self, expr: &Node<Expression>) -> Result<Type> {
        let ty = match expr.as_ref() {
            Expression::Constant(c) => c.ty(),
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
                if let UnaryOp::Increment | UnaryOp::Decrement = op.as_ref() {
                    if operand_ty.is_array() {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Array is not assignable".to_owned(),
                            span: expr.span,
                        });
                    }
                    if !Self::is_lvalue(expr) {
                        return Err(CompilerError {
                            kind: ErrorKind::Resolve,
                            msg: "Expression is not assignable".to_owned(),
                            span: expr.span,
                        });
                    }
                }
                if matches!(op.as_ref(), UnaryOp::Complement) && !operand_ty.is_int() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Unary operator requires an integer type".to_string(),
                        span: expr.span,
                    });
                }
                if matches!(op.as_ref(), UnaryOp::Negate) && operand_ty.is_pointer() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Unary operator requires an non-pointer type".to_string(),
                        span: expr.span,
                    });
                }
                match op.as_ref() {
                    UnaryOp::Not => Type::Int,
                    _ => operand_ty,
                }
            }
            Expression::Postfix { expr, .. } => {
                let operand_ty = self.check_expression(expr)?;
                if operand_ty.is_array() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Array is not assignable".to_owned(),
                        span: expr.span,
                    });
                }
                if !Self::is_lvalue(expr) {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Expression is not assignable".to_owned(),
                        span: expr.span,
                    });
                }
                operand_ty
            }
            Expression::Binary { left, right, op } => {
                let left_ty = self.check_and_convert_expr(left)?;
                let right_ty = self.check_and_convert_expr(right)?;

                match op.as_ref() {
                    BinaryOp::Reminder
                    | BinaryOp::ShiftRight
                    | BinaryOp::ShiftLeft
                    | BinaryOp::BinOr
                    | BinaryOp::BinXor
                    | BinaryOp::BinAnd => {
                        if !left_ty.is_int() || !right_ty.is_int() {
                            let span = if !left_ty.is_int() {
                                left.span
                            } else {
                                right.span
                            };
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Operator requires integer operands".to_string(),
                                span,
                            });
                        }
                    }
                    BinaryOp::Multiply | BinaryOp::Divide if !left_ty.is_arithmetic() => {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Operator is invalid".to_string(),
                            span: left.span,
                        });
                    }
                    BinaryOp::Multiply | BinaryOp::Divide if !right_ty.is_arithmetic() => {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Operator is invalid".to_string(),
                            span: right.span,
                        });
                    }
                    BinaryOp::Add if left_ty.is_pointer() && !right_ty.is_int() => {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Operator is invalid".to_string(),
                            span: right.span,
                        });
                    }
                    BinaryOp::Add if right_ty.is_pointer() && !left_ty.is_int() => {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Operator is invalid".to_string(),
                            span: left.span,
                        });
                    }
                    BinaryOp::Subtract => match (&left_ty, &right_ty) {
                        (Type::Pointer(left_inner), Type::Pointer(right_inner)) => {
                            if left_inner != right_inner {
                                return Err(CompilerError {
                                    kind: ErrorKind::Type,
                                    msg: "Invalid pointer operator type".to_string(),
                                    span: right.span,
                                });
                            }
                        }
                        (left_ty, Type::Pointer(_)) if left_ty.is_int() => {
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Cannot substract a pointer from an int".to_string(),
                                span: right.span,
                            });
                        }
                        (Type::Pointer(_), _) if !right_ty.is_int() => {
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Operator is invalid".to_string(),
                                span: right.span,
                            });
                        }
                        _ => {}
                    },
                    BinaryOp::LessThan
                    | BinaryOp::LessOrEqualThan
                    | BinaryOp::GreaterThan
                    | BinaryOp::GreaterOrEqualThan
                        if !left_ty.is_arithmetic() || !left_ty.is_arithmetic() =>
                    {
                        let Type::Pointer(left_inner) = left_ty.clone() else {
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Operator is invalid".to_string(),
                                span: left.span,
                            });
                        };
                        let Type::Pointer(right_inner) = right_ty.clone() else {
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Operator is invalid".to_string(),
                                span: right.span,
                            });
                        };
                        if left_inner != right_inner {
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Operator is invalid".to_string(),
                                span: right.span,
                            });
                        }
                    }
                    _ => {}
                };

                match op.as_ref() {
                    BinaryOp::And | BinaryOp::Or => Type::Int,
                    BinaryOp::ShiftRight | BinaryOp::ShiftLeft => left_ty,
                    BinaryOp::Equal | BinaryOp::NotEqual => {
                        let common = if left_ty.is_pointer() || right_ty.is_pointer() {
                            self.common_pointer_type(left, &left_ty, right, &right_ty)?
                        } else {
                            Self::common_type(&left_ty, &right_ty)
                        };
                        self.cast_if_needed(left, &left_ty, &common);
                        self.cast_if_needed(right, &right_ty, &common);
                        Type::Int
                    }
                    BinaryOp::Subtract if left_ty.is_pointer() && right_ty.is_pointer() => {
                        Type::Long // TODO: Suspicious
                    }
                    BinaryOp::Add | BinaryOp::Subtract
                        if left_ty.is_pointer() && right_ty.is_int() =>
                    {
                        self.cast_if_needed(right, &right_ty, &Type::Long);
                        left_ty
                    }
                    BinaryOp::Add if left_ty.is_int() && right_ty.is_pointer() => {
                        self.cast_if_needed(left, &left_ty, &Type::Long);
                        right_ty
                    }
                    _ => {
                        let common = Self::common_type(&left_ty, &right_ty);
                        self.cast_if_needed(left, &left_ty, &common);
                        self.cast_if_needed(right, &right_ty, &common);
                        match op.as_ref() {
                            BinaryOp::LessThan
                            | BinaryOp::LessOrEqualThan
                            | BinaryOp::GreaterThan
                            | BinaryOp::GreaterOrEqualThan => Type::Int,
                            _ => common,
                        }
                    }
                }
            }

            Expression::Assignment { left, right, op } => {
                let left_ty = self.check_expression(left)?;
                if left_ty.is_array() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Array is not assignable".to_owned(),
                        span: left.span,
                    });
                }

                if !Self::is_lvalue(left) {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Expression is not assignable".to_owned(),
                        span: left.span,
                    });
                }
                let right_ty = self.check_and_convert_expr(right)?;

                match op.as_ref() {
                    AssignOp::BitAndEqual
                    | AssignOp::BitOrEqual
                    | AssignOp::BitXorEqual
                    | AssignOp::ShiftLeftEqual
                    | AssignOp::ShiftRightEqual
                    | AssignOp::ModEqual => {
                        if !left_ty.is_int() || !right_ty.is_int() {
                            let span = if !left_ty.is_int() {
                                left.span
                            } else {
                                right.span
                            };
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Assign compound operation requires integer operands"
                                    .to_string(),
                                span,
                            });
                        }
                    }
                    AssignOp::MulEqual | AssignOp::DivEqual => {
                        if left_ty.is_pointer() || right_ty.is_pointer() {
                            let span = if left_ty.is_pointer() {
                                left.span
                            } else {
                                right.span
                            };
                            return Err(CompilerError {
                                kind: ErrorKind::Type,
                                msg: "Assign compound operation cannot be used on pointer type"
                                    .to_string(),
                                span,
                            });
                        }
                    }
                    AssignOp::AddEqual | AssignOp::SubEqual
                        if left_ty.is_pointer() && !right_ty.is_int() =>
                    {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Assign compound operation on a pointer requires integer operand"
                                .to_string(),
                            span: right.span,
                        });
                    }

                    AssignOp::AddEqual | AssignOp::SubEqual if right_ty.is_pointer() => {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Assign compound operator cannot be a pointer type".to_string(),
                            span: right.span,
                        });
                    }
                    _ => {}
                };

                match op.as_ref() {
                    AssignOp::Equal => self.convert_by_assignment(right, &right_ty, &left_ty)?,
                    AssignOp::ShiftLeftEqual | AssignOp::ShiftRightEqual => {
                        self.cast_if_needed(right, &right_ty, &left_ty);
                        left_ty
                    }
                    _ => {
                        let common = Self::common_type(&left_ty, &right_ty);
                        self.cast_if_needed(left, &left_ty, &common);
                        self.cast_if_needed(right, &right_ty, &common);
                        self.cast_if_needed(expr, &common, &left_ty);
                        common
                    }
                }
            }

            Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                self.check_and_convert_expr(cond)?;
                let then_ty = self.check_and_convert_expr(then_expr)?;
                let else_ty = self.check_and_convert_expr(else_expr)?;
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
                    let arg_ty = self.check_and_convert_expr(arg)?;
                    self.convert_by_assignment(arg, &arg_ty, param_ty)?;
                }
                *function_ty.ret.clone()
            }
            Expression::Cast { target, expr } => {
                let ty = self.check_and_convert_expr(expr)?;
                if target.is_pointer() && ty.is_double() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Cannot cast a double to a pointer".to_string(),
                        span: expr.span,
                    });
                }
                if target.is_double() && ty.is_pointer() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Cannot cast a pointer to a double".to_string(),
                        span: expr.span,
                    });
                }
                if target.is_array() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Cannot cast to an array type".to_string(),
                        span: expr.span,
                    });
                }
                *target.data.clone()
            }
            Expression::Dereference(inner) => {
                let inner_ty = self.check_and_convert_expr(inner)?;
                match inner_ty {
                    Type::Pointer(referenced) => *referenced.clone(),
                    _ => {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Cannot dereference a non-pointer".to_string(),
                            span: expr.span,
                        })
                    }
                }
            }
            Expression::AddressOf(inner) => {
                if Self::is_lvalue(inner) {
                    let inner_ty = self.check_expression(inner)?;
                    Type::Pointer(inner_ty.clone().into())
                } else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Can't take address of non-lvalue!".to_string(),
                        span: expr.span,
                    });
                }
            }
            Expression::Subscript(expr1, expr2) => {
                let ty1 = self.check_and_convert_expr(expr1)?;
                let ty2 = self.check_and_convert_expr(expr2)?;
                match (&ty1, &ty2) {
                    (Type::Pointer(referenced), _) if ty2.is_int() => *referenced.clone(),
                    (_, Type::Pointer(referenced)) if ty1.is_int() => *referenced.clone(),
                    _ => {
                        return Err(CompilerError {
                            kind: ErrorKind::Type,
                            msg: "Subscript requires integer and pointer types".to_string(),
                            span: expr.span,
                        })
                    }
                }
            }
        };
        self.expression_types.insert(expr.id, ty.clone());
        Ok(ty)
    }

    fn is_lvalue(expr: &Expression) -> bool {
        matches!(
            expr,
            Expression::Var(_) | Expression::Dereference(_) | Expression::Subscript(_, _)
        )
    }

    fn is_null_constant(expr: &Expression) -> bool {
        matches!(
            expr,
            Expression::Constant(
                Constant::Int(0) | Constant::Long(0) | Constant::UInt(0) | Constant::ULong(0)
            )
        )
    }

    fn convert_by_assignment(
        &mut self,
        expr: &Node<Expression>,
        ty: &Type,
        target: &Type,
    ) -> Result<Type> {
        if ty == target {
            Ok(ty.clone())
        } else if ty.is_arithmetic() && target.is_arithmetic()
            || Self::is_null_constant(expr) && target.is_pointer()
        {
            Ok(self.cast_if_needed(expr, ty, target))
        } else {
            Err(CompilerError {
                kind: ErrorKind::Type,
                msg: "Cannot convert type for assignment!".to_string(),
                span: expr.span,
            })
        }
    }

    // TODO: maybe expected is not needed
    fn cast_if_needed(&mut self, expr: &Node<Expression>, ty: &Type, expected: &Type) -> Type {
        if ty == expected {
            expected.clone()
        } else {
            self.implicit_casts.insert(expr.id, expected.clone());
            expected.clone()
        }
    }

    fn common_type(ty1: &Type, ty2: &Type) -> Type {
        let result = if ty1 == ty2 {
            ty1
        } else if matches!(ty1, Type::Double) || matches!(ty2, Type::Double) {
            &Type::Double
        } else if ty1.size() == ty2.size() {
            if ty1.is_signed() {
                ty2
            } else {
                ty1
            }
        } else if ty1.size() > ty2.size() {
            ty1
        } else {
            ty2
        };
        result.clone()
    }

    fn common_pointer_type(
        &self,
        e1: &Node<Expression>,
        ty1: &Type,
        e2: &Node<Expression>,
        ty2: &Type,
    ) -> Result<Type> {
        if ty1 == ty2 {
            Ok(ty1.clone())
        } else if Self::is_null_constant(e1) {
            Ok(ty2.clone())
        } else if Self::is_null_constant(e2) {
            Ok(ty1.clone())
        } else {
            Err(CompilerError {
                kind: ErrorKind::Type,
                msg: "Expressions have incompatible types".to_string(),
                span: e1.span + e2.span,
            })
        }
    }
}

pub fn check(program: &Node<Program>) -> Result<SemanticData> {
    let mut type_checker = TypeChecker::default();
    type_checker.check(program)?;
    Ok(SemanticData {
        symbols: type_checker.symbols,
        expression_types: type_checker.expression_types,
        pointer_decays: type_checker.pointer_decays,
        implicit_casts: type_checker.implicit_casts,
        switch_cases: type_checker.switch_cases,
    })
}
