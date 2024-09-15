#[cfg(test)]
mod test;

use crate::ast::{
    Block, BlockItem, Declaration, Expression, ForInit, Node, Program, Statement, UnaryOp,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::symbol::Symbol;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct Resolver {
    scopes: VecDeque<HashMap<Symbol, Symbol>>,
    labels: HashMap<Symbol, Symbol>,
    loops: VecDeque<Symbol>,
    counter: usize,
}

impl Resolver {
    fn resolve(mut self, mut program: Node<Program>) -> Result<Node<Program>> {
        self.resolve_block(&mut program.function_definition.body)?;
        self.check_gotos_block(&mut program.function_definition.body)?;

        Ok(program)
    }

    fn resolve_block(&mut self, block: &mut Block) -> Result<()> {
        self.begin_scope();
        for block_item in &mut block.items {
            match block_item {
                BlockItem::Stmt(stmt) => self.resolve_statement(stmt)?,
                BlockItem::Decl(decl) => self.resolve_declaration(decl)?,
            }
        }
        self.end_scope();
        Ok(())
    }

    fn check_gotos_block(&self, block: &mut Node<Block>) -> Result<()> {
        for block_item in &mut block.items {
            if let BlockItem::Stmt(stmt) = block_item {
                self.check_gotos_stmt(stmt)?
            }
        }
        Ok(())
    }

    fn check_gotos_stmt(&self, stmt: &mut Node<Statement>) -> Result<()> {
        match stmt.as_mut() {
            Statement::Goto(name) => {
                let x = name.symbol.as_str();
                println!("{x}");
                if let Some(new_name) = self.labels.get(&name.symbol) {
                    name.symbol = new_name.clone();
                } else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: format!("Undefined label '{}'", name.symbol),
                        span: name.span,
                    });
                }
            }
            Statement::If {
                then_stmt,
                else_stmt,
                ..
            } => {
                self.check_gotos_stmt(then_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.check_gotos_stmt(else_stmt)?;
                }
            }
            Statement::Compound(block) => {
                self.check_gotos_block(block)?;
            }
            Statement::While { body, .. } => {
                self.check_gotos_stmt(body)?;
            }
            Statement::DoWhile { body, .. } => {
                self.check_gotos_stmt(body)?;
            }
            Statement::For { body, .. } => {
                self.check_gotos_stmt(body)?;
            }

            Statement::Labeled { stmt, .. } => {
                self.check_gotos_stmt(stmt)?;
            }

            Statement::Return { .. }
            | Statement::Expression(_)
            | Statement::Break(_)
            | Statement::Continue(_)
            | Statement::Null => {}
        }
        Ok(())
    }

    fn resolve_declaration(&mut self, decl: &mut Declaration) -> Result<()> {
        let name = &decl.name.symbol;
        let unique_name = self.make_name(name).clone();
        let Some(scope) = self.scopes.front_mut() else {
            return Err(CompilerError {
                kind: ErrorKind::Resolve,
                msg: format!("Variable '{name}' declared outside of a scope"),
                span: decl.name.span,
            });
        };
        if scope.contains_key(name) {
            return Err(CompilerError {
                kind: ErrorKind::Resolve,
                msg: format!("Variable '{name}' was already declared"),
                span: decl.name.span,
            });
        }
        scope.insert(name.clone(), unique_name.clone());
        decl.name.symbol = unique_name;
        if let Some(init) = &mut decl.init {
            self.resolve_expression(init)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &mut Node<Statement>) -> Result<()> {
        match stmt.as_mut() {
            Statement::Return { expr } => {
                self.resolve_expression(expr)?;
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
            Statement::Labeled { name, stmt } => {
                if self.labels.contains_key(&name.symbol) {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: format!("Label '{}' was already defined", name.symbol),
                        span: name.span,
                    });
                }
                let new_label = self.make_label(&name.symbol);
                self.labels.insert(name.symbol.clone(), new_label.clone());
                name.symbol = new_label;
                self.resolve_statement(stmt)?;
            }
            Statement::Compound(block) => {
                self.resolve_block(block)?;
            }

            Statement::While { cond, body, label } | Statement::DoWhile { cond, body, label } => {
                *label = self.make_label("loop");
                self.loops.push_front(label.clone());
                self.resolve_expression(cond)?;
                self.resolve_statement(body)?;
                self.loops.pop_front();
            }
            Statement::For {
                init,
                cond,
                post,
                body,
                label,
            } => {
                *label = self.make_label("loop");
                self.loops.push_front(label.clone());
                let mut scoped_init = false;
                match init {
                    ForInit::Decl(d) => {
                        self.begin_scope();
                        scoped_init = true;
                        self.resolve_declaration(d)?
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
                self.loops.pop_front();
            }
            Statement::Break(label) => {
                if let Some(current_loop) = self.loops.front_mut() {
                    *label = current_loop.clone();
                } else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "'break' statement not in loop or switch statement".to_owned(),
                        span: stmt.span,
                    });
                }
            }
            Statement::Continue(label) => {
                if let Some(current_loop) = self.loops.front_mut() {
                    *label = current_loop.clone();
                } else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "'continue' statement not in loop statement".to_owned(),
                        span: stmt.span,
                    });
                }
            }
            Statement::Goto(_) => {
                // Note: checking if label exists is done in another pass
            }
            Statement::Null => {}
        }
        Ok(())
    }

    fn resolve_expression(&mut self, expr: &mut Node<Expression>) -> Result<()> {
        match expr.as_mut() {
            Expression::Var(name) => {
                for scope in &self.scopes {
                    if let Some(declared) = scope.get(name) {
                        *name = declared.clone();
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
        }
        Ok(())
    }

    fn make_name(&mut self, name: &str) -> Symbol {
        let unique_name = format!("{name}.{i}", i = self.counter);
        self.counter += 1;
        unique_name
    }

    fn make_label(&mut self, name: &str) -> Symbol {
        let unique_name = format!("{name}_{i}", i = self.counter);
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

pub fn resolve(program: Node<Program>) -> Result<Node<Program>> {
    Resolver::default().resolve(program)
}
