pub mod pretty;

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
    Return(Option<Node<Expression>>),
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
    String(Symbol),
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
        target: Node<TypeSpec>,
        expr: Node<Expression>,
    },
    Dereference(Node<Expression>),
    AddressOf(Node<Expression>),
    Subscript(Node<Expression>, Node<Expression>),
    SizeOfExpr(Node<Expression>),
    SizeOfType(Node<TypeSpec>),
    Dot {
        aggregate: Node<Expression>,
        field: Node<Identifier>,
    },
    Arrow {
        pointer: Node<Expression>,
        field: Node<Identifier>,
    },
}

impl Expression {
    pub fn is_null_constant(&self) -> bool {
        matches!(
            self,
            Expression::Constant(
                Constant::Int(0) | Constant::Long(0) | Constant::UInt(0) | Constant::ULong(0)
            )
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Double(f64),
    Char(i8),
    UChar(u8),
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum Declaration {
    Var(VarDeclaration),
    Function(FunctionDeclaration),
    Struct(NameAndFields),
    Union(NameAndFields),
}

#[derive(Debug)]
pub enum TypeSpec {
    Char,
    SChar,
    UChar,
    Int,
    UInt,
    Long,
    ULong,
    Double,
    Function(FunctionTypeSpec),
    Pointer(Node<TypeSpec>),
    Array(Node<TypeSpec>, usize),
    Struct(Node<Identifier>),
    Union(Node<Identifier>),
    Void,
}

#[derive(Debug)]
pub struct FunctionTypeSpec {
    pub params: Vec<Node<TypeSpec>>,
    pub ret: Node<TypeSpec>,
}

#[derive(Debug)]
pub struct VarDeclaration {
    pub name: Node<Identifier>,
    pub init: Option<Node<Initializer>>,
    pub type_spec: Node<TypeSpec>,
    pub storage_class: Option<Node<StorageClass>>,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: Node<Identifier>,
    pub params: Vec<Node<Identifier>>,
    pub body: Option<Node<Block>>,
    pub type_spec: FunctionTypeSpec,
    pub storage_class: Option<Node<StorageClass>>,
}

#[derive(Debug)]
pub struct NameAndFields {
    pub name: Node<Identifier>,
    pub fields: Vec<Node<Field>>,
}

#[derive(Debug)]
pub struct Field {
    pub name: Node<Identifier>,
    pub type_spec: Node<TypeSpec>,
}

#[derive(Debug)]
pub enum Initializer {
    Single(Node<Expression>),
    Compound(Vec<Node<Initializer>>),
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

impl Constant {
    pub fn is_int(&self) -> bool {
        !matches!(self, Constant::Double(_))
    }

    pub fn as_u64(&self) -> u64 {
        match self {
            Constant::Char(v) => *v as u64,
            Constant::UChar(v) => *v as u64,
            Constant::Int(v) => *v as u64,
            Constant::UInt(v) => *v as u64,
            Constant::Long(v) => *v as u64,
            Constant::ULong(v) => *v,
            Constant::Double(v) => *v as u64,
        }
    }
}
