use crate::lexer::Span;
use crate::symbol::Symbol;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Node<T> {
    pub span: Span,
    pub data: Box<T>,
}

#[derive(Debug)]
pub struct Program {
    pub function_definition: Node<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: Node<Identifier>,
    pub body: Vec<BlockItem>,
}

#[derive(Debug)]
pub struct Identifier {
    pub symbol: Symbol,
}

#[derive(Debug)]
pub enum BlockItem {
    Stmt(Node<Statement>),
    Decl(Node<Declaration>),
}

#[derive(Debug)]
pub enum Statement {
    Return {
        expr: Node<Expression>,
    },
    If {
        cond: Node<Expression>,
        then_stmt: Node<Statement>,
        else_stmt: Option<Node<Statement>>,
    },
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
    Postfix {
        op: Node<PostfixOp>,
        expr: Node<Expression>,
    },
    Binary {
        op: Node<BinaryOp>,
        left: Node<Expression>,
        right: Node<Expression>,
    },
    Assignment {
        op: Node<AssignOp>,
        left: Node<Expression>,
        right: Node<Expression>,
    },
    Conditional {
        cond: Node<Expression>,
        then_expr: Node<Expression>,
        else_expr: Node<Expression>,
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
    Increment,
    Decrement,
}

#[derive(Debug)]
pub enum PostfixOp {
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
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqualThan,
    GreaterThan,
    GreaterOrEqualThan,
}

#[derive(Debug)]
pub enum AssignOp {
    Equal,
    AddEqual,
    SubEqual,
    MulEqual,
    DivEqual,
    ModEqual,
    BitAndEqual,
    BitOrEqual,
    BitXorEqual,
    ShiftLeftEqual,
    ShiftRightEqual,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> AsRef<T> for Node<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> AsMut<T> for Node<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
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
