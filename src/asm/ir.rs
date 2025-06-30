use crate::semantic::StaticInit;
use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Program {
    pub top_level: Vec<TopLevel>,
}

#[derive(Debug)]
pub enum TopLevel {
    Function(Function),
    Variable(StaticVariable),
    Constant(StaticConstant),
}

#[derive(Debug)]
pub struct Function {
    pub name: Symbol,
    pub global: bool,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct StaticVariable {
    pub name: Symbol,
    pub global: bool,
    pub alignment: u8,
    pub init: Vec<StaticInit>,
}

#[derive(Debug)]
pub struct StaticConstant {
    pub name: Symbol,
    pub alignment: u8,
    pub init: StaticInit,
}

#[derive(Debug)]
pub enum Instruction {
    Mov(AsmType, Operand, Operand),
    Movsx(Operand, Operand),
    MovZeroExtend(Operand, Operand),
    Lea(Operand, Operand),
    Cvttsd2si(AsmType, Operand, Operand),
    Cvtsi2sd(AsmType, Operand, Operand),
    Unary(AsmType, UnaryOp, Operand),
    Binary(AsmType, BinaryOp, Operand, Operand),
    Cmp(AsmType, Operand, Operand),
    Idiv(AsmType, Operand),
    Div(AsmType, Operand),
    Sal(AsmType, Operand),
    Shl(AsmType, Operand),
    Sar(AsmType, Operand),
    Shr(AsmType, Operand),
    Cdq(AsmType),
    Jmp(Symbol),
    JmpCC(CondCode, Symbol),
    SetCC(CondCode, Operand),
    Label(Symbol),
    Push(Operand),
    Call(Symbol),
    Ret,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    DivDouble,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(u64),
    Reg(Reg),
    Pseudo(Symbol),
    PseudoMem(Symbol, i64),
    Data(bool, Symbol),
    Memory(Reg, i64),
    Indexed(Reg, Reg, u8)
}

impl Operand {
    pub(crate) fn is_mem(&self) -> bool {
        matches!(self, Operand::Memory(..) | Operand::Data(..) | Operand::Indexed(..))
    }
}

impl From<Reg> for Operand {
    fn from(value: Reg) -> Self {
        Self::Reg(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    Ax,
    Cx,
    Dx,
    Di,
    Si,
    R8,
    R9,
    R10,
    R11,
    SP,
    BP,
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM14,
    XMM15,
}

#[derive(Debug, Clone, Copy)]
pub enum AsmType {
    Longword,
    Quadword,
    Double,
    ByteArray { size: u64, alignment: u8 }
}

#[derive(Debug)]
pub enum CondCode {
    A,
    AE,
    B,
    BE,
    E,
    G,
    GE,
    L,
    LE,
    NE,
    P,
    NP,
}
