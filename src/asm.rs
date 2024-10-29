use crate::ast::{Constant, Type};
use crate::semantic::{Attributes, SemanticData, StaticInit};
use crate::symbol::Symbol;
use crate::tacky;
use std::collections::HashMap;

// TODO: Extract asm ast into a separate module.
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
    Cvttsd2si(Operand, Operand),
    Cvtsi2sd(Operand, Operand),
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
    Shr,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    DivDouble
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(i64),
    Reg(Reg),
    Pseudo(Symbol),
    Data(Symbol),
    Stack(i64),
}

impl Operand {
    fn is_mem(&self) -> bool {
        matches!(self, Operand::Stack(_) | Operand::Data(_))
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
    Double
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
            tacky::Val::Constant(Constant::Double(_)) => todo!(),
            tacky::Val::Var(name) => self.symbol_asm_ty(name),
        }
    }

    fn symbol_asm_ty(&self, symbol: &Symbol) -> AsmType {
        match self.symbol_ty(symbol) {
            Type::Int | Type::UInt => AsmType::Longword,
            Type::Long | Type::ULong => AsmType::Quadword,
            Type::Double => todo!(),
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
            tacky::Val::Constant(Constant::Double(_)) => todo!(),
            tacky::Val::Var(name) => match self.symbol_ty(name) {
                Type::Int | Type::Long => true,
                Type::UInt | Type::ULong => false,
                Type::Double => todo!(),
                Type::Function(_) => unreachable!(),
            },
        }
    }
}

struct Compiler {
    doubles: HashMap<u64, Symbol>,
    double_counter: usize,
}

impl Compiler {
    fn generate(&mut self, program: &tacky::Program) -> Program {
        let mut top_level = Vec::new();
        for element in &program.top_level {
            match element {
                tacky::TopLevel::Function(f) => {
                    top_level.push(TopLevel::Function(self.generate_function(f, &program.semantics)))
                }
                tacky::TopLevel::Variable(v) => top_level.push(TopLevel::Variable(
                    self.generate_static_variable(v, &program.semantics),
                )),
            }
        }
        Program { top_level }
    }

    fn generate_function(&mut self, function: &tacky::Function, semantic: &SemanticData) -> Function {
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
                        self.generate_val(val),
                        Reg::Ax.into(),
                    ));
                    instructions.push(Instruction::Ret);
                }

                tacky::Instruction::Unary { op, src, dst } => match op {
                    tacky::UnaryOp::Not => {
                        instructions.push(Instruction::Cmp(
                            semantic.val_asm_ty(src),
                            Operand::Imm(0),
                            self.generate_val(src),
                        ));
                        instructions.push(Instruction::Mov(
                            semantic.val_asm_ty(dst),
                            Operand::Imm(0),
                            self.generate_val(dst),
                        ));
                        instructions.push(Instruction::SetCC(CondCode::E, self.generate_val(dst)));
                    }
                    _ => {
                        let ty = semantic.val_asm_ty(src);
                        instructions.push(Instruction::Mov(ty, self.generate_val(src), self.generate_val(dst)));
                        instructions.push(Instruction::Unary(ty, op.to_asm(), self.generate_val(dst)));
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
                        instructions.push(Instruction::Mov(ty, self.generate_val(src1), Reg::Ax.into()));
                        if semantic.is_signed(src1) {
                            instructions.push(Instruction::Cdq(ty));
                            instructions.push(Instruction::Idiv(ty, self.generate_val(src2)));
                        } else {
                            instructions.push(Instruction::Mov(ty, Operand::Imm(0), Reg::Dx.into()));
                            instructions.push(Instruction::Div(ty, self.generate_val(src2)));
                        }
                        instructions.push(Instruction::Mov(ty, Reg::Ax.into(), self.generate_val(dst)));
                    }
                    tacky::BinaryOp::Reminder => {
                        let ty = semantic.val_asm_ty(src1);
                        instructions.push(Instruction::Mov(ty, self.generate_val(src1), Reg::Ax.into()));
                        if semantic.is_signed(src1) {
                            instructions.push(Instruction::Cdq(ty));
                            instructions.push(Instruction::Idiv(ty, self.generate_val(src2)));
                        } else {
                            instructions.push(Instruction::Mov(ty, Operand::Imm(0), Reg::Dx.into()));
                            instructions.push(Instruction::Div(ty, self.generate_val(src2)));
                        }
                        instructions.push(Instruction::Mov(ty, Reg::Dx.into(), self.generate_val(dst)));
                    }
                    tacky::BinaryOp::ShiftLeft => {
                        let ty = semantic.val_asm_ty(src1);
                        instructions.push(Instruction::Mov(ty, self.generate_val(src1), self.generate_val(dst)));
                        instructions.push(Instruction::Mov(ty, self.generate_val(src2), Reg::Cx.into()));
                        if semantic.is_signed(src1) {
                            instructions.push(Instruction::Sal(ty, self.generate_val(dst)));
                        } else {
                            instructions.push(Instruction::Shl(ty, self.generate_val(dst)));
                        }
                    }
                    tacky::BinaryOp::ShiftRight => {
                        let ty = semantic.val_asm_ty(src1);
                        instructions.push(Instruction::Mov(ty, self.generate_val(src1), self.generate_val(dst)));
                        instructions.push(Instruction::Mov(ty, self.generate_val(src2), Reg::Cx.into()));
                        if semantic.is_signed(src1) {
                            instructions.push(Instruction::Sar(ty, self.generate_val(dst)));
                        } else {
                            instructions.push(Instruction::Shr(ty, self.generate_val(dst)));
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
                            self.generate_val(src2),
                            self.generate_val(src1),
                        ));
                        instructions.push(Instruction::Mov(
                            semantic.val_asm_ty(dst),
                            Operand::Imm(0),
                            self.generate_val(dst),
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
                        instructions.push(Instruction::SetCC(cond, self.generate_val(dst)));
                    }
                    _ => {
                        let ty = semantic.val_asm_ty(src1);
                        instructions.push(Instruction::Mov(ty, self.generate_val(src1), self.generate_val(dst)));
                        instructions.push(Instruction::Binary(
                            ty,
                            op.to_asm(),
                            self.generate_val(src2),
                            self.generate_val(dst),
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
                        self.generate_val(cond),
                    ));
                    instructions.push(Instruction::JmpCC(CondCode::E, target.clone()));
                }
                tacky::Instruction::JumpIfNotZero { cond, target } => {
                    instructions.push(Instruction::Cmp(
                        semantic.val_asm_ty(cond),
                        Operand::Imm(0),
                        self.generate_val(cond),
                    ));
                    instructions.push(Instruction::JmpCC(CondCode::NE, target.clone()));
                }
                tacky::Instruction::Copy { src, dst } => {
                    instructions.push(Instruction::Mov(
                        semantic.val_asm_ty(src),
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::Label(l) => {
                    instructions.push(Instruction::Label(l.clone()));
                }
                tacky::Instruction::FnCall { name, args, dst } => {
                    self.generate_call(&mut instructions, semantic, name, args, dst);
                }
                tacky::Instruction::SignExtend { src, dst } => {
                    instructions.push(Instruction::Movsx(self.generate_val(src), self.generate_val(dst)));
                }
                tacky::Instruction::Truncate { src, dst } => {
                    instructions.push(Instruction::Mov(
                        AsmType::Longword,
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::ZeroExtend { src, dst } => {
                    instructions.push(Instruction::MovZeroExtend(self.generate_val(src), self.generate_val(dst)));
                }

                tacky::Instruction::DoubleToInt { .. } => todo!(),
                tacky::Instruction::DoubleToUInt { .. } => todo!(),
                tacky::Instruction::IntToDouble { .. } => todo!(),
                tacky::Instruction::UIntToDouble { .. } => todo!(),
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

        let stack_size = self.replace_pseudo_registers(&mut instructions, &backend_symbols);
        let instructions = self.fixup_instructions(instructions, stack_size);

        Function {
            name: function.name.clone(),
            global: function.global,
            instructions,
        }
    }

    fn generate_static_variable(
        &mut self,
        var: &tacky::StaticVariable,
        semantic: &SemanticData,
    ) -> StaticVariable {
        StaticVariable {
            name: var.name.clone(),
            global: var.global,
            init: var.init,
            alignment: match semantic.symbol_asm_ty(&var.name) {
                AsmType::Longword => 4,
                AsmType::Quadword | AsmType::Double => 8,
            },
        }
    }

    fn generate_call(
        &mut self,
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
                Reg::SP.into(),
            ))
        }

        let register_args = args.iter().take(6);
        for (reg, val) in ARG_REGISTERS.iter().zip(register_args) {
            instructions.push(Instruction::Mov(
                semantic.val_asm_ty(val),
                self.generate_val(val),
                Operand::Reg(*reg),
            ));
        }

        let stack_args = args.iter().skip(6);
        for val in stack_args.clone().rev() {
            let operand = self.generate_val(val);
            let ty = semantic.val_asm_ty(val);
            if matches!(operand, Operand::Imm(_) | Operand::Reg(_)) || matches!(ty, AsmType::Quadword) {
                instructions.push(Instruction::Push(operand))
            } else {
                instructions.push(Instruction::Mov(ty, operand, Reg::Ax.into()));
                instructions.push(Instruction::Push(Reg::Ax.into()));
            }
        }

        instructions.push(Instruction::Call(name.clone()));

        let bytes_to_remove = 8 * stack_args.count() as i64 + padding;
        if bytes_to_remove != 0 {
            instructions.push(Instruction::Binary(
                AsmType::Quadword,
                BinaryOp::Add,
                Operand::Imm(bytes_to_remove),
                Reg::SP.into(),
            ));
        }
        instructions.push(Instruction::Mov(
            semantic.val_asm_ty(dst),
            Reg::Ax.into(),
            self.generate_val(dst),
        ));
    }

    fn replace_pseudo_registers(
        &mut self,
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
                        AsmType::Double => todo!()
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

    fn fixup_instructions(&mut self, instructions: Vec<Instruction>, stack_size: i64) -> Vec<Instruction> {
        let mut fixed = Vec::with_capacity(instructions.len() + 1);

        // Trick to align to the next multiple of 16 or same value if it's already there:
        // https://math.stackexchange.com/a/291494
        let stack_size = ((stack_size - 1) | 15) + 1;
        fixed.push(Instruction::Binary(
            AsmType::Quadword,
            BinaryOp::Sub,
            Operand::Imm(stack_size),
            Reg::SP.into(),
        ));

        for instruction in instructions.into_iter() {
            match instruction {
                Instruction::Mov(ty, src, dst) => {
                    let src = if let Operand::Imm(v) = src {
                        if v as u64 > i32::MAX as u64 {
                            let value = match ty {
                                AsmType::Longword => (v as i32) as i64,
                                AsmType::Quadword => v,
                                AsmType::Double => todo!()
                            };
                            fixed.push(Instruction::Mov(ty, Operand::Imm(value), Reg::R10.into()));
                            Reg::R10.into()
                        } else {
                            src
                        }
                    } else if src.is_mem() && dst.is_mem() {
                        fixed.push(Instruction::Mov(ty, src, Reg::R10.into()));
                        Reg::R10.into()
                    } else {
                        src
                    };
                    fixed.push(Instruction::Mov(ty, src, dst));
                }
                Instruction::Movsx(src, dst) => {
                    let src = if let Operand::Imm(value) = src {
                        fixed.push(Instruction::Mov(
                            AsmType::Longword,
                            Operand::Imm(value),
                            Reg::R10.into(),
                        ));
                        Reg::R10.into()
                    } else {
                        src
                    };
                    if dst.is_mem() {
                        fixed.push(Instruction::Movsx(src, Reg::R11.into()));
                        fixed.push(Instruction::Mov(AsmType::Quadword, Reg::R11.into(), dst));
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
                                fixed.push(Instruction::Mov(ty, left, Reg::R10.into()));
                                Reg::R10.into()
                            } else {
                                left
                            }
                        } else if left.is_mem() && right.is_mem() {
                            fixed.push(Instruction::Mov(ty, left, Reg::R10.into()));
                            Reg::R10.into()
                        } else {
                            left
                        };
                        if matches!(op, BinaryOp::Mul) && right.is_mem() {
                            fixed.push(Instruction::Mov(ty, right.clone(), Reg::R11.into()));
                            fixed.push(Instruction::Binary(
                                ty,
                                BinaryOp::Mul,
                                left,
                                Reg::R11.into(),
                            ));
                            fixed.push(Instruction::Mov(ty, Reg::R11.into(), right));
                        } else {
                            fixed.push(Instruction::Binary(ty, op, left, right));
                        }
                    }
                Instruction::Cmp(ty, left, right) => {
                    let left = if let Operand::Imm(v) = left {
                        if v as u64 > i32::MAX as u64 {
                            fixed.push(Instruction::Mov(ty, left, Reg::R10.into()));
                            Reg::R10.into()
                        } else {
                            left
                        }
                    } else if left.is_mem() && right.is_mem() {
                        fixed.push(Instruction::Mov(ty, left, Reg::R10.into()));
                        Reg::R10.into()
                    } else {
                        left
                    };

                    let right = if let Operand::Imm(value) = right {
                        fixed.push(Instruction::Mov(ty, Operand::Imm(value), Reg::R11.into()));
                        Reg::R11.into()
                    } else {
                        right
                    };
                    fixed.push(Instruction::Cmp(ty, left, right));
                }
                Instruction::Idiv(ty, Operand::Imm(value))
                | Instruction::Div(ty, Operand::Imm(value)) => {
                    fixed.push(Instruction::Mov(ty, Operand::Imm(value), Reg::R10.into()));
                    fixed.push(Instruction::Idiv(ty, Reg::R10.into()));
                }
                Instruction::Push(Operand::Imm(value)) => {
                    let value = if value as u64 > i32::MAX as u64 {
                        fixed.push(Instruction::Mov(
                            AsmType::Quadword,
                            Operand::Imm(value),
                            Reg::R10.into(),
                        ));
                        Reg::R10.into()
                    } else {
                        Operand::Imm(value)
                    };
                    fixed.push(Instruction::Push(value));
                }
                Instruction::MovZeroExtend(src, Operand::Reg(dst)) => {
                    fixed.push(Instruction::Mov(AsmType::Longword, src, Operand::Reg(dst)));
                }
                Instruction::MovZeroExtend(src, dst) => {
                    fixed.push(Instruction::Mov(AsmType::Longword, src, Reg::R11.into()));
                    fixed.push(Instruction::Mov(AsmType::Quadword, Reg::R11.into(), dst));
                }
                other => fixed.push(other),
            }
        }
        fixed
    }

    fn generate_val(&mut self, val: &tacky::Val) -> Operand {
        match val {
            tacky::Val::Constant(value) => {
                if let Constant::Double(double) = value {
                    let key = double.to_bits();
                    let existing_constant = self.doubles.get(&key);
                    let name = match existing_constant {
                        Some(name) => name.clone(),
                        None => {
                            let name = format!("$double_{}", self.doubles.len());
                            self.doubles.insert(key, name.clone());
                            name
                        }
                    };
                    Operand::Data(name)
                } else {
                    Operand::Imm(value.as_u64() as i64)
                }

            }
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

pub fn generate(program: &tacky::Program) -> Program {
    let mut compiler = Compiler {
        doubles: HashMap::new(),
        double_counter: 0,
    };
    compiler.generate(program)
}