use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Program {
    pub function_definition: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Symbol,
    pub body: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Statement),
    Decl(Declaration),
}

#[derive(Debug)]
pub enum Statement {
    Return { expr: Expression },
    Expression(Expression),
    Null,
}

#[derive(Debug)]
pub enum Expression {
    Constant(i64),
    Var(Symbol),
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Assignment {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub struct Declaration {
    pub name: Symbol,
    pub init: Option<Expression>,
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
    Not,
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
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqualThan,
    GreaterThan,
    GreaterOrEqualThan,
}
