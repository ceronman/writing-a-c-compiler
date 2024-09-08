use crate::ast;
use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Symbol,
    pub body: Vec<Instruction>,
}

#[derive(Debug)]
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
}

#[derive(Debug, Clone)]
pub enum Val {
    Constant(i64),
    Var(Symbol),
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
    Not,
    Increment,
    Decrement,
}

#[derive(Debug)]
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
    fn emit_body(mut self, body: &[ast::Node<ast::BlockItem>]) -> Vec<Instruction> {
        for block_item in body {
            match block_item.as_ref() {
                ast::BlockItem::Stmt(ast::Statement::Return { expr }) => {
                    let val = self.emit_expr(expr);
                    self.instructions.push(Instruction::Return(val));
                }
                ast::BlockItem::Stmt(ast::Statement::Expression(expr)) => {
                    self.emit_expr(expr);
                }
                ast::BlockItem::Decl(ast::Declaration {
                    name,
                    init: Some(init),
                }) => {
                    let result = self.emit_expr(init);
                    self.instructions.push(Instruction::Copy {
                        src: result,
                        dst: Val::Var(name.symbol.clone()),
                    });
                }

                _ => {}
            }
        }
        self.instructions
            .push(Instruction::Return(Val::Constant(0)));
        self.instructions
    }

    fn emit_expr(&mut self, expr: &ast::Expression) -> Val {
        match expr {
            ast::Expression::Constant(val) => Val::Constant(*val),
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
                        let end_label = self.make_label("end");
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
                        let end_label = self.make_label("end");
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
        }
    }

    fn make_temp(&mut self) -> Val {
        let tmp = Val::Var(format!("tmp.{i}", i = self.tmp_counter));
        self.tmp_counter += 1;
        tmp
    }

    fn make_label(&mut self, prefix: &str) -> Symbol {
        let result = format!("{prefix}{i}", i = self.label_counter);
        self.label_counter += 1;
        result
    }
}

pub fn emit(program: &ast::Program) -> Program {
    let function = &program.function_definition;
    Program {
        function: Function {
            name: function.name.symbol.clone(),
            body: TackyGenerator::default().emit_body(&function.body),
        },
    }
}
