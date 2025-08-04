pub mod pretty;

#[cfg(test)]
mod test;

use crate::ast;
use crate::semantic::{Attributes, InitialValue, SemanticData, StaticInit, SymbolData, Type};
use crate::symbol::Symbol;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Program {
    pub top_level: Vec<TopLevel>,
    pub semantics: SemanticData,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Function(Function),
    Variable(StaticVariable),
    Constant(StaticConstant),
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
    pub ty: Type,
    pub init: Vec<StaticInit>,
}

#[derive(Debug, Clone)]
pub struct StaticConstant {
    pub name: Symbol,
    pub ty: Type,
    pub init: StaticInit,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Return(Option<Val>),
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
        dst: Option<Val>,
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
    AddPtr {
        ptr: Val,
        index: Val,
        scale: usize,
        dst: Val,
    },
    CopyToOffset {
        src: Val,
        dst: Symbol,
        offset: i64,
    },
}

pub type Constant = ast::Constant;

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
    strings: HashMap<Symbol, Symbol>,
    instructions: Vec<Instruction>,
    tmp_counter: u32,
    label_counter: u32,
}

impl TackyGenerator {
    fn emit_instructions(&mut self, function: &ast::Block) -> Vec<Instruction> {
        self.emit_block(function);
        self.instructions
            .push(Instruction::Return(Some(Val::Constant(Constant::Int(0)))));
        self.instructions.clone()
    }

    fn emit_block(&mut self, block: &ast::Block) {
        for block_item in &block.items {
            match block_item {
                ast::BlockItem::Stmt(stmt) => self.emit_statement(stmt),
                ast::BlockItem::Decl(decl) => match decl.as_ref() {
                    ast::Declaration::Var(decl) => self.emit_var_declaration(decl),
                    ast::Declaration::Function(_) => {}
                    ast::Declaration::Struct(_) => todo!()
                },
            }
        }
    }

    fn emit_var_declaration(&mut self, decl: &ast::VarDeclaration) {
        if decl.storage_class.is_some() {
            return;
        }
        if let Some(init) = &decl.init {
            self.emit_initializer(0, 0, &decl.name.symbol, init, &decl.ty.to_semantic());
        }
    }

    fn emit_initializer(
        &mut self,
        level: usize,
        offset: usize,
        name: &Symbol,
        initializer: &ast::Initializer,
        ty: &Type,
    ) {
        match initializer {
            ast::Initializer::Single(init) => {
                if let ast::Expression::String(s) = init.as_ref()
                    && ty.is_array()
                {
                    let Type::Array(inner, len) = ty else {
                        panic!("String initializer used with non-array type");
                    };
                    for (i, c) in s.chars().enumerate() {
                        self.instructions.push(Instruction::CopyToOffset {
                            src: Val::Constant(Constant::from_char(c, inner)),
                            dst: name.clone(),
                            offset: (offset + i) as i64,
                        });
                    }
                    let padding = *len - s.len();
                    for i in 0..padding {
                        self.instructions.push(Instruction::CopyToOffset {
                            src: Val::Constant(Constant::Char(0)),
                            dst: name.clone(),
                            offset: (offset + s.len() + i) as i64,
                        });
                    }
                } else {
                    let result = self.emit_expr(init);
                    if level == 0 {
                        self.instructions.push(Instruction::Copy {
                            src: result,
                            dst: Val::Var(name.clone()),
                        });
                    } else {
                        self.instructions.push(Instruction::CopyToOffset {
                            src: result,
                            dst: name.clone(),
                            offset: offset as i64,
                        });
                    }
                }
            }
            ast::Initializer::Compound(initializers) => {
                let Type::Array(inner, len) = ty else {
                    panic!("Compound initializer used with a non-array type")
                };
                for i in 0..*len {
                    let size = inner.size();
                    if let Some(initializer) = initializers.get(i) {
                        self.emit_initializer(
                            level + 1,
                            offset + i * size,
                            name,
                            initializer,
                            inner,
                        )
                    } else {
                        self.emit_zero_initializer(offset + i * size, name, inner)
                    }
                }
            }
        }
    }

    fn emit_zero_initializer(&mut self, offset: usize, name: &Symbol, ty: &Type) {
        let constant = match ty {
            Type::Char | Type::SChar => Constant::Char(0),
            Type::UChar => Constant::UChar(0),
            Type::Int => Constant::Int(0),
            Type::UInt => Constant::UInt(0),
            Type::Long => Constant::Long(0),
            Type::ULong => Constant::ULong(0),
            Type::Double => Constant::Double(0.0),
            Type::Pointer(_) => Constant::ULong(0),
            Type::Void | Type::Function(_) => panic!("Zero initializer for invalid type"),
            Type::Array(inner, size) => {
                let ty_size = inner.size();
                for i in 0..*size {
                    self.emit_zero_initializer(offset + i * ty_size, name, inner)
                }
                return;
            }
            Type::Struct(_) => todo!()
        };
        self.instructions.push(Instruction::CopyToOffset {
            src: Val::Constant(constant),
            dst: name.clone(),
            offset: offset as i64,
        });
    }

    fn emit_statement(&mut self, stmt: &ast::Statement) {
        match stmt {
            ast::Statement::Return(expr) => {
                if let Some(expr) = expr {
                    let val = self.emit_expr(expr);
                    self.instructions.push(Instruction::Return(Some(val)));
                } else {
                    self.instructions.push(Instruction::Return(None));
                }
            }
            ast::Statement::Expression(expr) => {
                self.emit_expr(expr);
            }
            ast::Statement::If {
                cond,
                then_stmt,
                else_stmt,
            } => {
                let cond_val = self.make_cond(cond);
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
                let cond_val = self.make_cond(cond);

                self.instructions.push(Instruction::JumpIfNotZero {
                    cond: cond_val,
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
                let cond_val = self.make_cond(cond);
                self.instructions.push(Instruction::JumpIfZero {
                    cond: cond_val,
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
                let cond_val = if let Some(cond) = cond {
                    self.make_cond(cond)
                } else {
                    Val::Constant(Constant::Int(1))
                };
                let break_label = format!("break_{label}");
                self.instructions.push(Instruction::JumpIfZero {
                    cond: cond_val,
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
                        src1: case_value,
                        src2: cond.clone(),
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
                let break_label = format!("break_{label}");
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

    // Hack to deal with evaluating double conditionals that support NaN.
    fn make_cond(&mut self, cond: &ast::Node<ast::Expression>) -> Val {
        let cond_val = self.emit_expr(cond);
        let cond_ty = self.semantics.expr_type(cond).clone();
        if cond_ty.is_double() {
            let dst = self.make_temp(&Type::Int);
            self.instructions.push(Instruction::Binary {
                op: BinaryOp::NotEqual,
                src1: cond_val,
                src2: Val::Constant(Constant::Double(0.0)),
                dst: dst.clone(),
            });
            dst
        } else {
            cond_val
        }
    }

    fn emit_expr(&mut self, expr: &ast::Node<ast::Expression>) -> Val {
        let expr_ty = self.semantics.expr_type(expr).clone();
        match self.expression(expr) {
            ExprResult::Operand(val) => self.cast_if_needed(val, expr),
            ExprResult::Dereference(ptr) => {
                if self.semantics.pointer_decays.contains_key(&expr.id) {
                    ptr
                } else {
                    let dst = self.make_temp(&expr_ty);
                    self.instructions.push(Instruction::Load {
                        ptr,
                        dst: dst.clone(),
                    });
                    self.cast_if_needed(dst, expr)
                }
            }
        }
    }

    fn expression(&mut self, expr: &ast::Node<ast::Expression>) -> ExprResult {
        let expr_ty = self.semantics.expr_type(expr).clone();
        let result = match expr.as_ref() {
            ast::Expression::Constant(value) => Val::Constant(value.clone()),
            ast::Expression::String(s) => self.make_string_const(s),
            ast::Expression::Var(name) => Val::Var(name.clone()),
            ast::Expression::Unary { op, expr } => {
                let lvalue = self.expression(expr);
                let val = self.get_or_load(&lvalue, expr);
                let dst = self.make_temp(&expr_ty);

                if let Type::Pointer(inner) = self.semantics.expr_type(expr).clone()
                    && let ast::UnaryOp::Decrement | ast::UnaryOp::Increment = op.as_ref()
                {
                    let index = match op.as_ref() {
                        ast::UnaryOp::Increment => 1,
                        ast::UnaryOp::Decrement => -1,
                        _ => unreachable!(),
                    };
                    let index = Val::Constant(Constant::Long(index));
                    let scale = inner.size();
                    self.instructions.push(Instruction::AddPtr {
                        ptr: val,
                        index,
                        scale,
                        dst: dst.clone(),
                    });
                } else {
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
                }

                if let ast::UnaryOp::Increment | ast::UnaryOp::Decrement = op.as_ref() {
                    self.copy_or_store(&lvalue, dst.clone());
                }
                dst
            }
            ast::Expression::Postfix { op, expr } => {
                let lvalue = self.expression(expr);
                let val = self.get_or_load(&lvalue, expr);
                let dst = self.make_temp(&expr_ty);
                self.instructions.push(Instruction::Copy {
                    src: val.clone(),
                    dst: dst.clone(),
                });
                let decremented = self.make_temp(&expr_ty);

                if let Type::Pointer(inner) = self.semantics.expr_type(expr).clone() {
                    let index = match op.as_ref() {
                        ast::PostfixOp::Increment => 1,
                        ast::PostfixOp::Decrement => -1,
                    };
                    let index = Val::Constant(Constant::Long(index));
                    let scale = inner.size();
                    self.instructions.push(Instruction::AddPtr {
                        ptr: val,
                        index,
                        scale,
                        dst: decremented.clone(),
                    });
                } else {
                    let tacky_op = match op.as_ref() {
                        ast::PostfixOp::Increment => UnaryOp::Increment,
                        ast::PostfixOp::Decrement => UnaryOp::Decrement,
                    };

                    self.instructions.push(Instruction::Unary {
                        op: tacky_op,
                        src: val.clone(),
                        dst: decremented.clone(),
                    });
                }

                self.copy_or_store(&lvalue, decremented.clone());
                dst
            }
            ast::Expression::Binary { op, left, right } => {
                let src1 = self.emit_expr(left);
                let dst = self.make_temp(&expr_ty);
                let op = match op.as_ref() {
                    ast::BinaryOp::Add => {
                        let left_ty = self.semantics.expr_type(left).clone();
                        let right_ty = self.semantics.expr_type(right).clone();
                        if let Type::Pointer(inner) = left_ty {
                            let ptr = src1;
                            let index = self.emit_expr(right);
                            let scale = inner.size();
                            self.instructions.push(Instruction::AddPtr {
                                ptr,
                                index,
                                scale,
                                dst: dst.clone(),
                            });
                            return ExprResult::Operand(dst);
                        } else if let Type::Pointer(inner) = right_ty {
                            let ptr = self.emit_expr(right);
                            let index = src1;
                            let scale = inner.size();
                            self.instructions.push(Instruction::AddPtr {
                                ptr,
                                index,
                                scale,
                                dst: dst.clone(),
                            });
                            return ExprResult::Operand(dst);
                        } else {
                            BinaryOp::Add
                        }
                    }
                    ast::BinaryOp::Subtract => {
                        let left_ty = self.semantics.expr_type(left).clone();
                        let right_ty = self.semantics.expr_type(right).clone();
                        if let (Type::Pointer(inner1), Type::Pointer(inner2)) =
                            (&left_ty, &right_ty)
                        {
                            assert_eq!(inner1, inner2);
                            let src2 = self.emit_expr(right);
                            let diff = self.make_temp(&Type::Long);
                            self.instructions.push(Instruction::Binary {
                                op: BinaryOp::Subtract,
                                src1,
                                src2,
                                dst: diff.clone(),
                            });

                            let size = Val::Constant(Constant::Long(inner1.size() as i64));
                            self.instructions.push(Instruction::Binary {
                                op: BinaryOp::Divide,
                                src1: diff,
                                src2: size,
                                dst: dst.clone(),
                            });
                            return ExprResult::Operand(dst);
                        } else if let Type::Pointer(inner) = left_ty {
                            let ptr = src1;
                            let index = self.emit_expr(right);
                            let negated = self.make_temp(&Type::Long);
                            self.instructions.push(Instruction::Unary {
                                op: UnaryOp::Negate,
                                src: index,
                                dst: negated.clone(),
                            });
                            let scale = inner.size();
                            self.instructions.push(Instruction::AddPtr {
                                ptr,
                                index: negated,
                                scale,
                                dst: dst.clone(),
                            });
                            return ExprResult::Operand(dst);
                        } else {
                            BinaryOp::Subtract
                        }
                    }
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
                    let src1 = self.get_or_load(&lvalue, left);
                    let dst = self.make_temp(&expr_ty);
                    let src2 = self.emit_expr(right);
                    if let Type::Pointer(inner) = self.semantics.expr_type(left).clone() {
                        if let BinaryOp::Add | BinaryOp::Subtract = op {
                            let index = match op {
                                BinaryOp::Add => src2,
                                BinaryOp::Subtract => {
                                    let negated = self.make_temp(&Type::Long);
                                    self.instructions.push(Instruction::Unary {
                                        op: UnaryOp::Negate,
                                        src: src2,
                                        dst: negated.clone(),
                                    });
                                    negated.clone()
                                }
                                _ => unreachable!(),
                            };
                            let scale = inner.size();
                            self.instructions.push(Instruction::AddPtr {
                                ptr: src1,
                                index,
                                scale,
                                dst: dst.clone(),
                            });
                        }
                    } else {
                        self.instructions.push(Instruction::Binary {
                            op,
                            src1,
                            src2,
                            dst: dst.clone(),
                        });
                    }
                    dst
                } else {
                    self.emit_expr(right)
                };
                let rvalue = self.cast_if_needed(rvalue, expr);
                self.copy_or_store(&lvalue, rvalue.clone());
                rvalue
            }

            ast::Expression::Conditional {
                cond,
                then_expr,
                else_expr,
            } => {
                let cond_val = self.make_cond(cond);
                let end_label = self.make_label("end_if");
                let else_label = self.make_label("else");
                let true_ty = self.semantics.expr_type(then_expr).clone();
                let result = if true_ty.is_void() {
                    Val::Var("DUMMY".into())
                } else {
                    self.make_temp(&expr_ty)
                };
                self.instructions.push(Instruction::JumpIfZero {
                    cond: cond_val,
                    target: else_label.clone(),
                });
                let true_value = self.emit_expr(then_expr);
                if !true_ty.is_void() {
                    self.instructions.push(Instruction::Copy {
                        src: true_value,
                        dst: result.clone(),
                    });
                }
                self.instructions.push(Instruction::Jump {
                    target: end_label.clone(),
                });
                self.instructions.push(Instruction::Label(else_label));
                let false_value = self.emit_expr(else_expr);
                let else_ty = self.semantics.expr_type(else_expr).clone();
                if !else_ty.is_void() {
                    self.instructions.push(Instruction::Copy {
                        src: false_value,
                        dst: result.clone(),
                    });
                }
                self.instructions.push(Instruction::Label(end_label));
                result
            }

            ast::Expression::FunctionCall { name, args } => {
                let args: Vec<Val> = args.iter().map(|a| self.emit_expr(a)).collect();
                let (result, dst) = if expr_ty.is_void() {
                    (Val::Var("DUMMY".into()), None)
                } else {
                    let dst = self.make_temp(&expr_ty);
                    (dst.clone(), Some(dst))
                };
                self.instructions.push(Instruction::FnCall {
                    name: name.symbol.clone(),
                    args,
                    dst,
                });
                result
            }

            ast::Expression::Cast {
                target,
                expr: inner,
            } => {
                let result = self.emit_expr(inner);
                if target.to_semantic().is_void() {
                    return ExprResult::Operand(Val::Var("DUMMY".into()));
                } else {
                    let inner_ty = self.semantics.expr_type(inner).clone();
                    self.cast(result, &inner_ty, &target.to_semantic())
                }
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
            ast::Expression::Subscript(expr1, expr2) => {
                let ty1 = self.semantics.expr_type(expr1).clone();
                // Semantic check ensures only a pointer and ints are supported
                // it also converts the type array to a pointer
                let (ptr_expr, index_expr) = if ty1.is_pointer() {
                    (expr1, expr2)
                } else {
                    (expr2, expr1)
                };
                let ptr = self.emit_expr(ptr_expr);
                let index_ty = self.semantics.expr_type(index_expr).clone();
                let index = self.emit_expr(index_expr);
                let index = self.cast(index, &index_ty, &Type::Long);
                let ptr_ty = self.semantics.expr_type(ptr_expr).clone();
                let Type::Pointer(inner) = &ptr_ty else {
                    unreachable!();
                };
                let scale = inner.size();
                let dst = self.make_temp(&ptr_ty);
                self.instructions.push(Instruction::AddPtr {
                    ptr,
                    index,
                    scale,
                    dst: dst.clone(),
                });
                return ExprResult::Dereference(dst);
            }
            ast::Expression::SizeOfType(ty) => {
                return ExprResult::Operand(Val::Constant(Constant::ULong(ty.to_semantic().size() as u64)));
            }
            ast::Expression::SizeOfExpr(e) => {
                let size = if let Some(target) = self.semantics.implicit_casts.get(&e.id).cloned() {
                    target.size()
                } else if let Some(target) = self.semantics.pointer_decays.get(&e.id).cloned() {
                    target.size()
                } else {
                    self.semantics.expr_type(e).size()
                };
                return ExprResult::Operand(Val::Constant(Constant::ULong(size as u64)));
            },
            ast::Expression::Dot { .. } | ast::Expression::Arrow { .. } => todo!()
        };
        ExprResult::Operand(result)
    }

    fn get_or_load(&mut self, result: &ExprResult, expr: &ast::Node<ast::Expression>) -> Val {
        let expr_ty = self.semantics.expr_type(expr).clone();
        let val = match result {
            ExprResult::Operand(val) => val.clone(),
            ExprResult::Dereference(ptr) => {
                let dst = self.make_temp(&expr_ty);
                self.instructions.push(Instruction::Load {
                    ptr: ptr.clone(),
                    dst: dst.clone(),
                });
                dst
            }
        };
        self.cast_if_needed(val, expr)
    }

    fn copy_or_store(&mut self, result: &ExprResult, src: Val) {
        match result {
            ExprResult::Operand(val) => {
                self.instructions.push(Instruction::Copy {
                    src,
                    dst: val.clone(),
                });
            }
            ExprResult::Dereference(ptr) => {
                self.instructions.push(Instruction::Store {
                    src,
                    ptr: ptr.clone(),
                });
            }
        }
    }

    fn cast_if_needed(&mut self, val: Val, expr: &ast::Node<ast::Expression>) -> Val {
        let expr_ty = self.semantics.expr_type(expr).clone();
        let val = if let Some(target) = self.semantics.pointer_decays.get(&expr.id).cloned() {
            let dst = self.make_temp(&target);
            self.instructions.push(Instruction::GetAddress {
                src: val,
                dst: dst.clone(),
            });
            dst
        } else {
            val
        };
        if let Some(target) = self.semantics.implicit_casts.get(&expr.id).cloned() {
            self.cast(val, &expr_ty, &target)
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
        assert!(!matches!(ty, Type::Void));
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

    fn make_string_const(&mut self, s: &Symbol) -> Val {
        let existing_constant = self.strings.get(s);
        let name = match existing_constant {
            Some(name) => name.clone(),
            None => {
                let name = format!("string.{}", self.strings.len());
                self.strings.insert(s.clone(), name.clone());
                self.semantics.symbols.insert(
                    name.clone(),
                    SymbolData {
                        ty: Type::Array(Type::Char.into(), s.len() + 1),
                        attrs: Attributes::Const {
                            init: StaticInit::String {
                                symbol: s.clone(),
                                null_terminated: true,
                            },
                        },
                    },
                );
                name
            }
        };
        Val::Var(name)
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
        strings: Default::default(),
        instructions: vec![],
        tmp_counter: 0,
        label_counter: 0,
    };
    for decl in &program.declarations {
        generator.instructions.clear();
        if let ast::Declaration::Function(function) = decl.as_ref() {
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
    }

    for (name, symbol_data) in &generator.semantics.symbols {
        match symbol_data.attrs.clone() {
            Attributes::Static {
                initial_value,
                global,
            } => {
                let ty = symbol_data.ty.clone();
                match initial_value {
                    InitialValue::Initial(init) => {
                        top_level.push(TopLevel::Variable(StaticVariable {
                            name: name.clone(),
                            ty,
                            global,
                            init,
                        }))
                    }
                    InitialValue::Tentative => {
                        let init = StaticInit::ZeroInit(ty.size());
                        top_level.push(TopLevel::Variable(StaticVariable {
                            name: name.clone(),
                            ty,
                            global,
                            init: vec![init],
                        }))
                    }
                    InitialValue::NoInitializer => continue,
                }
            }
            Attributes::Const { init } => top_level.push(TopLevel::Constant(StaticConstant {
                name: name.clone(),
                ty: symbol_data.ty.clone(),
                init,
            })),
            _ => {}
        }
    }

    Program {
        top_level,
        semantics: generator.semantics,
    }
}
