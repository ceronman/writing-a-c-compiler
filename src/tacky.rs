use crate::ast;
use crate::ast::{Expression, Statement};
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
    Unary { op: UnaryOp, src: Val, dst: Val },
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
}

pub fn generate(program: &ast::Program) -> Program {
    let function = &program.function_definition;
    Program {
        function: Function {
            name: function.name.clone(),
            body: Generator::default().emit_body(&function.body),
        },
    }
}

#[derive(Default)]
struct Generator {
    instructions: Vec<Instruction>,
    tmp_counter: u32,
}

impl Generator {
    fn emit_body(mut self, body: &Statement) -> Vec<Instruction> {
        match body {
            Statement::Return { expr } => {
                let val = self.emit_expr(expr);
                self.instructions.push(Instruction::Return(val));
            }
        }
        self.instructions
    }

    fn emit_expr(&mut self, expr: &Expression) -> Val {
        match expr {
            Expression::Constant(val) => Val::Constant(*val),
            Expression::Unary { op, expr } => {
                let src = self.emit_expr(expr);
                let dst = self.make_temp();
                let op = match op {
                    ast::UnaryOp::Complement => UnaryOp::Complement,
                    ast::UnaryOp::Negate => UnaryOp::Negate,
                };
                self.instructions.push(Instruction::Unary {
                    op,
                    src,
                    dst: dst.clone(),
                });
                dst
            }
        }
    }

    fn make_temp(&mut self) -> Val {
        let tmp = Val::Var(format!("tmp.{i}", i = self.tmp_counter));
        self.tmp_counter += 1;
        tmp
    }
}
