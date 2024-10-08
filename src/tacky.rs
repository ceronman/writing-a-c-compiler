#[cfg(test)]
mod test;

use crate::ast;
use crate::ast::Declaration;
use crate::semantic::{Attributes, InitialValue, SymbolTable};
use crate::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Program {
    pub top_level: Vec<TopLevel>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Function(Function),
    Variable(StaticVariable),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Symbol,
    pub global: bool,
    pub params: Vec<Symbol>,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct StaticVariable {
    pub name: Symbol,
    pub global: bool,
    pub init: i64,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Return(Val),
    Unary {
        op: UnaryOp,
        src: Val,
        dst: Val,
    },
    Binary {
        op: BinaryOp,
        src1: Val,
        src2: Val,
        dst: Val,
    },
    Copy {
        src: Val,
        dst: Val,
    },
    Jump {
        target: Symbol,
    },
    JumpIfZero {
        cond: Val,
        target: Symbol,
    },
    JumpIfNotZero {
        cond: Val,
        target: Symbol,
    },
    Label(Symbol),
    FnCall {
        name: Symbol,
        args: Vec<Val>,
        dst: Val,
    },
}

#[derive(Debug, Clone)]
pub enum Val {
    Constant(i64),
    Var(Symbol),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Complement,
    Negate,
    Not,
    Increment,
    Decrement,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Reminder,
    BinAnd,
    BinOr,
    BinXor,
    ShiftLeft,
    ShiftRight,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

#[derive(Default)]
struct TackyGenerator {
    instructions: Vec<Instruction>,
    tmp_counter: u32,
    label_counter: u32,
}

impl TackyGenerator {
    fn emit_instructions(&mut self, function: &ast::Block) -> Vec<Instruction> {
        self.emit_block(function);
        self.instructions
            .push(Instruction::Return(Val::Constant(0)));
        self.instructions.clone()
    }

    fn emit_block(&mut self, block: &ast::Block) {
        for block_item in &block.items {
            match block_item {
                ast::BlockItem::Stmt(stmt) => self.emit_statement(stmt),
                ast::BlockItem::Decl(decl) => match decl.as_ref() {
                    ast::Declaration::Var(decl) => self.emit_var_declaration(decl),
                    ast::Declaration::Function(_) => {}
                },
            }
        }
    }

    fn emit_var_declaration(&mut self, decl: &ast::VarDeclaration) {
        if decl.storage_class.is_some() {
            return;
        }
        if let Some(init) = &decl.init {
            let result = self.emit_expr(init);
            self.instructions.push(Instruction::Copy {
                src: result,
                dst: Val::Var(decl.name.symbol.clone()),
            });
        }
    }

    fn emit_statement(&mut self, stmt: &ast::Statement) {
        match stmt {
            ast::Statement::Return(expr) => {
                let val = self.emit_expr(expr);
                self.instructions.push(Instruction::Return(val));
            }
            ast::Statement::Expression(expr) => {
                self.emit_expr(expr);
            }
            ast::Statement::If {
                cond,
                then_stmt,
                else_stmt,
            } => {
                let cond_val = self.emit_expr(cond);
                let end_label = self.make_label("end_if");
                let else_label = self.make_label("else");

                match else_stmt {
                    Some(else_stmt) => {
                        self.instructions.push(Instruction::JumpIfZero {
                            cond: cond_val,
                            target: else_label.clone(),
                        });
                        self.emit_statement(then_stmt);
                        self.instructions.push(Instruction::Jump {
                            target: end_label.clone(),
                        });
                        self.instructions.push(Instruction::Label(else_label));
                        self.emit_statement(else_stmt);
                    }
                    None => {
                        self.instructions.push(Instruction::JumpIfZero {
                            cond: cond_val,
                            target: end_label.clone(),
                        });
                        self.emit_statement(then_stmt);
                    }
                }
                self.instructions.push(Instruction::Label(end_label));
            }

            ast::Statement::Labeled { name, body: stmt } => {
                self.instructions
                    .push(Instruction::Label(name.symbol.clone()));
                self.emit_statement(stmt);
            }

            ast::Statement::Goto(label) => {
                self.instructions.push(Instruction::Jump {
                    target: label.symbol.clone(),
                });
            }

            ast::Statement::Compound(block) => self.emit_block(block),

            ast::Statement::Null => {}

            ast::Statement::DoWhile { cond, body, label } => {
                let start_label = format!("start_{label}");
                self.instructions
                    .push(Instruction::Label(start_label.clone()));
                self.emit_statement(body);
                self.instructions
                    .push(Instruction::Label(format!("continue_{label}")));
                let cond = self.emit_expr(cond);
                self.instructions.push(Instruction::JumpIfNotZero {
                    cond,
                    target: start_label,
                });
                self.instructions
                    .push(Instruction::Label(format!("break_{label}")));
            }
            ast::Statement::While { cond, body, label } => {
                let continue_label = format!("continue_{label}");
                let break_label = format!("break_{label}");
                self.instructions
                    .push(Instruction::Label(continue_label.clone()));
                let cond = self.emit_expr(cond);
                self.instructions.push(Instruction::JumpIfZero {
                    cond,
                    target: break_label.clone(),
                });
                self.emit_statement(body);
                self.instructions.push(Instruction::Jump {
                    target: continue_label,
                });
                self.instructions.push(Instruction::Label(break_label));
            }
            ast::Statement::For {
                init,
                cond,
                post,
                body,
                label,
            } => {
                match init {
                    ast::ForInit::Decl(decl) => self.emit_var_declaration(decl),
                    ast::ForInit::Expr(expr) => {
                        self.emit_expr(expr);
                    }
                    ast::ForInit::None => {}
                }
                let start_label = format!("start_{label}");
                self.instructions
                    .push(Instruction::Label(start_label.clone()));
                let cond = if let Some(cond) = cond {
                    self.emit_expr(cond)
                } else {
                    Val::Constant(1)
                };
                let break_label = format!("break_{label}");
                self.instructions.push(Instruction::JumpIfZero {
                    cond,
                    target: break_label.clone(),
                });
                self.emit_statement(body);
                self.instructions
                    .push(Instruction::Label(format!("continue_{label}")));
                if let Some(post) = post {
                    self.emit_expr(post);
                }
                self.instructions.push(Instruction::Jump {
                    target: start_label,
                });
                self.instructions.push(Instruction::Label(break_label));
            }
            ast::Statement::Switch { expr, body, labels } => {
                let cond = self.emit_expr(expr);
                for (value, label) in &labels.valued {
                    let case_value = Val::Constant(*value);
                    let result = self.make_temp();
                    self.instructions.push(Instruction::Binary {
                        op: BinaryOp::Equal,
                        src1: cond.clone(),
                        src2: case_value,
                        dst: result.clone(),
                    });
                    self.instructions.push(Instruction::JumpIfNotZero {
                        cond: result,
                        target: label.clone(),
                    })
                }
                if let Some(label) = &labels.default {
                    self.instructions.push(Instruction::Jump {
                        target: label.clone(),
                    })
                }
                let break_label = format!("break_{}", labels.label);
                self.instructions.push(Instruction::Jump {
                    target: break_label.clone(),
                });
                self.emit_statement(body);
                self.instructions.push(Instruction::Label(break_label))
            }
            ast::Statement::Break(label) => {
                let target = format!("break_{label}");
                self.instructions.push(Instruction::Jump { target });
            }
            ast::Statement::Continue(label) => {
                let target = format!("continue_{label}");
                self.instructions.push(Instruction::Jump { target });
            }
            ast::Statement::Case { label, body, .. } | ast::Statement::Default { label, body } => {
                self.instructions.push(Instruction::Label(label.clone()));
                self.emit_statement(body);
            }
        }
    }

    fn emit_expr(&mut self, expr: &ast::Expression) -> Val {
        match expr {
            ast::Expression::Constant(c) => {
                let value = match c {
                    ast::Constant::Int(v) => *v as i64,
                    ast::Constant::Long(v) => *v,
                };
                Val::Constant(value)
            }
            ast::Expression::Unary { op, expr } => {
                let val = self.emit_expr(expr);
                let dst = self.make_temp();
                let tacky_op = match op.as_ref() {
                    ast::UnaryOp::Complement => UnaryOp::Complement,
                    ast::UnaryOp::Negate => UnaryOp::Negate,
                    ast::UnaryOp::Not => UnaryOp::Not,
                    ast::UnaryOp::Increment => UnaryOp::Increment,
                    ast::UnaryOp::Decrement => UnaryOp::Decrement,
                };
                self.instructions.push(Instruction::Unary {
                    op: tacky_op,
                    src: val.clone(),
                    dst: dst.clone(),
                });
                if let ast::UnaryOp::Increment | ast::UnaryOp::Decrement = op.as_ref() {
                    self.instructions.push(Instruction::Copy {
                        src: dst.clone(),
                        dst: val,
                    })
                }
                dst
            }
            ast::Expression::Postfix { op, expr } => {
                let val = self.emit_expr(expr);
                let dst = self.make_temp();
                self.instructions.push(Instruction::Copy {
                    src: val.clone(),
                    dst: dst.clone(),
                });

                let tacky_op = match op.as_ref() {
                    ast::PostfixOp::Increment => UnaryOp::Increment,
                    ast::PostfixOp::Decrement => UnaryOp::Decrement,
                };

                let decremented = self.make_temp();
                self.instructions.push(Instruction::Unary {
                    op: tacky_op,
                    src: val.clone(),
                    dst: decremented.clone(),
                });
                self.instructions.push(Instruction::Copy {
                    src: decremented,
                    dst: val,
                });
                dst
            }
            ast::Expression::Binary { op, left, right } => {
                let src1 = self.emit_expr(left);
                let dst = self.make_temp();
                let op = match op.as_ref() {
                    ast::BinaryOp::Add => BinaryOp::Add,
                    ast::BinaryOp::Subtract => BinaryOp::Subtract,
                    ast::BinaryOp::Multiply => BinaryOp::Multiply,
                    ast::BinaryOp::Divide => BinaryOp::Divide,
                    ast::BinaryOp::Reminder => BinaryOp::Reminder,
                    ast::BinaryOp::BinAnd => BinaryOp::BinAnd,
                    ast::BinaryOp::BinOr => BinaryOp::BinOr,
                    ast::BinaryOp::BinXor => BinaryOp::BinXor,
                    ast::BinaryOp::ShiftLeft => BinaryOp::ShiftLeft,
                    ast::BinaryOp::ShiftRight => BinaryOp::ShiftRight,
                    ast::BinaryOp::Equal => BinaryOp::Equal,
                    ast::BinaryOp::NotEqual => BinaryOp::NotEqual,
                    ast::BinaryOp::LessThan => BinaryOp::LessThan,
                    ast::BinaryOp::LessOrEqualThan => BinaryOp::LessOrEqual,
                    ast::BinaryOp::GreaterThan => BinaryOp::GreaterThan,
                    ast::BinaryOp::GreaterOrEqualThan => BinaryOp::GreaterOrEqual,
                    ast::BinaryOp::And => {
                        let result = self.make_temp();
                        let false_label = self.make_label("and_false");
                        let end_label = self.make_label("and_end");
                        self.instructions.push(Instruction::JumpIfZero {
                            cond: src1,
                            target: false_label.clone(),
                        });
                        let src2 = self.emit_expr(right);
                        self.instructions.push(Instruction::JumpIfZero {
                            cond: src2,
                            target: false_label.clone(),
                        });
                        self.instructions.push(Instruction::Copy {
                            src: Val::Constant(1),
                            dst: result.clone(),
                        });
                        self.instructions.push(Instruction::Jump {
                            target: end_label.clone(),
                        });
                        self.instructions.push(Instruction::Label(false_label));
                        self.instructions.push(Instruction::Copy {
                            src: Val::Constant(0),
                            dst: result.clone(),
                        });
                        self.instructions.push(Instruction::Label(end_label));
                        return result;
                    }
                    ast::BinaryOp::Or => {
                        let result = self.make_temp();
                        let true_label = self.make_label("or_true");
                        let end_label = self.make_label("or_end");
                        self.instructions.push(Instruction::JumpIfNotZero {
                            cond: src1,
                            target: true_label.clone(),
                        });
                        let src2 = self.emit_expr(right);
                        self.instructions.push(Instruction::JumpIfNotZero {
                            cond: src2,
                            target: true_label.clone(),
                        });
                        self.instructions.push(Instruction::Copy {
                            src: Val::Constant(0),
                            dst: result.clone(),
                        });
                        self.instructions.push(Instruction::Jump {
                            target: end_label.clone(),
                        });
                        self.instructions.push(Instruction::Label(true_label));
                        self.instructions.push(Instruction::Copy {
                            src: Val::Constant(1),
                            dst: result.clone(),
                        });
                        self.instructions.push(Instruction::Label(end_label));
                        return result;
                    }
                };
                let src2 = self.emit_expr(right);
                self.instructions.push(Instruction::Binary {
                    op,
                    src1,
                    src2,
                    dst: dst.clone(),
                });
                dst
            }

            ast::Expression::Var(name) => Val::Var(name.clone()),
            ast::Expression::Assignment { op, left, right } => {
                let ast::Expression::Var(name) = left.as_ref() else {
                    unreachable!()
                };
                let op = match op.as_ref() {
                    ast::AssignOp::Equal => None,
                    ast::AssignOp::AddEqual => Some(BinaryOp::Add),
                    ast::AssignOp::SubEqual => Some(BinaryOp::Subtract),
                    ast::AssignOp::MulEqual => Some(BinaryOp::Multiply),
                    ast::AssignOp::DivEqual => Some(BinaryOp::Divide),
                    ast::AssignOp::ModEqual => Some(BinaryOp::Reminder),
                    ast::AssignOp::BitAndEqual => Some(BinaryOp::BinAnd),
                    ast::AssignOp::BitOrEqual => Some(BinaryOp::BinOr),
                    ast::AssignOp::BitXorEqual => Some(BinaryOp::BinXor),
                    ast::AssignOp::ShiftLeftEqual => Some(BinaryOp::ShiftLeft),
                    ast::AssignOp::ShiftRightEqual => Some(BinaryOp::ShiftRight),
                };

                let result = if let Some(op) = op {
                    let src1 = self.emit_expr(left);
                    let dst = self.make_temp();
                    let src2 = self.emit_expr(right);
                    self.instructions.push(Instruction::Binary {
                        op,
                        src1,
                        src2,
                        dst: dst.clone(),
                    });
                    dst
                } else {
                    self.emit_expr(right)
                };

                self.instructions.push(Instruction::Copy {
                    src: result,
                    dst: Val::Var(name.clone()),
                });
                Val::Var(name.clone())
            }

            ast::Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                let cond_val = self.emit_expr(cond);
                let end_label = self.make_label("end_if");
                let else_label = self.make_label("else");
                let result = self.make_temp();
                self.instructions.push(Instruction::JumpIfZero {
                    cond: cond_val,
                    target: else_label.clone(),
                });
                let true_value = self.emit_expr(then_expr);
                self.instructions.push(Instruction::Copy {
                    src: true_value,
                    dst: result.clone(),
                });
                self.instructions.push(Instruction::Jump {
                    target: end_label.clone(),
                });
                self.instructions.push(Instruction::Label(else_label));
                let false_value = self.emit_expr(else_expr);
                self.instructions.push(Instruction::Copy {
                    src: false_value,
                    dst: result.clone(),
                });
                self.instructions.push(Instruction::Label(end_label));
                result
            }

            ast::Expression::FunctionCall { name, args } => {
                let args: Vec<Val> = args.iter().map(|a| self.emit_expr(a)).collect();
                let result = self.make_temp();
                self.instructions.push(Instruction::FnCall {
                    name: name.symbol.clone(),
                    args,
                    dst: result.clone(),
                });
                result
            }

            ast::Expression::Cast { .. } => todo!(),
        }
    }

    fn make_temp(&mut self) -> Val {
        let tmp = Val::Var(format!("tmp.{i}", i = self.tmp_counter));
        self.tmp_counter += 1;
        tmp
    }

    fn make_label(&mut self, prefix: &str) -> Symbol {
        let result = format!("{prefix}_{i}", i = self.label_counter);
        self.label_counter += 1;
        result
    }
}

pub fn emit(program: &ast::Program, symbol_table: &SymbolTable) -> Program {
    let mut top_level = Vec::new();
    let mut generator = TackyGenerator::default();
    for decl in &program.declarations {
        generator.instructions.clear();
        match decl.as_ref() {
            Declaration::Function(function) => {
                let name = function.name.symbol.clone();
                let symbol_data = symbol_table
                    .get(&name)
                    .expect("Function without symbol data");
                let Attributes::Function { global, .. } = symbol_data.attrs else {
                    panic!("Function with incorrect symbol attributes");
                };
                let Some(body) = &function.body else { continue };
                top_level.push(TopLevel::Function(Function {
                    name,
                    global,
                    params: function.params.iter().map(|i| i.symbol.clone()).collect(),
                    body: generator.emit_instructions(body),
                }));
            }
            Declaration::Var(_) => {}
        }
    }

    for (name, symbol_data) in symbol_table {
        if let Attributes::Static {
            initial_value,
            global,
        } = symbol_data.attrs
        {
            match initial_value {
                InitialValue::Initial(init) => top_level.push(TopLevel::Variable(StaticVariable {
                    name: name.clone(),
                    global,
                    init,
                })),
                InitialValue::Tentative => top_level.push(TopLevel::Variable(StaticVariable {
                    name: name.clone(),
                    global,
                    init: 0,
                })),
                InitialValue::NoInitializer => continue,
            }
        }
    }

    Program { top_level }
}
