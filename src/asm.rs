use crate::ast::{Constant, Type};
use crate::semantic::{Attributes, SemanticData, StaticInit};
use crate::symbol::Symbol;
use crate::tacky;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub top_level: Vec<TopLevel>,
}

#[derive(Debug)]
pub enum TopLevel {
    Function(Function),
    Variable(StaticVariable),
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
    pub init: StaticInit,
}

#[derive(Debug)]
pub enum Instruction {
    Mov(AsmType, Operand, Operand),
    Movsx(Operand, Operand),
    MovZeroExtend(Operand, Operand),
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
    Inc,
    Dec,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(i64),
    Reg(Reg),
    Pseudo(Symbol),
    Data(Symbol),
    Stack(i64),
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
}

#[derive(Debug, Clone, Copy)]
pub enum AsmType {
    Longword,
    Quadword,
}

const ARG_REGISTERS: [Reg; 6] = [Reg::Di, Reg::Si, Reg::Dx, Reg::Cx, Reg::R8, Reg::R9];

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
}

enum BackendSymbolData {
    Obj { ty: AsmType, is_static: bool },
    Fn { _defined: bool },
}

type BackendSymbolTable = HashMap<Symbol, BackendSymbolData>;

impl SemanticData {
    fn val_asm_ty(&self, val: &tacky::Val) -> AsmType {
        match val {
            tacky::Val::Constant(Constant::Int(_) | Constant::UInt(_)) => AsmType::Longword,
            tacky::Val::Constant(Constant::Long(_) | Constant::ULong(_)) => AsmType::Quadword,
            tacky::Val::Var(name) => self.symbol_asm_ty(name),
        }
    }

    fn symbol_asm_ty(&self, symbol: &Symbol) -> AsmType {
        match self.symbol_ty(symbol) {
            Type::Int | Type::UInt => AsmType::Longword,
            Type::Long | Type::ULong => AsmType::Quadword,
            Type::Function(_) => unreachable!(),
        }
    }

    fn symbol_ty(&self, symbol: &Symbol) -> &Type {
        &self.symbols.get(symbol).expect("Symbol without type").ty
    }

    fn is_signed(&self, val: &tacky::Val) -> bool {
        match val {
            tacky::Val::Constant(Constant::Int(_) | Constant::Long(_)) => true,
            tacky::Val::Constant(Constant::UInt(_) | Constant::ULong(_)) => false,
            tacky::Val::Var(name) => match self.symbol_ty(name) {
                Type::Int | Type::Long => true,
                Type::UInt | Type::ULong => false,
                Type::Function(_) => unreachable!(),
            },
        }
    }
}

pub fn generate(program: &tacky::Program) -> Program {
    let mut top_level = Vec::new();
    for element in &program.top_level {
        match element {
            tacky::TopLevel::Function(f) => {
                top_level.push(TopLevel::Function(generate_function(f, &program.semantics)))
            }
            tacky::TopLevel::Variable(v) => top_level.push(TopLevel::Variable(
                generate_static_variable(v, &program.semantics),
            )),
        }
    }
    Program { top_level }
}

fn generate_function(function: &tacky::Function, semantic: &SemanticData) -> Function {
    let mut instructions = Vec::new();

    for (i, param) in function.params.iter().cloned().enumerate() {
        let src = if i < ARG_REGISTERS.len() {
            Operand::Reg(ARG_REGISTERS[i])
        } else {
            let offset = 16 + 8 * (i - ARG_REGISTERS.len());
            Operand::Stack(offset as i64)
        };
        instructions.push(Instruction::Mov(
            semantic.symbol_asm_ty(&param),
            src,
            Operand::Pseudo(param),
        ))
    }

    for tacky_instruction in &function.body {
        match tacky_instruction {
            tacky::Instruction::Return(val) => {
                instructions.push(Instruction::Mov(
                    semantic.val_asm_ty(val),
                    val.to_asm(),
                    Operand::Reg(Reg::Ax),
                ));
                instructions.push(Instruction::Ret);
            }

            tacky::Instruction::Unary { op, src, dst } => match op {
                tacky::UnaryOp::Not => {
                    instructions.push(Instruction::Cmp(
                        semantic.val_asm_ty(src),
                        Operand::Imm(0),
                        src.to_asm(),
                    ));
                    instructions.push(Instruction::Mov(
                        semantic.val_asm_ty(dst),
                        Operand::Imm(0),
                        dst.to_asm(),
                    ));
                    instructions.push(Instruction::SetCC(CondCode::E, dst.to_asm()));
                }
                _ => {
                    let ty = semantic.val_asm_ty(src);
                    instructions.push(Instruction::Mov(ty, src.to_asm(), dst.to_asm()));
                    instructions.push(Instruction::Unary(ty, op.to_asm(), dst.to_asm()));
                }
            },

            tacky::Instruction::Binary {
                op,
                src1,
                src2,
                dst,
            } => match op {
                tacky::BinaryOp::Divide => {
                    let ty = semantic.val_asm_ty(src1);
                    instructions.push(Instruction::Mov(ty, src1.to_asm(), Operand::Reg(Reg::Ax)));
                    if semantic.is_signed(src1) {
                        instructions.push(Instruction::Cdq(ty));
                        instructions.push(Instruction::Idiv(ty, src2.to_asm()));
                    } else {
                        instructions.push(Instruction::Mov(
                            ty,
                            Operand::Imm(0),
                            Operand::Reg(Reg::Dx),
                        ));
                        instructions.push(Instruction::Div(ty, src2.to_asm()));
                    }
                    instructions.push(Instruction::Mov(ty, Operand::Reg(Reg::Ax), dst.to_asm()));
                }
                tacky::BinaryOp::Reminder => {
                    let ty = semantic.val_asm_ty(src1);
                    instructions.push(Instruction::Mov(ty, src1.to_asm(), Operand::Reg(Reg::Ax)));
                    if semantic.is_signed(src1) {
                        instructions.push(Instruction::Cdq(ty));
                        instructions.push(Instruction::Idiv(ty, src2.to_asm()));
                    } else {
                        instructions.push(Instruction::Mov(
                            ty,
                            Operand::Imm(0),
                            Operand::Reg(Reg::Dx),
                        ));
                        instructions.push(Instruction::Div(ty, src2.to_asm()));
                    }
                    instructions.push(Instruction::Mov(ty, Operand::Reg(Reg::Dx), dst.to_asm()));
                }
                tacky::BinaryOp::ShiftLeft => {
                    let ty = semantic.val_asm_ty(src1);
                    instructions.push(Instruction::Mov(ty, src1.to_asm(), dst.to_asm()));
                    instructions.push(Instruction::Mov(ty, src2.to_asm(), Operand::Reg(Reg::Cx)));
                    if semantic.is_signed(src1) {
                        instructions.push(Instruction::Sal(ty, dst.to_asm()));
                    } else {
                        instructions.push(Instruction::Shl(ty, dst.to_asm()));
                    }
                }
                tacky::BinaryOp::ShiftRight => {
                    let ty = semantic.val_asm_ty(src1);
                    instructions.push(Instruction::Mov(ty, src1.to_asm(), dst.to_asm()));
                    instructions.push(Instruction::Mov(ty, src2.to_asm(), Operand::Reg(Reg::Cx)));
                    if semantic.is_signed(src1) {
                        instructions.push(Instruction::Sar(ty, dst.to_asm()));
                    } else {
                        instructions.push(Instruction::Shr(ty, dst.to_asm()));
                    }
                }
                tacky::BinaryOp::Equal
                | tacky::BinaryOp::NotEqual
                | tacky::BinaryOp::GreaterThan
                | tacky::BinaryOp::GreaterOrEqual
                | tacky::BinaryOp::LessThan
                | tacky::BinaryOp::LessOrEqual => {
                    instructions.push(Instruction::Cmp(
                        semantic.val_asm_ty(src1),
                        src2.to_asm(),
                        src1.to_asm(),
                    ));
                    instructions.push(Instruction::Mov(
                        semantic.val_asm_ty(dst),
                        Operand::Imm(0),
                        dst.to_asm(),
                    ));
                    let cond = match (op, semantic.is_signed(src1)) {
                        (tacky::BinaryOp::Equal, _) => CondCode::E,
                        (tacky::BinaryOp::NotEqual, _) => CondCode::NE,
                        (tacky::BinaryOp::GreaterThan, true) => CondCode::G,
                        (tacky::BinaryOp::GreaterThan, false) => CondCode::A,
                        (tacky::BinaryOp::GreaterOrEqual, true) => CondCode::GE,
                        (tacky::BinaryOp::GreaterOrEqual, false) => CondCode::AE,
                        (tacky::BinaryOp::LessThan, true) => CondCode::L,
                        (tacky::BinaryOp::LessThan, false) => CondCode::B,
                        (tacky::BinaryOp::LessOrEqual, true) => CondCode::LE,
                        (tacky::BinaryOp::LessOrEqual, false) => CondCode::BE,
                        _ => unreachable!(),
                    };
                    instructions.push(Instruction::SetCC(cond, dst.to_asm()));
                }
                _ => {
                    let ty = semantic.val_asm_ty(src1);
                    instructions.push(Instruction::Mov(ty, src1.to_asm(), dst.to_asm()));
                    instructions.push(Instruction::Binary(
                        ty,
                        op.to_asm(),
                        src2.to_asm(),
                        dst.to_asm(),
                    ));
                }
            },

            tacky::Instruction::Jump { target } => {
                instructions.push(Instruction::Jmp(target.clone()));
            }
            tacky::Instruction::JumpIfZero { cond, target } => {
                instructions.push(Instruction::Cmp(
                    semantic.val_asm_ty(cond),
                    Operand::Imm(0),
                    cond.to_asm(),
                ));
                instructions.push(Instruction::JmpCC(CondCode::E, target.clone()));
            }
            tacky::Instruction::JumpIfNotZero { cond, target } => {
                instructions.push(Instruction::Cmp(
                    semantic.val_asm_ty(cond),
                    Operand::Imm(0),
                    cond.to_asm(),
                ));
                instructions.push(Instruction::JmpCC(CondCode::NE, target.clone()));
            }
            tacky::Instruction::Copy { src, dst } => {
                instructions.push(Instruction::Mov(
                    semantic.val_asm_ty(src),
                    src.to_asm(),
                    dst.to_asm(),
                ));
            }
            tacky::Instruction::Label(l) => {
                instructions.push(Instruction::Label(l.clone()));
            }
            tacky::Instruction::FnCall { name, args, dst } => {
                generate_call(&mut instructions, semantic, name, args, dst);
            }
            tacky::Instruction::SignExtend { src, dst } => {
                instructions.push(Instruction::Movsx(src.to_asm(), dst.to_asm()));
            }
            tacky::Instruction::Truncate { src, dst } => {
                instructions.push(Instruction::Mov(
                    AsmType::Longword,
                    src.to_asm(),
                    dst.to_asm(),
                ));
            }
            tacky::Instruction::ZeroExtend { src, dst } => {
                instructions.push(Instruction::MovZeroExtend(src.to_asm(), dst.to_asm()));
            }
        }
    }

    let mut backend_symbols = HashMap::new();
    for (symbol, data) in semantic.symbols.iter() {
        match data.attrs {
            Attributes::Function { defined, .. } => {
                backend_symbols.insert(symbol.clone(), BackendSymbolData::Fn { _defined: defined });
            }
            Attributes::Static { .. } => {
                backend_symbols.insert(
                    symbol.clone(),
                    BackendSymbolData::Obj {
                        ty: semantic.symbol_asm_ty(symbol),
                        is_static: true,
                    },
                );
            }
            Attributes::Local => {
                backend_symbols.insert(
                    symbol.clone(),
                    BackendSymbolData::Obj {
                        ty: semantic.symbol_asm_ty(symbol),
                        is_static: false,
                    },
                );
            }
        }
    }

    let stack_size = replace_pseudo_registers(&mut instructions, &backend_symbols);
    let instructions = fixup_instructions(instructions, stack_size);

    Function {
        name: function.name.clone(),
        global: function.global,
        instructions,
    }
}

fn generate_static_variable(
    var: &tacky::StaticVariable,
    semantic: &SemanticData,
) -> StaticVariable {
    StaticVariable {
        name: var.name.clone(),
        global: var.global,
        init: var.init,
        alignment: match semantic.symbol_asm_ty(&var.name) {
            AsmType::Longword => 4,
            AsmType::Quadword => 8,
        },
    }
}

fn generate_call(
    instructions: &mut Vec<Instruction>,
    semantic: &SemanticData,
    name: &Symbol,
    args: &[tacky::Val],
    dst: &tacky::Val,
) {
    let padding = if args.len() > 6 && args.len() % 2 != 0 {
        8
    } else {
        0
    };
    if padding != 0 {
        instructions.push(Instruction::Binary(
            AsmType::Quadword,
            BinaryOp::Sub,
            Operand::Imm(padding),
            Operand::Reg(Reg::SP),
        ))
    }

    let register_args = args.iter().take(6);
    for (reg, val) in ARG_REGISTERS.iter().zip(register_args) {
        instructions.push(Instruction::Mov(
            semantic.val_asm_ty(val),
            val.to_asm(),
            Operand::Reg(*reg),
        ));
    }

    let stack_args = args.iter().skip(6);
    for val in stack_args.clone().rev() {
        let operand = val.to_asm();
        let ty = semantic.val_asm_ty(val);
        if matches!(operand, Operand::Imm(_) | Operand::Reg(_)) || matches!(ty, AsmType::Quadword) {
            instructions.push(Instruction::Push(operand))
        } else {
            instructions.push(Instruction::Mov(ty, operand, Operand::Reg(Reg::Ax)));
            instructions.push(Instruction::Push(Operand::Reg(Reg::Ax)));
        }
    }

    instructions.push(Instruction::Call(name.clone()));

    let bytes_to_remove = 8 * stack_args.count() as i64 + padding;
    if bytes_to_remove != 0 {
        instructions.push(Instruction::Binary(
            AsmType::Quadword,
            BinaryOp::Add,
            Operand::Imm(bytes_to_remove),
            Operand::Reg(Reg::SP),
        ));
    }
    instructions.push(Instruction::Mov(
        semantic.val_asm_ty(dst),
        Operand::Reg(Reg::Ax),
        dst.to_asm(),
    ));
}

fn replace_pseudo_registers(
    instructions: &mut Vec<Instruction>,
    symbols: &BackendSymbolTable,
) -> i64 {
    let mut stack_size = 0;
    let mut stack_vars = HashMap::new();

    let mut update_operand = |operand: &mut Operand| {
        if let Operand::Pseudo(name) = operand {
            let Some(&BackendSymbolData::Obj { ty, is_static }) = symbols.get(name) else {
                panic!("Operand without symbol data")
            };
            let offset = if let Some(saved) = stack_vars.get(name).copied() {
                saved
            } else if is_static {
                *operand = Operand::Data(name.clone());
                return;
            } else {
                match ty {
                    AsmType::Longword => {
                        stack_size += 4;
                    }
                    AsmType::Quadword => {
                        stack_size += 8 + stack_size % 8;
                    }
                }
                stack_vars.insert(name.clone(), stack_size);
                stack_size
            };
            *operand = Operand::Stack(-offset);
        }
    };

    for instruction in instructions {
        match instruction {
            Instruction::Mov(_, src, dst)
            | Instruction::Movsx(src, dst)
            | Instruction::MovZeroExtend(src, dst)
            | Instruction::Binary(_, _, src, dst)
            | Instruction::Cmp(_, src, dst) => {
                update_operand(src);
                update_operand(dst);
            }
            Instruction::Unary(_, _, src)
            | Instruction::Push(src)
            | Instruction::Idiv(_, src)
            | Instruction::Div(_, src)
            | Instruction::Sal(_, src)
            | Instruction::Shl(_, src)
            | Instruction::Sar(_, src)
            | Instruction::Shr(_, src)
            | Instruction::SetCC(_, src) => update_operand(src),
            _ => continue,
        }
    }

    stack_size
}

fn fixup_instructions(instructions: Vec<Instruction>, stack_size: i64) -> Vec<Instruction> {
    let mut fixed = Vec::with_capacity(instructions.len() + 1);

    let stack_size = ((stack_size / 16) + 1) * 16;
    // TODO: test: let stack_size = stack_size + stack_size % 16;
    fixed.push(Instruction::Binary(
        AsmType::Quadword,
        BinaryOp::Sub,
        Operand::Imm(stack_size),
        Operand::Reg(Reg::SP),
    ));

    for instruction in instructions.into_iter() {
        match instruction {
            Instruction::Mov(ty, src, dst) => {
                let src = if let Operand::Imm(v) = src {
                    if v as u64 > i32::MAX as u64 {
                        let value = match ty {
                            AsmType::Longword => (v as i32) as i64,
                            AsmType::Quadword => v,
                        };
                        fixed.push(Instruction::Mov(
                            ty,
                            Operand::Imm(value),
                            Operand::Reg(Reg::R10),
                        ));
                        Operand::Reg(Reg::R10)
                    } else {
                        src
                    }
                } else if src.is_mem() && dst.is_mem() {
                    fixed.push(Instruction::Mov(ty, src, Operand::Reg(Reg::R10)));
                    Operand::Reg(Reg::R10)
                } else {
                    src
                };
                fixed.push(Instruction::Mov(ty, src, dst));
            }
            Instruction::Movsx(src, dst) => {
                // TODO: Add is_imm()
                let src = if let Operand::Imm(value) = src {
                    // TODO: Add shortcuts for constructing operands
                    fixed.push(Instruction::Mov(
                        AsmType::Longword,
                        Operand::Imm(value),
                        Operand::Reg(Reg::R10),
                    ));
                    Operand::Reg(Reg::R10)
                } else {
                    src
                };
                if dst.is_mem() {
                    fixed.push(Instruction::Movsx(src, Operand::Reg(Reg::R11)));
                    fixed.push(Instruction::Mov(
                        AsmType::Quadword,
                        Operand::Reg(Reg::R11),
                        dst,
                    ));
                } else {
                    fixed.push(Instruction::Movsx(src, dst));
                }
            }
            Instruction::Binary(ty, op, left, right)
                if matches!(
                    op,
                    BinaryOp::Add
                        | BinaryOp::Sub
                        | BinaryOp::Mul
                        | BinaryOp::And
                        | BinaryOp::Or
                        | BinaryOp::Xor
                ) =>
            {
                let left = if let Operand::Imm(v) = left {
                    if v as u64 > i32::MAX as u64 {
                        fixed.push(Instruction::Mov(ty, left, Operand::Reg(Reg::R10)));
                        Operand::Reg(Reg::R10)
                    } else {
                        left
                    }
                } else if left.is_mem() && right.is_mem() {
                    fixed.push(Instruction::Mov(ty, left, Operand::Reg(Reg::R10)));
                    Operand::Reg(Reg::R10)
                } else {
                    left
                };
                if matches!(op, BinaryOp::Mul)
                    && matches!(right, Operand::Stack(_) | Operand::Data(_))
                {
                    fixed.push(Instruction::Mov(ty, right.clone(), Operand::Reg(Reg::R11)));
                    fixed.push(Instruction::Binary(
                        ty,
                        BinaryOp::Mul,
                        left,
                        Operand::Reg(Reg::R11),
                    ));
                    fixed.push(Instruction::Mov(ty, Operand::Reg(Reg::R11), right));
                } else {
                    fixed.push(Instruction::Binary(ty, op, left, right));
                }
            }
            Instruction::Cmp(ty, left, right) => {
                // TODO: Repeated code
                let left = if let Operand::Imm(v) = left {
                    if v as u64 > i32::MAX as u64 {
                        fixed.push(Instruction::Mov(ty, left, Operand::Reg(Reg::R10)));
                        Operand::Reg(Reg::R10)
                    } else {
                        left
                    }
                } else if left.is_mem() && right.is_mem() {
                    fixed.push(Instruction::Mov(ty, left, Operand::Reg(Reg::R10)));
                    Operand::Reg(Reg::R10)
                } else {
                    left
                };

                let right = if let Operand::Imm(value) = right {
                    fixed.push(Instruction::Mov(
                        ty,
                        Operand::Imm(value),
                        Operand::Reg(Reg::R11),
                    ));
                    Operand::Reg(Reg::R11)
                } else {
                    right
                };
                fixed.push(Instruction::Cmp(ty, left, right));
            }
            Instruction::Idiv(ty, Operand::Imm(value))
            | Instruction::Div(ty, Operand::Imm(value)) => {
                fixed.push(Instruction::Mov(
                    ty,
                    Operand::Imm(value),
                    Operand::Reg(Reg::R10),
                ));
                fixed.push(Instruction::Idiv(ty, Operand::Reg(Reg::R10)));
            }
            Instruction::Push(Operand::Imm(value)) => {
                let value = if value as u64 > i32::MAX as u64 {
                    fixed.push(Instruction::Mov(
                        AsmType::Quadword,
                        Operand::Imm(value),
                        Operand::Reg(Reg::R10),
                    ));
                    Operand::Reg(Reg::R10)
                } else {
                    Operand::Imm(value)
                };
                fixed.push(Instruction::Push(value));
            }
            Instruction::MovZeroExtend(src, Operand::Reg(dst)) => {
                fixed.push(Instruction::Mov(AsmType::Longword, src, Operand::Reg(dst)));
            }
            Instruction::MovZeroExtend(src, dst) => {
                fixed.push(Instruction::Mov(
                    AsmType::Longword,
                    src,
                    Operand::Reg(Reg::R11),
                ));
                fixed.push(Instruction::Mov(
                    AsmType::Quadword,
                    Operand::Reg(Reg::R11),
                    dst,
                ));
            }
            other => fixed.push(other),
        }
    }
    fixed
}

impl tacky::Val {
    fn to_asm(&self) -> Operand {
        match self {
            // TODO: Constant::as_i64() ?
            tacky::Val::Constant(value) => match value {
                Constant::Int(v) => Operand::Imm(*v as i64),
                Constant::UInt(v) => Operand::Imm(*v as i64),
                Constant::Long(v) => Operand::Imm(*v),
                Constant::ULong(v) => Operand::Imm(*v as i64),
            },
            tacky::Val::Var(name) => Operand::Pseudo(name.clone()),
        }
    }
}

impl tacky::UnaryOp {
    fn to_asm(&self) -> UnaryOp {
        match self {
            tacky::UnaryOp::Complement => UnaryOp::Not,
            tacky::UnaryOp::Negate => UnaryOp::Neg,
            tacky::UnaryOp::Increment => UnaryOp::Inc,
            tacky::UnaryOp::Decrement => UnaryOp::Dec,

            tacky::UnaryOp::Not => unreachable!(), // `Not` does not have equivalent
        }
    }
}

impl tacky::BinaryOp {
    fn to_asm(&self) -> BinaryOp {
        match self {
            tacky::BinaryOp::Add => BinaryOp::Add,
            tacky::BinaryOp::Subtract => BinaryOp::Sub,
            tacky::BinaryOp::Multiply => BinaryOp::Mul,
            tacky::BinaryOp::BinAnd => BinaryOp::And,
            tacky::BinaryOp::BinOr => BinaryOp::Or,
            tacky::BinaryOp::BinXor => BinaryOp::Xor,

            _ => unreachable!(), // Other operators do not have equivalent
        }
    }
}

impl Operand {
    fn is_mem(&self) -> bool {
        matches!(self, Operand::Stack(_) | Operand::Data(_))
    }
}
