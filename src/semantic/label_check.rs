use crate::ast::{
    Block, BlockItem, Expression, FunctionDeclaration, Node, Program, Statement, SwitchLabels,
};
use crate::error::{CompilerError, ErrorKind};
use crate::symbol::Symbol;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct LabelChecker {
    labels: HashMap<Symbol, Symbol>,
    label_stack: VecDeque<LabelKind>,
    switch_labels: VecDeque<SwitchLabels>,
    counter: usize,
}

enum LabelKind {
    Loop(Symbol),
    Switch(Symbol),
}

impl LabelChecker {
    fn check(mut self, mut program: Node<Program>) -> crate::error::Result<Node<Program>> {
        for decl in &mut program.functions {
            self.labels.clear();
            debug_assert!(self.label_stack.is_empty());
            debug_assert!(self.switch_labels.is_empty());
            self.check_function_declaration(decl)?;
        }

        Ok(program)
    }
    fn check_statement(&mut self, stmt: &mut Node<Statement>) -> crate::error::Result<()> {
        match stmt.as_mut() {
            Statement::If {
                then_stmt,
                else_stmt,
                ..
            } => {
                self.check_statement(then_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.check_statement(else_stmt)?;
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
                self.check_statement(stmt)?;
            }
            Statement::Compound(block) => {
                self.check_block(block)?;
            }

            Statement::While { body, label, .. } | Statement::DoWhile { body, label, .. } => {
                *label = self.make_label("loop");
                self.label_stack.push_front(LabelKind::Loop(label.clone()));
                self.check_statement(body)?;
                self.label_stack.pop_front();
            }
            Statement::For { body, label, .. } => {
                *label = self.make_label("loop");
                self.label_stack.push_front(LabelKind::Loop(label.clone()));
                self.check_statement(body)?;
                self.label_stack.pop_front();
            }
            Statement::Switch { body, labels, .. } => {
                let new_labels = SwitchLabels {
                    label: self.make_label("switch"),
                    valued: vec![],
                    default: None,
                };
                self.label_stack
                    .push_front(LabelKind::Switch(new_labels.label.clone()));
                self.switch_labels.push_front(new_labels);
                self.check_statement(body)?;
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
                self.check_statement(body)?;
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
                self.check_statement(body)?;
            }

            Statement::Goto(_) => {
                // Note: checking if label exists is done in another pass
            }
            // TODO: Unify shape of return and expression in ast
            Statement::Return { .. } | Statement::Expression(..) | Statement::Null => {}
        }
        Ok(())
    }

    fn check_block(&mut self, block: &mut Block) -> crate::error::Result<()> {
        for block_item in &mut block.items {
            if let BlockItem::Stmt(stmt) = block_item {
                self.check_statement(stmt)?
            }
        }
        Ok(())
    }

    fn check_function_declaration(
        &mut self,
        decl: &mut FunctionDeclaration,
    ) -> crate::error::Result<()> {
        if let Some(body) = &mut decl.body {
            self.check_block(body)?;
            self.check_gotos_block(body)?;
        }
        Ok(())
    }

    fn check_gotos_block(&self, block: &mut Node<Block>) -> crate::error::Result<()> {
        for block_item in &mut block.items {
            if let BlockItem::Stmt(stmt) = block_item {
                self.check_gotos_stmt(stmt)?
            }
        }
        Ok(())
    }

    fn check_gotos_stmt(&self, stmt: &mut Node<Statement>) -> crate::error::Result<()> {
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

    fn make_label(&mut self, name: &str) -> Symbol {
        let unique_name = format!("{name}_{i}", i = self.counter);
        self.counter += 1;
        unique_name
    }
}

pub fn check(program: Node<Program>) -> crate::error::Result<Node<Program>> {
    LabelChecker::default().check(program)
}