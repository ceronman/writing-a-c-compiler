use crate::ast::{
    Block, BlockItem, Declaration, Expression, ForInit, FunctionDeclaration, Node, Program,
    Statement, VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::symbol::Symbol;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct TypeChecker {
    types: HashMap<Symbol, Type>,
    defined_functions: HashSet<Symbol>,
}

#[derive(Eq, PartialEq)]
enum Type {
    Int,
    Function { num_params: usize },
}

impl TypeChecker {
    fn check(&mut self, program: &Program) -> Result<()> {
        for decl in &program.declarations {
            match decl.as_ref() {
                Declaration::Var(_) => todo!(),
                Declaration::Function(f) => self.check_function_declaration(f, true)?,
            }
        }
        Ok(())
    }

    fn check_var_declaration(&mut self, decl: &VarDeclaration) -> Result<()> {
        self.types.insert(decl.name.symbol.clone(), Type::Int);
        if let Some(init) = &decl.init {
            self.check_expression(init)?;
        }
        Ok(())
    }

    fn check_function_declaration(
        &mut self,
        decl: &FunctionDeclaration,
        top_level: bool,
    ) -> Result<()> {
        let name = &decl.name.symbol;
        let this_ty = Type::Function {
            num_params: decl.params.len(),
        };
        let has_body = decl.body.is_some();
        if let Some(prev_ty) = self.types.get(name) {
            if *prev_ty != this_ty {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: format!("Conflicting declaration types for '{name}'"),
                    span: decl.name.span,
                });
            }
        }
        if self.defined_functions.contains(name) && has_body {
            return Err(CompilerError {
                kind: ErrorKind::Type,
                msg: format!("Function '{name}' is defined more than once"),
                span: decl.name.span,
            });
        }
        self.types.insert(name.clone(), this_ty);
        if let Some(body) = &decl.body {
            if !top_level {
                return Err(CompilerError {
                    kind: ErrorKind::Type,
                    msg: "Nested function definitions are not allowed ".to_owned(),
                    span: decl.name.span,
                });
            }
            self.defined_functions.insert(name.clone());
            for param in &decl.params {
                self.types.insert(param.symbol.clone(), Type::Int);
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
                    ForInit::Decl(d) => self.check_var_declaration(d)?,
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
            Declaration::Var(d) => self.check_var_declaration(d),
            Declaration::Function(d) => self.check_function_declaration(d, false),
        }
    }

    fn check_expression(&self, expr: &Node<Expression>) -> Result<()> {
        match expr.as_ref() {
            Expression::Constant(_) => {}
            Expression::Var(name) => {
                let Some(Type::Int) = self.types.get(name) else {
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
                let Some(Type::Function { num_params }) = self.types.get(&symbol) else {
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

pub fn check(program: Node<Program>) -> Result<Node<Program>> {
    TypeChecker::default().check(&program)?;
    Ok(program)
}
