use crate::ast::{
    Block, BlockItem, Declaration, Expression, ForInit, FunctionDeclaration, Identifier,
    Initializer, InnerRef, NameAndFields, Node, Program, Statement, StorageClass, TypeSpec,
    VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::symbol::Symbol;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct Resolver {
    scopes: VecDeque<Scope>,
    structs_scopes: VecDeque<StructScope>,
    counter: usize,
}

type Scope = HashMap<Symbol, Resolution>;
type StructScope = HashMap<Symbol, Symbol>;

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
                Declaration::Struct(d) | Declaration::Union(d) => {
                    self.resolve_type_declaration(d)?
                }
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
            Declaration::Struct(decl) | Declaration::Union(decl) => {
                self.resolve_type_declaration(decl)
            }
        }
    }

    fn resolve_file_var_declaration(&mut self, decl: &mut VarDeclaration) -> Result<()> {
        self.resolve_type(&mut decl.type_spec)?;
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
        self.resolve_type(&mut decl.type_spec)?;
        let symbol = &decl.name.symbol;
        let unique_name = self.make_name(symbol);
        let scope = self.scopes.front_mut().expect("Invalid scope state");
        let storage = decl.storage_class.inner_ref();
        if let Some(entry) = scope.get(symbol)
            && !(entry.linked && matches!(storage, Some(StorageClass::Extern)))
        {
            return Err(CompilerError {
                kind: ErrorKind::Resolve,
                msg: format!("Variable '{symbol}' was already declared"),
                span: decl.name.span,
            });
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
                self.resolve_initializer(init)?;
            }
        }
        Ok(())
    }

    fn resolve_initializer(&mut self, init: &mut Node<Initializer>) -> Result<()> {
        match init.as_mut() {
            Initializer::Single(expr) => self.resolve_expression(expr),
            Initializer::Compound(initializers) => {
                for initializer in initializers {
                    self.resolve_initializer(initializer)?;
                }
                Ok(())
            }
        }
    }

    fn resolve_function_declaration(&mut self, decl: &mut FunctionDeclaration) -> Result<()> {
        self.resolve_type(&mut decl.type_spec.ret)?;
        for param in &mut decl.type_spec.params {
            self.resolve_type(param)?;
        }
        let symbol = &decl.name.symbol;
        let scope = self.scopes.front_mut().expect("Invalid scope state");
        if let Some(resolution) = scope.get(symbol)
            && !resolution.linked
        {
            return Err(CompilerError {
                kind: ErrorKind::Resolve,
                msg: format!("Variable '{symbol}' was already declared"),
                span: decl.name.span,
            });
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

    fn resolve_type_declaration(&mut self, decl: &mut NameAndFields) -> Result<()> {
        let tag = &decl.name.symbol;
        let unique_name = self.make_name(tag);
        let scope = self
            .structs_scopes
            .front_mut()
            .expect("Invalid scope state");

        let unique_name = if let Some(n) = scope.get(tag) {
            n.clone()
        } else {
            scope.insert(tag.clone(), unique_name.clone());
            unique_name
        };

        decl.name.symbol = unique_name;
        for field in &mut decl.fields {
            self.resolve_type(&mut field.type_spec)?;
        }
        Ok(())
    }

    fn resolve_type(&mut self, ty: &mut Node<TypeSpec>) -> Result<()> {
        match ty.as_mut() {
            TypeSpec::Struct(tag) | TypeSpec::Union(tag) => {
                for scope in &self.structs_scopes {
                    if let Some(declared) = scope.get(&tag.symbol) {
                        tag.symbol = declared.clone();
                        return Ok(());
                    }
                }
                Err(CompilerError {
                    kind: ErrorKind::Resolve,
                    msg: format!("Undeclared structure type '{}'", tag.symbol),
                    span: ty.span,
                })
            }
            TypeSpec::Pointer(inner) | TypeSpec::Array(inner, _) => self.resolve_type(inner),
            TypeSpec::Function(f) => {
                self.resolve_type(&mut f.ret)?;
                for param in &mut f.params {
                    self.resolve_type(param)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn resolve_statement(&mut self, stmt: &mut Node<Statement>) -> Result<()> {
        match stmt.as_mut() {
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.resolve_expression(expr)?;
                }
            }
            Statement::Expression(expr) => {
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
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Unary { expr: operand, .. } => {
                self.resolve_expression(operand)?;
            }
            Expression::Postfix { expr, .. } => {
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

            Expression::Cast { target, expr } => {
                self.resolve_type(target)?;
                self.resolve_expression(expr)?;
            }

            Expression::SizeOfType(ty) => {
                self.resolve_type(ty)?;
            }

            Expression::Dereference(expr)
            | Expression::AddressOf(expr)
            | Expression::SizeOfExpr(expr)
            | Expression::Arrow { pointer: expr, .. }
            | Expression::Dot {
                aggregate: expr, ..
            } => {
                self.resolve_expression(expr)?;
            }
            Expression::Subscript(expr1, expr2) => {
                self.resolve_expression(expr1)?;
                self.resolve_expression(expr2)?;
            }
            Expression::Constant(_) | Expression::String(_) => {}
        }
        Ok(())
    }

    fn make_name(&mut self, name: &Symbol) -> Symbol {
        let unique_name = format!("{name}.{i}", i = self.counter);
        self.counter += 1;
        Symbol::from(unique_name)
    }

    fn begin_scope(&mut self) {
        self.scopes.push_front(HashMap::new());
        self.structs_scopes.push_front(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop_front().expect("No scope to end!");
        self.structs_scopes.pop_front().expect("No scope to end!");
    }
}

pub fn check(program: Node<Program>) -> Result<Node<Program>> {
    Resolver::default().resolve(program)
}
