#[cfg(test)]
mod test;

use crate::ast::{BlockItem, Declaration, Expression, Node, Program, Statement, UnaryOp};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::symbol::Symbol;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Resolver {
    locals: HashMap<Symbol, Symbol>,
    labels: HashSet<Symbol>,
    counter: usize,
}

impl Resolver {
    fn resolve(mut self, mut program: Node<Program>) -> Result<Node<Program>> {
        for block_item in &mut program.function_definition.body.items {
            match block_item {
                BlockItem::Stmt(stmt) => self.resolve_statement(stmt)?,
                BlockItem::Decl(decl) => self.resolve_declaration(decl)?,
            }
        }

        // Second pass to check `goto` statements
        for block_item in &mut program.function_definition.body.items {
            if let BlockItem::Stmt(stmt) = block_item {
                if let Statement::Goto(name) = stmt.as_ref() {
                    if !self.labels.contains(&name.symbol) {
                        return Err(CompilerError {
                            kind: ErrorKind::Resolve,
                            msg: format!("Undefined label '{}'", name.symbol),
                            span: name.span,
                        });
                    }
                }
            }
        }
        Ok(program)
    }

    fn resolve_declaration(&mut self, decl: &mut Declaration) -> Result<()> {
        let name = &decl.name.symbol;
        if self.locals.contains_key(name) {
            return Err(CompilerError {
                kind: ErrorKind::Resolve,
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
                Ok(())
            }
            Statement::Labeled { name, stmt } => {
                if self.labels.contains(&name.symbol) {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: format!("Label '{}' was already defined", name.symbol),
                        span: name.span,
                    });
                }
                self.labels.insert(name.symbol.clone());
                self.resolve_statement(stmt)
            }
            Statement::Goto(_) => {
                // Note: checking if label exists is done in another pass
                Ok(())
            }
            Statement::Compound(_) => todo!(),
        }
    }

    fn resolve_expression(&mut self, expr: &mut Node<Expression>) -> Result<()> {
        match expr.as_mut() {
            Expression::Var(name) => {
                if let Some(declared) = self.locals.get(name) {
                    *name = declared.clone();
                } else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: format!("Undeclared variable '{name}'"),
                        span: expr.span,
                    });
                }
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
        }
        Ok(())
    }

    fn make_name(&mut self, name: &Symbol) -> Symbol {
        let unique_name = format!("{name}.{i}", i = self.counter);
        self.counter += 1;
        unique_name
    }
}

pub fn resolve(program: Node<Program>) -> Result<Node<Program>> {
    Resolver::default().resolve(program)
}
