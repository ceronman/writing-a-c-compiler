#[cfg(test)]
mod test;

use crate::ast;
use crate::semantic::{Attributes, InitialValue, SemanticData, StaticInit, SymbolData};
use crate::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Program {
    pub top_level: Vec<TopLevel>,
    pub semantics: SemanticData,
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
    pub ty: ast::Type,
    pub init: StaticInit,
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
    SignExtend {
        src: Val,
        dst: Val,
    },
    Truncate {
        src: Val,
        dst: Val,
    },
    ZeroExtend {
        src: Val,
        dst: Val,
    },
    DoubleToInt {
        src: Val,
        dst: Val,
    },
    DoubleToUInt {
        src: Val,
        dst: Val,
    },
    IntToDouble {
        src: Val,
        dst: Val,
    },
    UIntToDouble {
        src: Val,
        dst: Val,
    },
    GetAddress {
        src: Val,
        dst: Val,
    },
    Load {
        ptr: Val,
        dst: Val,
    },
    Store {
        src: Val,
        ptr: Val,
    },
}

pub type Constant = ast::Constant;
pub type Type = ast::Type;

#[derive(Debug, Clone)]
pub enum Val {
    Constant(Constant),
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

#[derive(Debug)]
enum ExprResult {
    Operand(Val),
    Dereference(Val),
}

struct TackyGenerator {
    semantics: SemanticData,
    instructions: Vec<Instruction>,
    tmp_counter: u32,
    label_counter: u32,
}

impl TackyGenerator {
    fn emit_instructions(&mut self, function: &ast::Block) -> Vec<Instruction> {
        self.emit_block(function);
        self.instructions
            .push(Instruction::Return(Val::Constant(Constant::Int(0))));
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
                    Val::Constant(Constant::Int(1))
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
            ast::Statement::Switch { expr, body, label } => {
                let cond = self.emit_expr(expr);
                let expr_ty = self.semantics.expr_type(expr).clone();
                let switch_cases = self.semantics.switch_cases(expr).clone();
                for (value, label) in &switch_cases.values {
                    let case_value = Val::Constant(value.clone());
                    let result = self.make_temp(&expr_ty);
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
                if let Some(label) = &switch_cases.default {
                    self.instructions.push(Instruction::Jump {
                        target: label.clone(),
                    })
                }
                let break_label = format!("break_{}", label);
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
    fn emit_expr(&mut self, expr: &ast::Node<ast::Expression>) -> Val {
        let expr_ty = self.semantics.expr_type(expr).clone();
        match self.expression(expr) {
            ExprResult::Operand(val) => self.cast_if_needed(val, expr, &expr_ty),
            ExprResult::Dereference(ptr) => {
                let dst = self.make_temp(&expr_ty);
                self.instructions.push(Instruction::Load {
                    ptr,
                    dst: dst.clone(),
                });
                self.cast_if_needed(dst, expr, &expr_ty)
            }
        }
    }

    fn expression(&mut self, expr: &ast::Node<ast::Expression>) -> ExprResult {
        let expr_ty = self.semantics.expr_type(expr).clone();
        let result = match expr.as_ref() {
            ast::Expression::Constant(value) => Val::Constant(value.clone()),
            ast::Expression::Unary { op, expr } => {
                let tacky_op = match op.as_ref() {
                    ast::UnaryOp::Complement => UnaryOp::Complement,
                    ast::UnaryOp::Negate => UnaryOp::Negate,
                    ast::UnaryOp::Not => UnaryOp::Not,
                    ast::UnaryOp::Increment => UnaryOp::Increment,
                    ast::UnaryOp::Decrement => UnaryOp::Decrement,
                };

                let lvalue = self.expression(expr);
                // TODO: This logic is repeated with emit_expr()
                let val = match &lvalue {
                    ExprResult::Operand(val) => self.cast_if_needed(val.clone(), expr, &expr_ty),
                    ExprResult::Dereference(ptr) => {
                        let dst = self.make_temp(&expr_ty);
                        self.instructions.push(Instruction::Load {
                            ptr: ptr.clone(),
                            dst: dst.clone(),
                        });
                        self.cast_if_needed(dst, expr, &expr_ty)
                    }
                };
                let dst = self.make_temp(&expr_ty);

                self.instructions.push(Instruction::Unary {
                    op: tacky_op,
                    src: val.clone(),
                    dst: dst.clone(),
                });
                if let ast::UnaryOp::Increment | ast::UnaryOp::Decrement = op.as_ref() {
                    // TODO: This logic is repeated with emit_expr()
                    match &lvalue {
                        ExprResult::Operand(val) => {
                            self.instructions.push(Instruction::Copy {
                                src: dst.clone(),
                                dst: val.clone(),
                            });
                        }
                        ExprResult::Dereference(ptr) => {
                            self.instructions.push(Instruction::Store {
                                src: dst.clone(),
                                ptr: ptr.clone(),
                            });
                        }
                    }
                }
                dst
            }
            ast::Expression::Postfix { op, expr } => {
                let lvalue = self.expression(expr);
                // TODO: This logic is repeated with emit_expr()
                let val = match &lvalue {
                    ExprResult::Operand(val) => self.cast_if_needed(val.clone(), expr, &expr_ty),
                    ExprResult::Dereference(ptr) => {
                        let dst = self.make_temp(&expr_ty);
                        self.instructions.push(Instruction::Load {
                            ptr: ptr.clone(),
                            dst: dst.clone(),
                        });
                        self.cast_if_needed(dst, expr, &expr_ty)
                    }
                };
                let dst = self.make_temp(&expr_ty);
                self.instructions.push(Instruction::Copy {
                    src: val.clone(),
                    dst: dst.clone(),
                });

                let tacky_op = match op.as_ref() {
                    ast::PostfixOp::Increment => UnaryOp::Increment,
                    ast::PostfixOp::Decrement => UnaryOp::Decrement,
                };

                let decremented = self.make_temp(&expr_ty);
                self.instructions.push(Instruction::Unary {
                    op: tacky_op,
                    src: val.clone(),
                    dst: decremented.clone(),
                });

                // TODO: This logic is repeated with emit_expr()
                match &lvalue {
                    ExprResult::Operand(val) => {
                        self.instructions.push(Instruction::Copy {
                            src: decremented.clone(),
                            dst: val.clone(),
                        });
                    }
                    ExprResult::Dereference(ptr) => {
                        self.instructions.push(Instruction::Store {
                            src: decremented.clone(),
                            ptr: ptr.clone(),
                        });
                    }
                }
                dst
            }
            ast::Expression::Binary { op, left, right } => {
                let src1 = self.emit_expr(left);
                let dst = self.make_temp(&expr_ty);
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
                        let result = self.make_temp(&expr_ty);
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
                            src: Val::Constant(Constant::Int(1)),
                            dst: result.clone(),
                        });
                        self.instructions.push(Instruction::Jump {
                            target: end_label.clone(),
                        });
                        self.instructions.push(Instruction::Label(false_label));
                        self.instructions.push(Instruction::Copy {
                            src: Val::Constant(Constant::Int(0)),
                            dst: result.clone(),
                        });
                        self.instructions.push(Instruction::Label(end_label));
                        return ExprResult::Operand(result);
                    }
                    ast::BinaryOp::Or => {
                        let result = self.make_temp(&expr_ty);
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
                            src: Val::Constant(Constant::Int(0)),
                            dst: result.clone(),
                        });
                        self.instructions.push(Instruction::Jump {
                            target: end_label.clone(),
                        });
                        self.instructions.push(Instruction::Label(true_label));
                        self.instructions.push(Instruction::Copy {
                            src: Val::Constant(Constant::Int(1)),
                            dst: result.clone(),
                        });
                        self.instructions.push(Instruction::Label(end_label));
                        return ExprResult::Operand(result);
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
                let lvalue = self.expression(left);

                let rvalue = if let Some(op) = op {
                    let left_ty = self.semantics.expr_type(left).clone();
                    // TODO: This logic is repeated with emit_expr()
                    let src1 = match &lvalue {
                        ExprResult::Operand(val) => {
                            self.cast_if_needed(val.clone(), left, &left_ty)
                        }
                        ExprResult::Dereference(ptr) => {
                            let dst = self.make_temp(&expr_ty);
                            self.instructions.push(Instruction::Load {
                                ptr: ptr.clone(),
                                dst: dst.clone(),
                            });
                            dst
                        }
                    };
                    let dst = self.make_temp(&expr_ty);
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

                let rvalue = self.cast_if_needed(rvalue, expr, &expr_ty);

                match &lvalue {
                    ExprResult::Operand(val) => {
                        self.instructions.push(Instruction::Copy {
                            src: rvalue,
                            dst: val.clone(),
                        });
                        return lvalue;
                    }
                    ExprResult::Dereference(ptr) => {
                        self.instructions.push(Instruction::Store {
                            src: rvalue.clone(),
                            ptr: ptr.clone(),
                        });
                        rvalue
                    }
                }
            }

            ast::Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                let cond_val = self.emit_expr(cond);
                let end_label = self.make_label("end_if");
                let else_label = self.make_label("else");
                let result = self.make_temp(&expr_ty);
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
                let result = self.make_temp(&expr_ty);
                self.instructions.push(Instruction::FnCall {
                    name: name.symbol.clone(),
                    args,
                    dst: result.clone(),
                });
                result
            }

            ast::Expression::Cast {
                target,
                expr: inner,
            } => {
                let result = self.emit_expr(inner);
                let inner_ty = self.semantics.expr_type(inner).clone();
                self.cast(result, &inner_ty, target)
            }

            ast::Expression::Dereference(inner) => {
                let result = self.emit_expr(inner);
                return ExprResult::Dereference(result);
            }
            ast::Expression::AddressOf(inner) => match self.expression(inner) {
                ExprResult::Operand(val) => {
                    let dst = self.make_temp(&expr_ty);
                    self.instructions.push(Instruction::GetAddress {
                        src: val,
                        dst: dst.clone(),
                    });
                    dst
                }
                ExprResult::Dereference(ptr) => ptr,
            },
        };
        ExprResult::Operand(result)
    }

    fn cast_if_needed(
        &mut self,
        val: Val,
        expr: &ast::Node<ast::Expression>,
        expr_ty: &Type,
    ) -> Val {
        if let Some(target) = self.semantics.implicit_casts.get(&expr.id).cloned() {
            self.cast(val, expr_ty, &target)
        } else {
            val
        }
    }

    fn cast(&mut self, src: Val, src_ty: &Type, target: &Type) -> Val {
        if target == src_ty {
            src
        } else {
            let dst = self.make_temp(target);

            if *src_ty == Type::Double {
                if target.is_signed() {
                    self.instructions.push(Instruction::DoubleToInt {
                        src,
                        dst: dst.clone(),
                    });
                } else {
                    self.instructions.push(Instruction::DoubleToUInt {
                        src,
                        dst: dst.clone(),
                    });
                }
            } else if *target == Type::Double {
                if src_ty.is_signed() {
                    self.instructions.push(Instruction::IntToDouble {
                        src,
                        dst: dst.clone(),
                    });
                } else {
                    self.instructions.push(Instruction::UIntToDouble {
                        src,
                        dst: dst.clone(),
                    });
                }
            } else if target.size() == src_ty.size() {
                self.instructions.push(Instruction::Copy {
                    src,
                    dst: dst.clone(),
                });
            } else if target.size() < src_ty.size() {
                self.instructions.push(Instruction::Truncate {
                    src,
                    dst: dst.clone(),
                });
            } else if src_ty.is_signed() {
                self.instructions.push(Instruction::SignExtend {
                    src,
                    dst: dst.clone(),
                });
            } else {
                self.instructions.push(Instruction::ZeroExtend {
                    src,
                    dst: dst.clone(),
                });
            }
            dst
        }
    }

    fn make_temp(&mut self, ty: &Type) -> Val {
        let name = format!("tmp.{i}", i = self.tmp_counter);
        let tmp = Val::Var(name.clone());
        self.semantics.symbols.insert(
            name,
            SymbolData {
                ty: ty.clone(),
                attrs: Attributes::Local,
            },
        );
        self.tmp_counter += 1;
        tmp
    }

    fn make_label(&mut self, prefix: &str) -> Symbol {
        let result = format!("{prefix}_{i}", i = self.label_counter);
        self.label_counter += 1;
        result
    }
}

pub fn emit(program: &ast::Program, semantics: SemanticData) -> Program {
    let mut top_level = Vec::new();
    let mut generator = TackyGenerator {
        semantics,
        instructions: vec![],
        tmp_counter: 0,
        label_counter: 0,
    };
    for decl in &program.declarations {
        generator.instructions.clear();
        match decl.as_ref() {
            ast::Declaration::Function(function) => {
                let name = function.name.symbol.clone();
                let symbol_data = generator
                    .semantics
                    .symbols
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
            ast::Declaration::Var(_) => {}
        }
    }

    for (name, symbol_data) in &generator.semantics.symbols {
        if let Attributes::Static {
            initial_value,
            global,
        } = symbol_data.attrs
        {
            let ty = symbol_data.ty.clone();
            match initial_value {
                InitialValue::Initial(init) => top_level.push(TopLevel::Variable(StaticVariable {
                    name: name.clone(),
                    ty,
                    global,
                    init,
                })),
                InitialValue::Tentative => {
                    let init = match &ty {
                        Type::Int => StaticInit::Int(0),
                        Type::Long => StaticInit::Long(0),
                        Type::UInt => StaticInit::UInt(0),
                        Type::ULong => StaticInit::ULong(0),
                        Type::Double => StaticInit::Double(0.0),
                        Type::Pointer(_) => StaticInit::ULong(0),
                        Type::Function(_) => unreachable!(),
                    };
                    top_level.push(TopLevel::Variable(StaticVariable {
                        name: name.clone(),
                        ty,
                        global,
                        init,
                    }))
                }
                InitialValue::NoInitializer => continue,
            }
        }
    }

    Program {
        top_level,
        semantics: generator.semantics,
    }
}
