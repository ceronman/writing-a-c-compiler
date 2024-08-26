use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Program {
    pub function_definition: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Symbol,
    pub body: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return { expr: Expression },
}

#[derive(Debug)]
pub enum Expression {
    Constant(i64),
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
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
}
