use crate::ast::{
    Block, BlockItem, Declaration, Expression, ForInit, FunctionDeclaration, InnerRef, Node,
    Program, Statement, StorageClass, VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::symbol::Symbol;
use std::collections::HashMap;

pub type SymbolTable = HashMap<Symbol, SymbolData>;

#[derive(Default)]
struct TypeChecker {
    symbol_table: SymbolTable,
}

#[derive(Debug)]
pub struct SymbolData {
    pub ty: Type,
    pub attrs: Attributes,
}

impl SymbolData {
    fn local() -> Self {
        SymbolData {
            ty: Type::Int,
            attrs: Attributes::Local,
        }
    }
    fn global() -> Self {
        SymbolData {
            ty: Type::Int,
            attrs: Attributes::Static {
                initial_value: InitialValue::NoInitializer,
                global: true,
            },
        }
    }
}

#[derive(Debug)]
pub enum Attributes {
    Function {
        defined: bool,
        global: bool,
    },
    Static {
        initial_value: InitialValue,
        global: bool,
    },
    Local,
}

#[derive(Clone, Copy, Debug)]
pub enum InitialValue {
    Tentative,
    Initial(i64),
    NoInitializer,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Int,
    Function { num_params: usize },
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
            if let Some(data) = self.symbol_table.get(&name) {
                if !matches!(data.ty, Type::Int) {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: format!("Function with name '{name}' already declared"),
                        span: decl.name.span,
                    });
                }
            } else {
                self.symbol_table
                    .insert(decl.name.symbol.clone(), SymbolData::global());
            }
        } else if let Some(StorageClass::Static) = decl.storage_class.inner_ref() {
            let initial_value = if let Some(init) = &decl.init {
                if let Expression::Constant(value) = init.as_ref() {
                    InitialValue::Initial(*value)
                } else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: format!("Non-constant initializer on local static variable '{name}'"),
                        span: init.span,
                    });
                }
            } else {
                InitialValue::Initial(0)
            };
            self.symbol_table.insert(
                decl.name.symbol.clone(),
                SymbolData {
                    ty: Type::Int,
                    attrs: Attributes::Static {
                        initial_value,
                        global: false,
                    },
                },
            );
        } else {
            self.symbol_table
                .insert(decl.name.symbol.clone(), SymbolData::local());
            if let Some(init) = &decl.init {
                self.check_expression(init)?;
            }
        }
        Ok(())
    }

    fn check_file_var_declaration(&mut self, decl: &VarDeclaration) -> Result<()> {
        let mut initial_value = if let Some(init) = &decl.init {
            if let Expression::Constant(value) = init.as_ref() {
                InitialValue::Initial(*value)
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
        if let Some(data) = self.symbol_table.get(&name) {
            if data.ty != Type::Int {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Name '{name}' is already used for a function"),
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
            initial_value = match (initial_value, old_initial) {
                (InitialValue::Initial(_), InitialValue::Initial(_)) => {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: format!("Variable '{name}' has conflicting definitions"),
                        span: decl.name.span,
                    });
                }
                (InitialValue::Initial(_), _) => *old_initial,
                (_, InitialValue::Tentative) => InitialValue::Tentative,
                _ => initial_value,
            };
        }
        let data = SymbolData {
            ty: Type::Int,
            attrs: Attributes::Static {
                initial_value,
                global,
            },
        };
        self.symbol_table.insert(name, data);
        Ok(())
    }

    fn check_function_declaration(
        &mut self,
        decl: &FunctionDeclaration,
        top_level: bool,
    ) -> Result<()> {
        let name = decl.name.symbol.clone();
        let this_ty = Type::Function {
            num_params: decl.params.len(),
        };
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

        if let Some(data) = self.symbol_table.get(&name) {
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
        self.symbol_table.insert(name, data);
        if let Some(body) = &decl.body {
            if !top_level {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: "Nested function definitions are not allowed ".to_owned(),
                    span: decl.name.span,
                });
            }
            for param in &decl.params {
                self.symbol_table
                    .insert(param.symbol.clone(), SymbolData::local());
            }
            self.check_block(body)?;
        }
        Ok(())
    }

    fn check_block(&mut self, block: &Block) -> Result<()> {
        for item in &block.items {
            match item {
                BlockItem::Stmt(stmt) => self.check_statement(stmt)?,
                BlockItem::Decl(decl) => self.check_declaration(decl)?,
            }
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Node<Statement>) -> Result<()> {
        match stmt.as_ref() {
            Statement::Return(expr) | Statement::Expression(expr) => self.check_expression(expr)?,
            Statement::If {
                cond,
                then_stmt,
                else_stmt,
            } => {
                self.check_expression(cond)?;
                self.check_statement(then_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.check_statement(else_stmt)?;
                }
            }
            Statement::Switch { expr, body, .. }
            | Statement::Case {
                value: expr, body, ..
            }
            | Statement::While {
                cond: expr, body, ..
            }
            | Statement::DoWhile {
                cond: expr, body, ..
            } => {
                self.check_expression(expr)?;
                self.check_statement(body)?;
            }
            Statement::Labeled { body, .. } | Statement::Default { body, .. } => {
                self.check_statement(body)?
            }
            Statement::Compound(block) => self.check_block(block)?,
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
                    ForInit::Expr(e) => self.check_expression(e)?,
                    ForInit::None => {}
                }
                if let Some(cond) = cond {
                    self.check_expression(cond)?;
                }
                if let Some(post) = post {
                    self.check_expression(post)?;
                }
                self.check_statement(body)?;
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

    fn check_expression(&self, expr: &Node<Expression>) -> Result<()> {
        match expr.as_ref() {
            Expression::Constant(_) => {}
            Expression::Var(name) => {
                let Some(data) = self.symbol_table.get(name) else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Unknown type of expression".to_string(),
                        span: expr.span,
                    });
                };
                if data.ty != Type::Int {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Function used as variable".to_string(),
                        span: expr.span,
                    });
                };
            }
            Expression::Binary { left, right, .. } | Expression::Assignment { left, right, .. } => {
                self.check_expression(left)?;
                self.check_expression(right)?;
            }
            Expression::Postfix { expr, .. } | Expression::Unary { expr, .. } => {
                self.check_expression(expr)?;
            }
            Expression::FunctionCall { name, args } => {
                let symbol = name.symbol.clone();
                let Some(data) = self.symbol_table.get(&symbol) else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Unknown type of expression".to_string(),
                        span: expr.span,
                    });
                };
                let Type::Function { num_params } = &data.ty else {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Variable used as function name".to_string(),
                        span: expr.span,
                    });
                };
                if *num_params != args.len() {
                    return Err(CompilerError {
                        kind: ErrorKind::Type,
                        msg: "Function called with the wrong number of arguments".to_string(),
                        span: expr.span,
                    });
                }
                for arg in args {
                    self.check_expression(arg)?;
                }
            }
            Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                self.check_expression(cond)?;
                self.check_expression(then_expr)?;
                self.check_expression(else_expr)?;
            }
        }
        Ok(())
    }
}

pub fn check(program: &Node<Program>) -> Result<SymbolTable> {
    let mut type_checker = TypeChecker::default();
    type_checker.check(&program)?;
    Ok(type_checker.symbol_table)
}
