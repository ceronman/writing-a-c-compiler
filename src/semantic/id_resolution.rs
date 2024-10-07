use crate::ast::{
    Block, BlockItem, Declaration, Expression, ForInit, FunctionDeclaration, Identifier, InnerRef,
    Node, Program, Statement, StorageClass, UnaryOp, VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::symbol::Symbol;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct Resolver {
    scopes: VecDeque<Scope>,
    counter: usize,
}

type Scope = HashMap<Symbol, Resolution>;

struct Resolution {
    name: Symbol,
    linked: bool,
}

impl Resolver {
    fn resolve(mut self, mut program: Node<Program>) -> Result<Node<Program>> {
        self.begin_scope();
        for decl in &mut program.declarations {
            match decl.as_mut() {
                Declaration::Var(d) => self.resolve_file_var_declaration(d)?,
                Declaration::Function(d) => self.resolve_function_declaration(d)?,
            };
        }
        self.end_scope();
        Ok(program)
    }

    fn resolve_block(&mut self, block: &mut Block) -> Result<()> {
        for block_item in &mut block.items {
            match block_item {
                BlockItem::Stmt(stmt) => self.resolve_statement(stmt)?,
                BlockItem::Decl(decl) => self.resolve_declaration(decl)?,
            }
        }
        Ok(())
    }

    fn resolve_declaration(&mut self, decl: &mut Declaration) -> Result<()> {
        match decl {
            Declaration::Var(decl) => self.resolve_local_var_declaration(decl),
            Declaration::Function(decl) => self.resolve_function_declaration(decl),
        }
    }
    fn resolve_file_var_declaration(&mut self, decl: &mut VarDeclaration) -> Result<()> {
        let scope = self.scopes.front_mut().expect("Invalid scope state");
        scope.insert(
            decl.name.symbol.clone(),
            Resolution {
                name: decl.name.symbol.clone(),
                linked: true,
            },
        );
        Ok(())
    }

    fn resolve_local_var_declaration(&mut self, decl: &mut VarDeclaration) -> Result<()> {
        let symbol = &decl.name.symbol;
        let unique_name = self.make_name(symbol);
        let scope = self.scopes.front_mut().expect("Invalid scope state");
        let storage = decl.storage_class.inner_ref();
        if let Some(entry) = scope.get(symbol) {
            if !(entry.linked && matches!(storage, Some(StorageClass::Extern))) {
                return Err(CompilerError {
                    kind: ErrorKind::Resolve,
                    msg: format!("Variable '{symbol}' was already declared"),
                    span: decl.name.span,
                });
            }
        }
        if let Some(StorageClass::Extern) = storage {
            scope.insert(
                symbol.clone(),
                Resolution {
                    name: decl.name.symbol.clone(),
                    linked: true,
                },
            );
        } else {
            scope.insert(
                symbol.clone(),
                Resolution {
                    name: unique_name.clone(),
                    linked: false,
                },
            );
            decl.name.symbol = unique_name;
            if let Some(init) = &mut decl.init {
                self.resolve_expression(init)?;
            }
        }
        Ok(())
    }

    fn resolve_function_declaration(&mut self, decl: &mut FunctionDeclaration) -> Result<()> {
        let symbol = &decl.name.symbol;
        let scope = self.scopes.front_mut().expect("Invalid scope state");
        if let Some(resolution) = scope.get(symbol) {
            if !resolution.linked {
                return Err(CompilerError {
                    kind: ErrorKind::Resolve,
                    msg: format!("Variable '{symbol}' was already declared"),
                    span: decl.name.span,
                });
            }
        }
        scope.insert(
            symbol.clone(),
            Resolution {
                name: symbol.clone(),
                linked: true,
            },
        );
        self.begin_scope();
        for param in &mut decl.params {
            self.resolve_param(param)?
        }
        if let Some(body) = &mut decl.body {
            self.resolve_block(body)?;
        }
        self.end_scope();
        Ok(())
    }

    fn resolve_param(&mut self, param: &mut Node<Identifier>) -> Result<()> {
        let symbol = param.symbol.clone();
        let unique_name = self.make_name(&symbol).clone();
        let scope = self.scopes.front_mut().expect("Invalid scope state");
        if scope.contains_key(&symbol) {
            return Err(CompilerError {
                kind: ErrorKind::Resolve,
                msg: format!("Parameter '{symbol}' was already declared"),
                span: param.span,
            });
        }
        param.symbol = unique_name.clone();
        scope.insert(
            symbol.clone(),
            Resolution {
                name: unique_name.clone(),
                linked: false,
            },
        );
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &mut Node<Statement>) -> Result<()> {
        match stmt.as_mut() {
            Statement::Return(expr) | Statement::Expression(expr) => {
                self.resolve_expression(expr)?;
            }
            Statement::If {
                cond,
                then_stmt,
                else_stmt,
            } => {
                self.resolve_expression(cond)?;
                self.resolve_statement(then_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.resolve_statement(else_stmt)?;
                }
            }
            Statement::Labeled { body, .. }
            | Statement::Case { body, .. }
            | Statement::Default { body, .. } => {
                self.resolve_statement(body)?;
            }
            Statement::Compound(block) => {
                self.begin_scope();
                self.resolve_block(block)?;
                self.end_scope();
            }

            Statement::While { cond, body, .. }
            | Statement::DoWhile { cond, body, .. }
            | Statement::Switch {
                expr: cond, body, ..
            } => {
                self.resolve_expression(cond)?;
                self.resolve_statement(body)?;
            }
            Statement::For {
                init,
                cond,
                post,
                body,
                ..
            } => {
                let mut scoped_init = false;
                match init {
                    ForInit::Decl(d) => {
                        self.begin_scope();
                        scoped_init = true;
                        self.resolve_local_var_declaration(d)?
                    }
                    ForInit::Expr(e) => self.resolve_expression(e)?,
                    ForInit::None => {}
                }
                if let Some(cond) = cond {
                    self.resolve_expression(cond)?;
                }
                if let Some(post) = post {
                    self.resolve_expression(post)?;
                }
                self.resolve_statement(body)?;
                if scoped_init {
                    self.end_scope();
                }
            }

            Statement::Break(_) | Statement::Continue(_) | Statement::Goto(_) | Statement::Null => {
            }
        }
        Ok(())
    }

    fn resolve_expression(&mut self, expr: &mut Node<Expression>) -> Result<()> {
        match expr.as_mut() {
            Expression::Var(name) => {
                for scope in &self.scopes {
                    if let Some(declared) = scope.get(name) {
                        *name = declared.name.clone();
                        return Ok(());
                    }
                }
                return Err(CompilerError {
                    kind: ErrorKind::Resolve,
                    msg: format!("Undeclared variable '{name}'"),
                    span: expr.span,
                });
            }
            Expression::Assignment { left, right, .. } => {
                if !matches!(left.as_ref(), Expression::Var(_)) {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "Expression is not assignable".to_owned(),
                        span: left.span,
                    });
                }
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Unary { expr: operand, op } => {
                if let UnaryOp::Increment | UnaryOp::Decrement = op.as_ref() {
                    if !matches!(operand.as_ref(), Expression::Var(_)) {
                        return Err(CompilerError {
                            kind: ErrorKind::Resolve,
                            msg: "Expression is not assignable".to_owned(),
                            span: operand.span,
                        });
                    }
                }
                self.resolve_expression(operand)?;
            }
            Expression::Postfix { expr, .. } => {
                if !matches!(expr.as_ref(), Expression::Var(_)) {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "Expression is not assignable".to_owned(),
                        span: expr.span,
                    });
                }
                self.resolve_expression(expr)?;
            }
            Expression::Binary { left, right, .. } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                self.resolve_expression(cond)?;
                self.resolve_expression(then_expr)?;
                self.resolve_expression(else_expr)?;
            }
            Expression::Constant(_) => {}
            Expression::FunctionCall { name, args } => {
                let symbol = &name.symbol;
                for scope in &self.scopes {
                    if let Some(resolution) = scope.get(symbol) {
                        name.symbol = resolution.name.clone();
                        for arg in args {
                            self.resolve_expression(arg)?;
                        }
                        return Ok(());
                    }
                }
                return Err(CompilerError {
                    kind: ErrorKind::Resolve,
                    msg: format!("Undeclared function '{symbol}'"),
                    span: name.span,
                });
            }
        }
        Ok(())
    }

    fn make_name(&mut self, name: &str) -> Symbol {
        let unique_name = format!("{name}.{i}", i = self.counter);
        self.counter += 1;
        unique_name
    }

    fn begin_scope(&mut self) {
        self.scopes.push_front(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop_front().expect("No scope to end!");
    }
}

pub fn check(program: Node<Program>) -> Result<Node<Program>> {
    Resolver::default().resolve(program)
}
