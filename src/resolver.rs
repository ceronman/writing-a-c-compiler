#[cfg(test)]
mod test;

use crate::ast::{
    Block, BlockItem, Declaration, Expression, ForInit, Node, Program, Statement, SwitchLabels,
    UnaryOp, VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::symbol::Symbol;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct Resolver {
    scopes: VecDeque<HashMap<Symbol, Symbol>>,
    labels: HashMap<Symbol, Symbol>,
    label_stack: VecDeque<LabelKind>,
    switch_labels: VecDeque<SwitchLabels>,
    counter: usize,
}

enum LabelKind {
    Loop(Symbol),
    Switch(Symbol),
}

impl Resolver {
    fn resolve(mut self, mut program: Node<Program>) -> Result<Node<Program>> {
        let body = program
            .functions
            .first_mut()
            .unwrap()
            .body
            .as_mut()
            .unwrap();
        self.resolve_block(body)?;
        self.check_gotos_block(body)?;

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
            Statement::Switch { body, .. }
            | Statement::While { body, .. }
            | Statement::DoWhile { body, .. }
            | Statement::For { body, .. }
            | Statement::Labeled { body, .. }
            | Statement::Case { body, .. }
            | Statement::Default { body, .. } => {
                self.check_gotos_stmt(body)?;
            }
            Statement::Compound(block) => {
                self.check_gotos_block(block)?;
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
        match decl {
            Declaration::Var(decl) => self.resolve_var_declaration(decl),
            Declaration::Function(_) => todo!(),
        }
    }

    fn resolve_var_declaration(&mut self, decl: &mut VarDeclaration) -> Result<()> {
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
            Statement::Labeled { name, body: stmt } => {
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
                self.label_stack.push_front(LabelKind::Loop(label.clone()));
                self.resolve_expression(cond)?;
                self.resolve_statement(body)?;
                self.label_stack.pop_front();
            }
            Statement::For {
                init,
                cond,
                post,
                body,
                label,
            } => {
                *label = self.make_label("loop");
                self.label_stack.push_front(LabelKind::Loop(label.clone()));
                let mut scoped_init = false;
                match init {
                    ForInit::Decl(d) => {
                        self.begin_scope();
                        scoped_init = true;
                        self.resolve_var_declaration(d)?
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
                self.label_stack.pop_front();
            }
            Statement::Switch {
                expr: cond,
                body,
                labels,
            } => {
                let new_labels = SwitchLabels {
                    label: self.make_label("switch"),
                    valued: vec![],
                    default: None,
                };
                self.label_stack
                    .push_front(LabelKind::Switch(new_labels.label.clone()));
                self.switch_labels.push_front(new_labels);
                self.resolve_expression(cond)?;
                self.resolve_statement(body)?;
                *labels = self
                    .switch_labels
                    .pop_front()
                    .expect("Unreachable: switch statement without cases");
                self.label_stack.pop_front();
            }
            Statement::Break(label) => {
                if let Some(LabelKind::Loop(enclosing_label) | LabelKind::Switch(enclosing_label)) =
                    self.label_stack.front_mut()
                {
                    *label = enclosing_label.clone();
                } else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "'break' statement not in loop or switch statement".to_owned(),
                        span: stmt.span,
                    });
                }
            }
            Statement::Continue(label) => {
                let Some(enclosing_label) = self.label_stack.iter().find_map(|kind| match kind {
                    LabelKind::Loop(label) => Some(label),
                    LabelKind::Switch(_) => None,
                }) else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "'continue' statement not in loop statement".to_owned(),
                        span: stmt.span,
                    });
                };
                *label = enclosing_label.clone();
            }
            Statement::Case { label, value, body } => {
                let Expression::Constant(int_value) = *value.as_ref() else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "case label does not reduce to an integer constant".to_owned(),
                        span: value.span,
                    });
                };
                let Some(enclosing_label) = self.label_stack.iter().find_map(|kind| match kind {
                    LabelKind::Loop(_) => None,
                    LabelKind::Switch(label) => Some(label),
                }) else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "case label not within a switch statement".to_owned(),
                        span: stmt.span,
                    });
                };
                *label = format!("case_{int_value}_{enclosing_label}");
                let cases = self
                    .switch_labels
                    .front_mut()
                    .expect("Unreachable: case without a switch");
                if cases.valued.iter().any(|&(value, _)| value == int_value) {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "duplicate case value".to_owned(),
                        span: value.span,
                    });
                }
                cases.valued.push((int_value, label.clone()));
                self.resolve_statement(body)?;
            }
            Statement::Default { label, body } => {
                let Some(enclosing_label) = self.label_stack.iter().find_map(|kind| match kind {
                    LabelKind::Loop(_) => None,
                    LabelKind::Switch(label) => Some(label),
                }) else {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "default label not within a switch statement".to_owned(),
                        span: stmt.span,
                    });
                };
                *label = format!("default_{enclosing_label}");
                let cases = self
                    .switch_labels
                    .front_mut()
                    .expect("Unreachable: default without a switch");
                if cases.default.is_some() {
                    return Err(CompilerError {
                        kind: ErrorKind::Resolve,
                        msg: "multiple default labels in one switch".to_owned(),
                        span: stmt.span,
                    });
                }
                cases.default = Some(label.clone());
                self.resolve_statement(body)?;
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
            _ => todo!(),
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
