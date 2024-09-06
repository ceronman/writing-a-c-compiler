use crate::lexer::Span;
use crate::symbol::Symbol;
use std::ops::Deref;

#[derive(Debug)]
pub struct Node<T> {
    pub span: Span,
    pub data: Box<T>,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> AsRef<T> for Node<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> Node<T> {
    pub fn from(span: Span, data: T) -> Self {
        Node {
            span,
            data: Box::new(data),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub function_definition: Node<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: Node<Identifier>,
    pub body: Vec<Node<BlockItem>>,
}

#[derive(Debug)]
pub struct Identifier {
    pub symbol: Symbol,
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Statement),
    Decl(Declaration),
}

#[derive(Debug)]
pub enum Statement {
    Return { expr: Node<Expression> },
    Expression(Node<Expression>),
    Null,
}

#[derive(Debug)]
pub enum Expression {
    Constant(i64),
    Var(Symbol),
    Unary {
        op: Node<UnaryOp>,
        expr: Node<Expression>,
    },
    Binary {
        op: Node<BinaryOp>,
        left: Node<Expression>,
        right: Node<Expression>,
    },
    Assignment {
        left: Node<Expression>,
        right: Node<Expression>,
    },
}

#[derive(Debug)]
pub struct Declaration {
    pub name: Node<Identifier>,
    pub init: Option<Node<Expression>>,
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
