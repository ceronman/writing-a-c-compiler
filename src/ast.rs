use crate::lexer::Span;
use crate::symbol::Symbol;
use std::ops::{Deref, DerefMut};

pub type NodeId = u32;

#[derive(Debug)]
pub struct Node<T> {
    pub id: NodeId,
    pub span: Span,
    pub data: Box<T>,
}

#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Node<Declaration>>,
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
    Return(Node<Expression>),
    If {
        cond: Node<Expression>,
        then_stmt: Node<Statement>,
        else_stmt: Option<Node<Statement>>,
    },
    Switch {
        expr: Node<Expression>,
        body: Node<Statement>,
        label: Symbol,
    },
    Expression(Node<Expression>),
    Labeled {
        name: Node<Identifier>,
        body: Node<Statement>,
    },
    Default {
        label: Symbol,
        body: Node<Statement>,
    },
    Case {
        label: Symbol,
        value: Node<Expression>,
        body: Node<Statement>,
    },
    Goto(Node<Identifier>),
    Compound(Node<Block>),
    While {
        cond: Node<Expression>,
        body: Node<Statement>,
        label: Symbol,
    },
    DoWhile {
        cond: Node<Expression>,
        body: Node<Statement>,
        label: Symbol,
    },
    For {
        init: ForInit,
        cond: Option<Node<Expression>>,
        post: Option<Node<Expression>>,
        body: Node<Statement>,
        label: Symbol,
    },
    Break(Symbol),
    Continue(Symbol),
    Null,
}

#[derive(Debug)]
pub enum ForInit {
    None,
    Decl(Node<VarDeclaration>),
    Expr(Node<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Constant(Constant),
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
    FunctionCall {
        name: Node<Identifier>,
        args: Vec<Node<Expression>>,
    },
    Cast {
        target: Node<Type>,
        expr: Node<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Double(f64),
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum Declaration {
    Var(VarDeclaration),
    Function(FunctionDeclaration),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Int,
    UInt,
    Long,
    ULong,
    Double,
    Function(FunctionType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub ret: Box<Type>,
}

#[derive(Debug)]
pub struct VarDeclaration {
    pub name: Node<Identifier>,
    pub init: Option<Node<Expression>>,
    pub ty: Type,
    pub storage_class: Option<Node<StorageClass>>,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: Node<Identifier>,
    pub params: Vec<Node<Identifier>>,
    pub body: Option<Node<Block>>,
    pub ty: FunctionType,
    pub storage_class: Option<Node<StorageClass>>,
}

#[derive(Debug)]
pub enum StorageClass {
    Static,
    Extern,
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

pub trait InnerRef<T> {
    fn inner_ref(&self) -> Option<&T>;
}

impl<T> InnerRef<T> for Option<Node<T>> {
    fn inner_ref(&self) -> Option<&T> {
        self.as_ref().map(Node::as_ref)
    }
}

impl Type {
    pub fn is_int(&self) -> bool {
        matches!(self, Type::Int | Type::UInt | Type::Long | Type::ULong)
    }

    pub fn size(&self) -> u8 {
        match self {
            Type::Int => 4,
            Type::UInt => 4,
            Type::Long => 8,
            Type::ULong => 8,
            Type::Double => 8,
            Type::Function(_) => panic!("Size of function type"),
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            Type::Int => true,
            Type::UInt => false,
            Type::Long => true,
            Type::ULong => false,
            Type::Double => false,
            Type::Function(_) => panic!("Size of function type"),
        }
    }
}

impl Constant {
    pub fn as_u64(&self) -> u64 {
        match self {
            Constant::Int(v) => *v as u64,
            Constant::UInt(v) => *v as u64,
            Constant::Long(v) => *v as u64,
            Constant::ULong(v) => *v,
            Constant::Double(v) => *v as u64,
        }
    }

    pub fn is_int(&self) -> bool {
        self.ty().is_int()
    }

    pub fn ty(&self) -> Type {
        match self {
            Constant::Int(_) => Type::Int,
            Constant::UInt(_) => Type::UInt,
            Constant::Long(_) => Type::Long,
            Constant::ULong(_) => Type::ULong,
            Constant::Double(_) => Type::Double,
        }
    }
}
