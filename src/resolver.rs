use crate::ast::{BlockItem, Declaration, Expression, Node, Program, Statement};
use crate::lexer::Span;
use crate::symbol::Symbol;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Default)]
struct Resolver {
    locals: HashMap<Symbol, Symbol>,
    counter: usize,
}

impl Resolver {
    fn resolve(mut self, mut program: Node<Program>) -> Result<Node<Program>> {
        for block_item in &mut program.function_definition.body {
            match block_item.as_mut() {
                BlockItem::Stmt(stmt) => self.resolve_statement(stmt)?,
                BlockItem::Decl(decl) => self.resolve_declaration(decl)?,
            }
        }
        Ok(program)
    }

    fn resolve_declaration(&mut self, decl: &mut Declaration) -> Result<()> {
        let name = &decl.name.symbol;
        if self.locals.contains_key(name) {
            return Err(NameError {
                msg: format!("Variable '{name}' was already declared"),
                span: decl.name.span,
            });
        }
        let unique_name = self.make_name(name);
        self.locals.insert(name.clone(), unique_name.clone());
        decl.name.symbol = unique_name;
        if let Some(init) = &mut decl.init {
            self.resolve_expression(init)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &mut Statement) -> Result<()> {
        match stmt {
            Statement::Return { expr } => self.resolve_expression(expr),
            Statement::Expression(expr) => self.resolve_expression(expr),
            Statement::Null => Ok(()),
        }
    }

    fn resolve_expression(&mut self, expr: &mut Node<Expression>) -> Result<()> {
        match expr.as_mut() {
            Expression::Var(name) => {
                if let Some(declared) = self.locals.get(name) {
                    *name = declared.clone();
                } else {
                    return Err(NameError {
                        msg: format!("Undeclared variable '{name}'"),
                        span: expr.span,
                    });
                }
            }
            Expression::Assignment { left, right, .. } => {
                if !matches!(left.as_ref(), Expression::Var(_)) {
                    return Err(NameError {
                        msg: "Invalid left side of the assignment".to_owned(),
                        span: left.span,
                    });
                }
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Unary { expr, .. } | Expression::Postfix { expr, .. } => {
                self.resolve_expression(expr)?;
            }
            Expression::Binary { left, right, .. } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Constant(_) => {}
        }
        Ok(())
    }

    fn make_name(&mut self, name: &Symbol) -> Symbol {
        let unique_name = format!("${name}{i}", i = self.counter);
        self.counter += 1;
        unique_name
    }
}

#[derive(Debug)]
pub struct NameError {
    pub msg: String,
    pub span: Span,
}

impl Display for NameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{msg} at {span:?}", msg = self.msg, span = self.span)
    }
}

impl Error for NameError {}

pub type Result<T> = std::result::Result<T, NameError>;

pub fn resolve(program: Node<Program>) -> Result<Node<Program>> {
    Resolver::default().resolve(program)
}
