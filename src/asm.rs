pub mod ir;
pub mod pretty;
mod register_allocation;

use crate::alignment::align_offset;
use crate::asm::ir::{
    AsmType, BinaryOp, CondCode, Function, Instruction, Operand, Program, Reg, StaticConstant,
    StaticVariable, TopLevel, UnaryOp,
};
use crate::ast::Constant;
use crate::semantic::{AggregateType, Attributes, SemanticData, StaticInit, Type, TypeEntry};
use crate::symbol::Symbol;
use crate::tacky;
use std::collections::HashMap;

const INT_ARG_REGISTERS: [Reg; 6] = [Reg::Di, Reg::Si, Reg::Dx, Reg::Cx, Reg::R8, Reg::R9];
const SSE_ARG_REGISTERS: [Reg; 8] = [
    Reg::XMM0,
    Reg::XMM1,
    Reg::XMM2,
    Reg::XMM3,
    Reg::XMM4,
    Reg::XMM5,
    Reg::XMM6,
    Reg::XMM7,
];

const INT_RETURN_REGISTERS: [Reg; 2] = [Reg::Ax, Reg::Dx];
const SSE_RETURN_REGISTERS: [Reg; 2] = [Reg::XMM0, Reg::XMM1];

enum BackendSymbolData {
    Obj {
        ty: AsmType,
        is_static: bool,
        is_const: bool,
    },
    Fn,
}

enum ParamClass {
    Integer,
    Sse,
    Memory,
}

type BackendSymbolTable = HashMap<Symbol, BackendSymbolData>;

impl SemanticData {
    fn val_asm_ty(&self, val: &tacky::Val) -> AsmType {
        match val {
            tacky::Val::Constant(Constant::Char(_) | Constant::UChar(_)) => AsmType::Byte,
            tacky::Val::Constant(Constant::Int(_) | Constant::UInt(_)) => AsmType::Longword,
            tacky::Val::Constant(Constant::Long(_) | Constant::ULong(_)) => AsmType::Quadword,
            tacky::Val::Constant(Constant::Double(_)) => AsmType::Double,
            tacky::Val::Var(name) => self.symbol_ty(name).to_asm(self),
        }
    }
}

impl Type {
    fn to_asm(&self, semantics: &SemanticData) -> AsmType {
        match self {
            Type::UChar | Type::SChar | Type::Char => AsmType::Byte,
            Type::Int | Type::UInt => AsmType::Longword,
            Type::Long | Type::ULong => AsmType::Quadword,
            Type::Double => AsmType::Double,
            Type::Void => unreachable!(),
            Type::Function(_) => unreachable!(),
            Type::Pointer(_) => AsmType::Quadword,
            Type::Array(inner, length) => {
                let size = inner.size(semantics) * length;
                let inner_asm_ty = inner.to_asm(semantics);
                let alignment = if size < 16 {
                    inner_asm_ty.alignment()
                } else {
                    16
                };
                AsmType::ByteArray { size, alignment }
            }
            Type::Struct(name) | Type::Union(name) => {
                let (size, alignment) =
                    if let Some(TypeEntry::Complete(aggregate)) = semantics.type_defs.get(name) {
                        (aggregate.size, aggregate.alignment)
                    } else {
                        // In case of incomplete types, these are dummy values.
                        (0, 0)
                    };
                AsmType::ByteArray { size, alignment }
            }
        }
    }
}

struct TypedOperand {
    operand: Operand,
    ty: AsmType,
}

struct FnArgs {
    int_reg_args: Vec<TypedOperand>,
    sse_reg_args: Vec<TypedOperand>,
    stack_args: Vec<TypedOperand>,
}

struct FnReturn {
    int_values: Vec<TypedOperand>,
    sse_values: Vec<TypedOperand>,
    in_memory: bool,
}

struct Compiler {
    doubles: HashMap<u64, Symbol>,
    label_counter: usize,
    semantics: SemanticData,
}

impl Compiler {
    fn generate(&mut self, program: &tacky::Program) -> Program {
        let mut top_level = Vec::new();
        for element in &program.top_level {
            match element {
                tacky::TopLevel::Function(f) => {
                    top_level.push(TopLevel::Function(self.generate_function(f)))
                }
                tacky::TopLevel::Variable(v) => {
                    top_level.push(TopLevel::Variable(self.generate_static_variable(v)))
                }
                tacky::TopLevel::Constant(c) => {
                    top_level.push(TopLevel::Constant(self.generate_constant(c)))
                }
            }
        }

        let mut backend_symbols = self.make_backend_symbols(&program.semantics);

        for (key, name) in &self.doubles {
            // -0.0 is used to negate floats by using xorpd instruction.
            // this requires 16 bit alignment. Hence, this hack.
            let alignment = if *key == (-0.0_f64).to_bits() { 16 } else { 8 };
            top_level.push(TopLevel::Constant(StaticConstant {
                name: name.clone(),
                alignment,
                init: StaticInit::Double(f64::from_bits(*key)),
            }));
            backend_symbols.insert(
                name.clone(),
                BackendSymbolData::Obj {
                    ty: AsmType::Double,
                    is_static: true,
                    is_const: true,
                },
            );
        }

        for tl in &mut top_level {
            if let TopLevel::Function(function) = tl {
                let stack_size = self.replace_pseudo_operands(function, &backend_symbols);
                self.fixup_instructions(function, stack_size);
            }
        }

        Program { top_level }
    }

    fn generate_function(&mut self, function: &tacky::Function) -> Function {
        let mut instructions = Vec::new();
        let params: Vec<tacky::Val> = function
            .params
            .iter()
            .cloned()
            .map(tacky::Val::Var)
            .collect();
        let return_in_memory = self.does_return_in_memory(&function.name);

        self.assign_parameters(&mut instructions, params, return_in_memory);

        for tacky_instruction in &function.body {
            match tacky_instruction {
                tacky::Instruction::Return(val) => {
                    self.generate_return(&mut instructions, val);
                }

                tacky::Instruction::Unary { op, src, dst } => {
                    let ty = self.semantics.val_asm_ty(src);
                    let is_double = matches!(ty, AsmType::Double);
                    match op {
                        tacky::UnaryOp::Increment => {
                            let one = if is_double {
                                self.make_double_constant(1.0)
                            } else {
                                Operand::Imm(1)
                            };
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Binary(
                                ty,
                                BinaryOp::Add,
                                one,
                                self.generate_val(dst),
                            ));
                        }
                        tacky::UnaryOp::Decrement => {
                            let one = if is_double {
                                self.make_double_constant(1.0)
                            } else {
                                Operand::Imm(1)
                            };
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Binary(
                                ty,
                                BinaryOp::Sub,
                                one,
                                self.generate_val(dst),
                            ));
                        }
                        tacky::UnaryOp::Not if is_double => {
                            instructions.push(Instruction::Binary(
                                AsmType::Double,
                                BinaryOp::Xor,
                                Reg::XMM0.into(),
                                Reg::XMM0.into(),
                            ));
                            instructions.push(Instruction::Cmp(
                                ty,
                                self.generate_val(src),
                                Reg::XMM0.into(),
                            ));
                            instructions.push(Instruction::Mov(
                                self.semantics.val_asm_ty(dst),
                                Operand::Imm(0),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Mov(
                                AsmType::Longword,
                                Operand::Imm(0),
                                Reg::Ax.into(),
                            ));
                            instructions.push(Instruction::Mov(
                                AsmType::Longword,
                                Operand::Imm(0),
                                Reg::Cx.into(),
                            ));
                            instructions.push(Instruction::SetCC(CondCode::E, Reg::Ax.into()));
                            instructions.push(Instruction::SetCC(CondCode::NP, Reg::Cx.into()));
                            instructions.push(Instruction::Binary(
                                AsmType::Longword,
                                BinaryOp::And,
                                Reg::Ax.into(),
                                Reg::Cx.into(),
                            ));
                            instructions.push(Instruction::Mov(
                                AsmType::Longword,
                                Reg::Cx.into(),
                                self.generate_val(dst),
                            ));
                        }
                        tacky::UnaryOp::Not => {
                            instructions.push(Instruction::Cmp(
                                ty,
                                Operand::Imm(0),
                                self.generate_val(src),
                            ));
                            instructions.push(Instruction::Mov(
                                self.semantics.val_asm_ty(dst),
                                Operand::Imm(0),
                                self.generate_val(dst),
                            ));
                            instructions
                                .push(Instruction::SetCC(CondCode::E, self.generate_val(dst)));
                        }
                        tacky::UnaryOp::Negate if is_double => {
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src),
                                self.generate_val(dst),
                            ));
                            let negative_zero = self.make_double_constant(-0.0);
                            instructions.push(Instruction::Binary(
                                ty,
                                BinaryOp::Xor,
                                negative_zero,
                                self.generate_val(dst),
                            ));
                        }

                        tacky::UnaryOp::Complement | tacky::UnaryOp::Negate => {
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Unary(
                                ty,
                                op.to_asm(),
                                self.generate_val(dst),
                            ));
                        }
                    }
                }

                tacky::Instruction::Binary {
                    op,
                    src1,
                    src2,
                    dst,
                } => {
                    let ty = self.semantics.val_asm_ty(src1);
                    let is_double = matches!(ty, AsmType::Double);
                    match op {
                        tacky::BinaryOp::Divide if !is_double => {
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src1),
                                Reg::Ax.into(),
                            ));
                            if self.semantics.is_signed(src1) {
                                instructions.push(Instruction::Cdq(ty));
                                instructions.push(Instruction::Idiv(ty, self.generate_val(src2)));
                            } else {
                                instructions.push(Instruction::Mov(
                                    ty,
                                    Operand::Imm(0),
                                    Reg::Dx.into(),
                                ));
                                instructions.push(Instruction::Div(ty, self.generate_val(src2)));
                            }
                            instructions.push(Instruction::Mov(
                                ty,
                                Reg::Ax.into(),
                                self.generate_val(dst),
                            ));
                        }
                        tacky::BinaryOp::Reminder => {
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src1),
                                Reg::Ax.into(),
                            ));
                            if self.semantics.is_signed(src1) {
                                instructions.push(Instruction::Cdq(ty));
                                instructions.push(Instruction::Idiv(ty, self.generate_val(src2)));
                            } else {
                                instructions.push(Instruction::Mov(
                                    ty,
                                    Operand::Imm(0),
                                    Reg::Dx.into(),
                                ));
                                instructions.push(Instruction::Div(ty, self.generate_val(src2)));
                            }
                            instructions.push(Instruction::Mov(
                                ty,
                                Reg::Dx.into(),
                                self.generate_val(dst),
                            ));
                        }
                        tacky::BinaryOp::ShiftLeft | tacky::BinaryOp::ShiftRight => {
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src1),
                                self.generate_val(dst),
                            ));
                            let bit_count = self.generate_val(src2);
                            let bit_count = if let Operand::Imm(_) = bit_count {
                                bit_count
                            } else {
                                instructions.push(Instruction::Mov(AsmType::Byte, bit_count, Reg::Cx.into()));
                                Reg::Cx.into()
                            };
                            let is_signed = self.semantics.is_signed(src1);
                            let shift_op = match op {
                                tacky::BinaryOp::ShiftLeft => if is_signed { BinaryOp::Sal } else { BinaryOp::Shl },
                                tacky::BinaryOp::ShiftRight => if is_signed { BinaryOp::Sar } else { BinaryOp::Shr }
                                _ => unreachable!(),
                            };
                            instructions.push(Instruction::Binary(ty, shift_op, bit_count, self.generate_val(dst)));

                        }
                        tacky::BinaryOp::Equal
                        | tacky::BinaryOp::NotEqual
                        | tacky::BinaryOp::GreaterThan
                        | tacky::BinaryOp::GreaterOrEqual
                        | tacky::BinaryOp::LessThan
                        | tacky::BinaryOp::LessOrEqual => {
                            instructions.push(Instruction::Cmp(
                                ty,
                                self.generate_val(src2),
                                self.generate_val(src1),
                            ));
                            instructions.push(Instruction::Mov(
                                self.semantics.val_asm_ty(dst),
                                Operand::Imm(0),
                                self.generate_val(dst),
                            ));
                            let is_double = matches!(ty, AsmType::Double);
                            let is_unsigned = !self.semantics.is_signed(src1);
                            let cond = match (op, is_unsigned || is_double) {
                                (tacky::BinaryOp::Equal, _) => CondCode::E,
                                (tacky::BinaryOp::NotEqual, _) => CondCode::NE,
                                (tacky::BinaryOp::GreaterThan, false) => CondCode::G,
                                (tacky::BinaryOp::GreaterThan, true) => CondCode::A,
                                (tacky::BinaryOp::GreaterOrEqual, false) => CondCode::GE,
                                (tacky::BinaryOp::GreaterOrEqual, true) => CondCode::AE,
                                (tacky::BinaryOp::LessThan, false) => CondCode::L,
                                (tacky::BinaryOp::LessThan, true) => CondCode::B,
                                (tacky::BinaryOp::LessOrEqual, false) => CondCode::LE,
                                (tacky::BinaryOp::LessOrEqual, true) => CondCode::BE,
                                _ => unreachable!(),
                            };
                            // Handling NaN:
                            match op {
                                tacky::BinaryOp::NotEqual if is_double => {
                                    // TODO: These moves are not necessary if using byte types.
                                    instructions.push(Instruction::Mov(AsmType::Longword, Operand::Imm(0), Reg::Ax.into()));
                                    instructions.push(Instruction::Mov(AsmType::Longword, Operand::Imm(0), Reg::Cx.into()));
                                    instructions.push(Instruction::SetCC(cond, Reg::Ax.into()));
                                    instructions.push(Instruction::SetCC(CondCode::P, Reg::Cx.into()));
                                    instructions.push(Instruction::Binary(AsmType::Longword, BinaryOp::Or, Reg::Ax.into(), Reg::Cx.into()));
                                    instructions.push(Instruction::Mov(AsmType::Longword, Reg::Cx.into(), self.generate_val(dst)));
                                }
                                _ if is_double => {
                                    instructions.push(Instruction::Mov(AsmType::Longword, Operand::Imm(0), Reg::Ax.into()));
                                    instructions.push(Instruction::Mov(AsmType::Longword, Operand::Imm(0), Reg::Cx.into()));
                                    instructions.push(Instruction::SetCC(cond, Reg::Ax.into()));
                                    instructions.push(Instruction::SetCC(CondCode::NP, Reg::Cx.into()));
                                    instructions.push(Instruction::Binary(AsmType::Longword, BinaryOp::And, Reg::Ax.into(), Reg::Cx.into()));
                                    instructions.push(Instruction::Mov(AsmType::Longword, Reg::Cx.into(), self.generate_val(dst)));
                                }
                                _ => {
                                    instructions.push(Instruction::SetCC(cond, self.generate_val(dst)));
                                }
                            }
                        }

                        tacky::BinaryOp::Add
                        | tacky::BinaryOp::Subtract
                        | tacky::BinaryOp::Multiply
                        | tacky::BinaryOp::Divide // Covers only if double
                        | tacky::BinaryOp::BinAnd
                        | tacky::BinaryOp::BinOr
                        | tacky::BinaryOp::BinXor => {
                            let ty = self.semantics.val_asm_ty(src1);
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src1),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Binary(
                                ty,
                                op.to_asm(),
                                self.generate_val(src2),
                                self.generate_val(dst),
                            ));
                        }
                    }
                }

                tacky::Instruction::Jump { target } => {
                    instructions.push(Instruction::Jmp(target.clone()));
                }
                tacky::Instruction::JumpIfZero { cond, target } => {
                    if let AsmType::Double = self.semantics.val_asm_ty(cond) {
                        instructions.push(Instruction::Binary(
                            AsmType::Double,
                            BinaryOp::Xor,
                            Reg::XMM0.into(),
                            Reg::XMM0.into(),
                        ));
                        instructions.push(Instruction::Cmp(
                            self.semantics.val_asm_ty(cond),
                            Reg::XMM0.into(),
                            self.generate_val(cond),
                        ));
                    } else {
                        instructions.push(Instruction::Cmp(
                            self.semantics.val_asm_ty(cond),
                            Operand::Imm(0),
                            self.generate_val(cond),
                        ));
                    }
                    instructions.push(Instruction::JmpCC(CondCode::E, target.clone()));
                }
                tacky::Instruction::JumpIfNotZero { cond, target } => {
                    if let AsmType::Double = self.semantics.val_asm_ty(cond) {
                        instructions.push(Instruction::Binary(
                            AsmType::Double,
                            BinaryOp::Xor,
                            Reg::XMM0.into(),
                            Reg::XMM0.into(),
                        ));
                        instructions.push(Instruction::Cmp(
                            self.semantics.val_asm_ty(cond),
                            Reg::XMM0.into(),
                            self.generate_val(cond),
                        ));
                    } else {
                        instructions.push(Instruction::Cmp(
                            self.semantics.val_asm_ty(cond),
                            Operand::Imm(0),
                            self.generate_val(cond),
                        ));
                    }
                    instructions.push(Instruction::JmpCC(CondCode::NE, target.clone()));
                }
                tacky::Instruction::Copy { src, dst } => {
                    let src_ty = self.semantics.val_asm_ty(src);

                    if let AsmType::ByteArray { size, .. } = src_ty {
                        Self::copy_bytes(
                            &mut instructions,
                            self.generate_val(src),
                            self.generate_val(dst),
                            size,
                        );
                    } else {
                        instructions.push(Instruction::Mov(
                            src_ty,
                            self.generate_val(src),
                            self.generate_val(dst),
                        ));
                    }
                }
                tacky::Instruction::Label(l) => {
                    instructions.push(Instruction::Label(l.clone()));
                }
                tacky::Instruction::FnCall { name, args, dst } => {
                    self.generate_call(&mut instructions, name, args, dst);
                }
                tacky::Instruction::SignExtend { src, dst } => {
                    let asm_type1 = self.semantics.val_asm_ty(src);
                    let asm_type2 = self.semantics.val_asm_ty(dst);
                    instructions.push(Instruction::Movsx(
                        asm_type1,
                        self.generate_val(src),
                        asm_type2,
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::Truncate { src, dst } => {
                    instructions.push(Instruction::Mov(
                        self.semantics.val_asm_ty(dst),
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::ZeroExtend { src, dst } => {
                    instructions.push(Instruction::MovZeroExtend(
                        self.semantics.val_asm_ty(src),
                        self.generate_val(src),
                        self.semantics.val_asm_ty(dst),
                        self.generate_val(dst),
                    ));
                }

                tacky::Instruction::DoubleToInt { src, dst } => {
                    if let AsmType::Byte = self.semantics.val_asm_ty(dst) {
                        instructions.push(Instruction::Cvttsd2si(
                            AsmType::Longword,
                            self.generate_val(src),
                            Reg::Ax.into(),
                        ));
                        instructions.push(Instruction::Mov(
                            AsmType::Byte,
                            Reg::Ax.into(),
                            self.generate_val(dst),
                        ))
                    } else {
                        instructions.push(Instruction::Cvttsd2si(
                            self.semantics.val_asm_ty(dst),
                            self.generate_val(src),
                            self.generate_val(dst),
                        ));
                    }
                }
                tacky::Instruction::IntToDouble { src, dst } => {
                    if let AsmType::Byte = self.semantics.val_asm_ty(src) {
                        instructions.push(Instruction::Movsx(
                            AsmType::Byte,
                            self.generate_val(src),
                            AsmType::Longword,
                            Reg::Ax.into(),
                        ));
                        instructions.push(Instruction::Cvtsi2sd(
                            AsmType::Longword,
                            Reg::Ax.into(),
                            self.generate_val(dst),
                        ));
                    } else {
                        instructions.push(Instruction::Cvtsi2sd(
                            self.semantics.val_asm_ty(src),
                            self.generate_val(src),
                            self.generate_val(dst),
                        ));
                    }
                }
                tacky::Instruction::DoubleToUInt { src, dst } => {
                    match self.semantics.val_asm_ty(dst) {
                        AsmType::Byte => {
                            instructions.push(Instruction::Cvttsd2si(
                                AsmType::Longword,
                                self.generate_val(src),
                                Reg::Ax.into(),
                            ));
                            instructions.push(Instruction::Mov(
                                AsmType::Byte,
                                Reg::Ax.into(),
                                self.generate_val(dst),
                            ));
                        }
                        AsmType::Longword => {
                            instructions.push(Instruction::Cvttsd2si(
                                AsmType::Quadword,
                                self.generate_val(src),
                                Reg::Ax.into(),
                            ));
                            instructions.push(Instruction::Mov(
                                AsmType::Longword,
                                Reg::Ax.into(),
                                self.generate_val(dst),
                            ));
                        }
                        AsmType::Quadword => {
                            let upper_bound_u64 = (i64::MAX as u64) + 1;
                            let upper_bound_f64 = upper_bound_u64 as f64;
                            instructions.push(Instruction::Cmp(
                                AsmType::Double,
                                self.make_double_constant(upper_bound_f64),
                                self.generate_val(src),
                            ));
                            let out_of_range_label = self.make_label("d2u_out_of_range");
                            let end_label = self.make_label("d2u_end");
                            instructions
                                .push(Instruction::JmpCC(CondCode::AE, out_of_range_label.clone()));
                            instructions.push(Instruction::Cvttsd2si(
                                AsmType::Quadword,
                                self.generate_val(src),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Jmp(end_label.clone()));
                            instructions.push(Instruction::Label(out_of_range_label));
                            instructions.push(Instruction::Mov(
                                AsmType::Double,
                                self.generate_val(src),
                                Reg::XMM14.into(),
                            ));
                            instructions.push(Instruction::Binary(
                                AsmType::Double,
                                BinaryOp::Sub,
                                self.make_double_constant(upper_bound_f64),
                                Reg::XMM14.into(),
                            ));
                            instructions.push(Instruction::Cvttsd2si(
                                AsmType::Quadword,
                                Reg::XMM14.into(),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Mov(
                                AsmType::Quadword,
                                Operand::Imm(upper_bound_u64 as i64),
                                Reg::Ax.into(),
                            ));
                            instructions.push(Instruction::Binary(
                                AsmType::Quadword,
                                BinaryOp::Add,
                                Reg::Ax.into(),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Label(end_label));
                        }
                        AsmType::Double | AsmType::ByteArray { .. } => unreachable!(),
                    }
                }
                tacky::Instruction::UIntToDouble { src, dst } => {
                    match self.semantics.val_asm_ty(src) {
                        AsmType::Byte => {
                            instructions.push(Instruction::MovZeroExtend(
                                AsmType::Byte,
                                self.generate_val(src),
                                AsmType::Longword,
                                Reg::Ax.into(),
                            ));
                            instructions.push(Instruction::Cvtsi2sd(
                                AsmType::Longword,
                                Reg::Ax.into(),
                                self.generate_val(dst),
                            ));
                        }
                        AsmType::Longword => {
                            instructions.push(Instruction::MovZeroExtend(
                                AsmType::Longword,
                                self.generate_val(src),
                                AsmType::Quadword,
                                Reg::Ax.into(),
                            ));
                            instructions.push(Instruction::Cvtsi2sd(
                                AsmType::Quadword,
                                Reg::Ax.into(),
                                self.generate_val(dst),
                            ));
                        }
                        AsmType::Quadword => {
                            instructions.push(Instruction::Cmp(
                                AsmType::Quadword,
                                Operand::Imm(0),
                                self.generate_val(src),
                            ));
                            let out_of_range_label = self.make_label("u2d_out_of_range");
                            let end_label = self.make_label("u2d_end");
                            instructions
                                .push(Instruction::JmpCC(CondCode::L, out_of_range_label.clone()));
                            instructions.push(Instruction::Cvtsi2sd(
                                AsmType::Quadword,
                                self.generate_val(src),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Jmp(end_label.clone()));
                            instructions.push(Instruction::Label(out_of_range_label));
                            instructions.push(Instruction::Mov(
                                AsmType::Quadword,
                                self.generate_val(src),
                                Reg::Ax.into(),
                            ));
                            instructions.push(Instruction::Mov(
                                AsmType::Quadword,
                                Reg::Ax.into(),
                                Reg::Dx.into(),
                            ));
                            instructions.push(Instruction::Mov(
                                AsmType::Quadword,
                                Operand::Imm(1),
                                Reg::Cx.into(),
                            ));
                            instructions.push(Instruction::Binary(
                                AsmType::Quadword,
                                BinaryOp::Shr,
                                Operand::Imm(1),
                                Reg::Dx.into(),
                            ));
                            instructions.push(Instruction::Binary(
                                AsmType::Quadword,
                                BinaryOp::And,
                                Operand::Imm(1),
                                Reg::Ax.into(),
                            ));
                            instructions.push(Instruction::Binary(
                                AsmType::Quadword,
                                BinaryOp::Or,
                                Reg::Ax.into(),
                                Reg::Dx.into(),
                            ));
                            instructions.push(Instruction::Cvtsi2sd(
                                AsmType::Quadword,
                                Reg::Dx.into(),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Binary(
                                AsmType::Double,
                                BinaryOp::Add,
                                self.generate_val(dst),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Label(end_label));
                        }
                        AsmType::Double | AsmType::ByteArray { .. } => unreachable!(),
                    }
                }
                tacky::Instruction::GetAddress { src, dst } => {
                    instructions.push(Instruction::Lea(
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::Load { ptr, dst } => {
                    instructions.push(Instruction::Mov(
                        AsmType::Quadword,
                        self.generate_val(ptr),
                        Reg::Ax.into(),
                    ));
                    let dst_ty = self.semantics.val_asm_ty(dst);
                    if let AsmType::ByteArray { size, .. } = dst_ty {
                        Self::copy_bytes(
                            &mut instructions,
                            Operand::Memory(Reg::Ax, 0),
                            self.generate_val(dst),
                            size,
                        );
                    } else {
                        instructions.push(Instruction::Mov(
                            dst_ty,
                            Operand::Memory(Reg::Ax, 0),
                            self.generate_val(dst),
                        ));
                    }
                }
                tacky::Instruction::Store { src, ptr } => {
                    instructions.push(Instruction::Mov(
                        AsmType::Quadword,
                        self.generate_val(ptr),
                        Reg::Ax.into(),
                    ));
                    let src_ty = self.semantics.val_asm_ty(src);
                    if let AsmType::ByteArray { size, .. } = src_ty {
                        Self::copy_bytes(
                            &mut instructions,
                            self.generate_val(src),
                            Operand::Memory(Reg::Ax, 0),
                            size,
                        );
                    } else {
                        instructions.push(Instruction::Mov(
                            src_ty,
                            self.generate_val(src),
                            Operand::Memory(Reg::Ax, 0),
                        ));
                    }
                }
                tacky::Instruction::AddPtr {
                    ptr,
                    index,
                    scale,
                    dst,
                } => {
                    instructions.push(Instruction::Mov(
                        AsmType::Quadword,
                        self.generate_val(ptr),
                        Reg::Ax.into(),
                    ));
                    if let tacky::Val::Constant(c) = index
                        && c.is_int()
                    {
                        let index = c.as_u64() as i64;
                        instructions.push(Instruction::Lea(
                            Operand::Memory(Reg::Ax, index * (*scale as i64)),
                            self.generate_val(dst),
                        ));
                    } else {
                        instructions.push(Instruction::Mov(
                            AsmType::Quadword,
                            self.generate_val(index),
                            Reg::Dx.into(),
                        ));
                        let indexing_scale = match *scale {
                            1 | 2 | 4 | 8 => *scale as u8,
                            _ => {
                                instructions.push(Instruction::Binary(
                                    AsmType::Quadword,
                                    BinaryOp::Mul,
                                    Operand::Imm(*scale as i64),
                                    Reg::Dx.into(),
                                ));
                                1
                            }
                        };
                        instructions.push(Instruction::Lea(
                            Operand::Indexed(Reg::Ax, Reg::Dx, indexing_scale),
                            self.generate_val(dst),
                        ));
                    }
                }
                tacky::Instruction::CopyToOffset { src, dst, offset } => {
                    let src_ty = self.semantics.val_asm_ty(src);
                    if let AsmType::ByteArray { size, .. } = src_ty {
                        Self::copy_bytes(
                            &mut instructions,
                            self.generate_val(src),
                            Operand::PseudoMem(dst.clone(), *offset),
                            size,
                        );
                    } else {
                        instructions.push(Instruction::Mov(
                            src_ty,
                            self.generate_val(src),
                            Operand::PseudoMem(dst.clone(), *offset),
                        ));
                    }
                }
                tacky::Instruction::CopyFromOffset { src, dst, offset } => {
                    let dst_ty = self.semantics.val_asm_ty(dst);
                    if let AsmType::ByteArray { size, .. } = dst_ty {
                        Self::copy_bytes(
                            &mut instructions,
                            Operand::PseudoMem(src.clone(), *offset),
                            self.generate_val(dst),
                            size,
                        );
                    } else {
                        instructions.push(Instruction::Mov(
                            dst_ty,
                            Operand::PseudoMem(src.clone(), *offset),
                            self.generate_val(dst),
                        ));
                    }
                }
            }
        }

        Function {
            name: function.name.clone(),
            global: function.global,
            instructions,
        }
    }

    fn does_return_in_memory(&self, function_name: &Symbol) -> bool {
        let Type::Function(function_ty) = self.semantics.symbol_ty(function_name) else {
            panic!("Function does not have a function type")
        };
        matches!(
            self.classify_type(&function_ty.ret).first(),
            Some(ParamClass::Memory)
        )
    }

    fn assign_parameters(
        &mut self,
        instructions: &mut Vec<Instruction>,
        params: Vec<tacky::Val>,
        return_in_memory: bool,
    ) {
        let FnArgs {
            int_reg_args,
            sse_reg_args,
            stack_args,
        } = self.classify_parameters(&params, return_in_memory);

        let to_skip = if return_in_memory {
            instructions.push(Instruction::Mov(
                AsmType::Quadword,
                Reg::Di.into(),
                Operand::Memory(Reg::BP, -8),
            ));
            1
        } else {
            0
        };
        for (&reg, TypedOperand { ty, operand }) in
            INT_ARG_REGISTERS.iter().skip(to_skip).zip(int_reg_args)
        {
            if let AsmType::ByteArray { size, .. } = ty {
                Self::copy_bytes_from_reg(instructions, reg, operand, size as i64);
            } else {
                instructions.push(Instruction::Mov(ty, reg.into(), operand));
            }
        }

        for (reg, TypedOperand { ty, operand }) in SSE_ARG_REGISTERS.iter().zip(sse_reg_args) {
            instructions.push(Instruction::Mov(ty, Operand::Reg(*reg), operand));
        }

        let mut offset = 16;
        for TypedOperand { ty, operand } in stack_args {
            if let AsmType::ByteArray { size, .. } = ty {
                Self::copy_bytes(
                    instructions,
                    Operand::Memory(Reg::BP, offset),
                    operand,
                    size,
                );
            } else {
                instructions.push(Instruction::Mov(
                    ty,
                    Operand::Memory(Reg::BP, offset),
                    operand,
                ));
            }
            offset += 8;
        }
    }

    fn copy_bytes(instructions: &mut Vec<Instruction>, src: Operand, dst: Operand, size: usize) {
        let mut part_offset = 0;
        while part_offset < size {
            let remaining_bytes = size - part_offset;
            let ty = match remaining_bytes {
                1..4 => AsmType::Byte,
                4..8 => AsmType::Longword,
                _ => AsmType::Quadword,
            };
            instructions.push(Instruction::Mov(
                ty,
                src.add_offset(part_offset as i64),
                dst.add_offset(part_offset as i64),
            ));
            part_offset += ty.size();
        }
    }

    fn copy_bytes_to_reg(
        instructions: &mut Vec<Instruction>,
        src: Operand,
        dst: Reg,
        byte_count: i64,
    ) {
        let mut offset = byte_count - 1;
        while offset >= 0 {
            instructions.push(Instruction::Mov(
                AsmType::Byte,
                src.add_offset(offset),
                dst.into(),
            ));
            if offset > 0 {
                instructions.push(Instruction::Binary(
                    AsmType::Quadword,
                    BinaryOp::Shl,
                    Operand::Imm(8),
                    dst.into(),
                ))
            }
            offset -= 1;
        }
    }

    fn copy_bytes_from_reg(
        instructions: &mut Vec<Instruction>,
        src: Reg,
        dst: Operand,
        byte_count: i64,
    ) {
        let mut offset = 0;
        while offset < byte_count {
            instructions.push(Instruction::Mov(
                AsmType::Byte,
                src.into(),
                dst.add_offset(offset),
            ));
            if offset < byte_count - 1 {
                instructions.push(Instruction::Binary(
                    AsmType::Quadword,
                    BinaryOp::Shr,
                    Operand::Imm(8),
                    src.into(),
                ))
            }
            offset += 1;
        }
    }

    fn make_backend_symbols(&self, semantic: &SemanticData) -> BackendSymbolTable {
        let mut backend_symbols = HashMap::new();
        for (symbol, data) in semantic.symbols.iter() {
            match data.attrs {
                Attributes::Function { .. } => {
                    backend_symbols.insert(symbol.clone(), BackendSymbolData::Fn);
                }
                Attributes::Static { .. } => {
                    backend_symbols.insert(
                        symbol.clone(),
                        BackendSymbolData::Obj {
                            ty: semantic.symbol_ty(symbol).to_asm(semantic),
                            is_static: true,
                            is_const: false,
                        },
                    );
                }
                Attributes::Const { .. } => {
                    backend_symbols.insert(
                        symbol.clone(),
                        BackendSymbolData::Obj {
                            ty: semantic.symbol_ty(symbol).to_asm(semantic),
                            is_static: true,
                            is_const: true,
                        },
                    );
                }
                Attributes::Local => {
                    backend_symbols.insert(
                        symbol.clone(),
                        BackendSymbolData::Obj {
                            ty: semantic.symbol_ty(symbol).to_asm(semantic),
                            is_static: false,
                            is_const: false,
                        },
                    );
                }
            }
        }
        backend_symbols
    }

    fn generate_constant(&mut self, c: &tacky::StaticConstant) -> StaticConstant {
        StaticConstant {
            name: c.name.clone(),
            alignment: c.ty.to_asm(&self.semantics).alignment(),
            init: c.init.clone(),
        }
    }

    fn generate_static_variable(&mut self, var: &tacky::StaticVariable) -> StaticVariable {
        let self1 = &self.semantics;
        let symbol = &var.name;
        StaticVariable {
            name: var.name.clone(),
            global: var.global,
            init: var.init.clone(),
            alignment: self1.symbol_ty(symbol).to_asm(&self.semantics).alignment(),
        }
    }

    fn generate_call(
        &mut self,
        instructions: &mut Vec<Instruction>,
        name: &Symbol,
        args: &[tacky::Val],
        dst: &Option<tacky::Val>,
    ) {
        let mut return_spec = FnReturn {
            int_values: vec![],
            sse_values: vec![],
            in_memory: false,
        };
        let mut reg_index = 0;

        if let Some(dst) = dst {
            return_spec = self.classify_return_value(dst);
            if return_spec.in_memory {
                let operand = self.generate_val(dst);
                instructions.push(Instruction::Lea(operand, Reg::Di.into()));
                reg_index = 1;
            }
        }

        let FnArgs {
            int_reg_args,
            sse_reg_args,
            stack_args,
        } = self.classify_parameters(args, return_spec.in_memory);

        let padding = if stack_args.len().is_multiple_of(2) {
            0
        } else {
            8
        };
        if padding != 0 {
            instructions.push(Instruction::Binary(
                AsmType::Quadword,
                BinaryOp::Sub,
                Operand::Imm(padding),
                Reg::SP.into(),
            ))
        }
        let bytes_to_remove = 8 * stack_args.len() as i64 + padding;

        for (&reg, TypedOperand { ty, operand }) in
            INT_ARG_REGISTERS.iter().skip(reg_index).zip(int_reg_args)
        {
            if let AsmType::ByteArray { size, .. } = ty {
                Self::copy_bytes_to_reg(instructions, operand, reg, size as i64);
            } else {
                instructions.push(Instruction::Mov(ty, operand, reg.into()));
            }
        }

        for (reg, TypedOperand { ty, operand }) in SSE_ARG_REGISTERS.iter().zip(sse_reg_args) {
            instructions.push(Instruction::Mov(ty, operand, Operand::Reg(*reg)));
        }

        for TypedOperand { ty, operand } in stack_args.into_iter().rev() {
            if let AsmType::ByteArray { size, .. } = ty {
                instructions.push(Instruction::Binary(
                    AsmType::Quadword,
                    BinaryOp::Sub,
                    Operand::Imm(8),
                    Reg::SP.into(),
                ));
                Self::copy_bytes(instructions, operand, Operand::Memory(Reg::SP, 0), size);
            } else if matches!(operand, Operand::Imm(_) | Operand::Reg(_))
                || matches!(ty, AsmType::Quadword | AsmType::Double)
            {
                instructions.push(Instruction::Push(operand))
            } else {
                instructions.push(Instruction::Mov(ty, operand, Reg::Ax.into()));
                instructions.push(Instruction::Push(Reg::Ax.into()));
            }
        }

        instructions.push(Instruction::Call(name.clone()));

        if bytes_to_remove != 0 {
            instructions.push(Instruction::Binary(
                AsmType::Quadword,
                BinaryOp::Add,
                Operand::Imm(bytes_to_remove),
                Reg::SP.into(),
            ));
        }

        if dst.is_some() && !return_spec.in_memory {
            for (&reg, TypedOperand { ty, operand }) in
                INT_RETURN_REGISTERS.iter().zip(return_spec.int_values)
            {
                if let AsmType::ByteArray { size, .. } = ty {
                    Self::copy_bytes_from_reg(instructions, reg, operand, size as i64);
                } else {
                    instructions.push(Instruction::Mov(ty, reg.into(), operand));
                }
            }

            for (&reg, TypedOperand { ty, operand }) in
                SSE_RETURN_REGISTERS.iter().zip(return_spec.sse_values)
            {
                instructions.push(Instruction::Mov(ty, reg.into(), operand));
            }
        }
    }

    fn generate_return(&mut self, instructions: &mut Vec<Instruction>, val: &Option<tacky::Val>) {
        let Some(val) = val else {
            instructions.push(Instruction::Ret);
            return;
        };

        let return_spec = self.classify_return_value(val);

        if return_spec.in_memory {
            instructions.push(Instruction::Mov(
                AsmType::Quadword,
                Operand::Memory(Reg::BP, -8),
                Reg::Ax.into(),
            ));
            Self::copy_bytes(
                instructions,
                self.generate_val(val),
                Operand::Memory(Reg::Ax, 0),
                self.semantics.val_asm_ty(val).size(),
            );
        } else {
            for (&reg, TypedOperand { ty, operand }) in
                INT_RETURN_REGISTERS.iter().zip(return_spec.int_values)
            {
                if let AsmType::ByteArray { size, .. } = ty {
                    Self::copy_bytes_to_reg(instructions, operand, reg, size as i64);
                } else {
                    instructions.push(Instruction::Mov(ty, operand, reg.into()));
                }
            }

            for (&reg, TypedOperand { ty, operand }) in
                SSE_RETURN_REGISTERS.iter().zip(return_spec.sse_values)
            {
                instructions.push(Instruction::Mov(ty, operand, reg.into()));
            }
        }

        instructions.push(Instruction::Ret);
    }

    fn classify_parameters(&mut self, values: &[tacky::Val], return_in_memory: bool) -> FnArgs {
        let mut int_reg_args = Vec::new();
        let mut sse_reg_args = Vec::new();
        let mut stack_args = Vec::new();

        let int_regs_available = if return_in_memory {
            INT_ARG_REGISTERS.len() - 1
        } else {
            INT_ARG_REGISTERS.len()
        };
        let sse_regs_available = SSE_ARG_REGISTERS.len();

        for value in values {
            let operand = self.generate_val(value);
            let asm_ty = self.semantics.val_asm_ty(value);
            let param_ty = self.semantics.val_ty(value);
            let operand = TypedOperand {
                operand,
                ty: asm_ty,
            };
            if let AsmType::Double = asm_ty {
                if sse_reg_args.len() < SSE_ARG_REGISTERS.len() {
                    sse_reg_args.push(operand);
                } else {
                    stack_args.push(operand);
                }
            } else if asm_ty.is_scalar() {
                if int_reg_args.len() < int_regs_available {
                    int_reg_args.push(operand);
                } else {
                    stack_args.push(operand);
                }
            } else {
                let classes = self.classify_type(&param_ty);
                let aggregate = value.as_aggregate(&self.semantics);
                let mut use_stack = true;
                let type_size = aggregate.size;
                if !matches!(classes[0], ParamClass::Memory) {
                    let mut tentative_ints = Vec::new();
                    let mut tentative_doubles = Vec::new();
                    let mut offset = 0;
                    for class in &classes {
                        let operand = Operand::PseudoMem(value.as_var(), offset);
                        if let ParamClass::Sse = class {
                            tentative_doubles.push(TypedOperand {
                                operand,
                                ty: AsmType::Double,
                            });
                        } else {
                            let eightbyte_type = self.get_eightbyte_type(offset, type_size);
                            tentative_ints.push(TypedOperand {
                                operand,
                                ty: eightbyte_type,
                            });
                        }
                        offset += 8
                    }
                    if (tentative_doubles.len() + sse_reg_args.len()) <= sse_regs_available
                        && (tentative_ints.len() + int_reg_args.len()) <= int_regs_available
                    {
                        sse_reg_args.extend(tentative_doubles);
                        int_reg_args.extend(tentative_ints);
                        use_stack = false;
                    }
                }
                if use_stack {
                    let mut offset = 0;
                    for _class in &classes {
                        let operand = Operand::PseudoMem(value.as_var(), offset);
                        let eightbyte_type = self.get_eightbyte_type(offset, type_size);
                        stack_args.push(TypedOperand {
                            operand,
                            ty: eightbyte_type,
                        });
                        offset += 8
                    }
                }
            }
        }
        FnArgs {
            int_reg_args,
            sse_reg_args,
            stack_args,
        }
    }

    fn get_eightbyte_type(&self, offset: i64, struct_size: usize) -> AsmType {
        let bytes_from_end = struct_size - offset as usize;
        if bytes_from_end >= 8 {
            AsmType::Quadword
        } else if bytes_from_end == 4 {
            AsmType::Longword
        } else if bytes_from_end == 1 {
            AsmType::Byte
        } else {
            AsmType::ByteArray {
                size: bytes_from_end,
                alignment: 0,
            }
        }
    }

    fn classify_return_value(&mut self, return_value: &tacky::Val) -> FnReturn {
        let return_ty = self.semantics.val_ty(return_value);
        let asm_ty = self.semantics.val_asm_ty(return_value);
        if let AsmType::Double = asm_ty {
            let operand = self.generate_val(return_value);
            FnReturn {
                int_values: vec![],
                sse_values: vec![TypedOperand {
                    operand,
                    ty: asm_ty,
                }],
                in_memory: false,
            }
        } else if asm_ty.is_scalar() {
            let operand = self.generate_val(return_value);
            FnReturn {
                int_values: vec![TypedOperand {
                    operand,
                    ty: asm_ty,
                }],
                sse_values: vec![],
                in_memory: false,
            }
        } else {
            let classes = self.classify_type(&return_ty);
            let aggregate = return_value.as_aggregate(&self.semantics);
            let type_size = aggregate.size;
            if let Some(ParamClass::Memory) = classes.first() {
                FnReturn {
                    int_values: vec![],
                    sse_values: vec![],
                    in_memory: true,
                }
            } else {
                let mut offset = 0;
                let mut int_values = Vec::new();
                let mut sse_values = Vec::new();
                for class in &classes {
                    let operand = Operand::PseudoMem(return_value.as_var(), offset);
                    match class {
                        ParamClass::Sse => {
                            sse_values.push(TypedOperand {
                                operand,
                                ty: AsmType::Double,
                            });
                        }
                        ParamClass::Integer => {
                            let eightbyte_type = self.get_eightbyte_type(offset, type_size);
                            int_values.push(TypedOperand {
                                operand,
                                ty: eightbyte_type,
                            });
                        }
                        ParamClass::Memory => {
                            panic!("Cannot return a struct in memory");
                        }
                    }
                    offset += 8
                }
                FnReturn {
                    int_values,
                    sse_values,
                    in_memory: false,
                }
            }
        }
    }

    fn classify_type(&self, ty: &Type) -> Vec<ParamClass> {
        let size = ty.size(&self.semantics);
        if size > 16 {
            return (0..size).step_by(8).map(|_| ParamClass::Memory).collect();
        }

        let mut classes: Vec<ParamClass> = Vec::new();

        fn classify_inner(classes: &mut Vec<ParamClass>, ty: &Type, semantics: &SemanticData) {
            match ty {
                Type::Double => classes.push(ParamClass::Sse),

                Type::Char
                | Type::SChar
                | Type::UChar
                | Type::Int
                | Type::UInt
                | Type::Long
                | Type::ULong
                | Type::Pointer(_) => classes.push(ParamClass::Integer),

                Type::Struct(name) => {
                    let struct_def = semantics.get_aggregate(name);
                    let mut field_classes: Vec<ParamClass> = Vec::new();
                    for field in &struct_def.fields {
                        classify_inner(&mut field_classes, &field.ty, semantics);
                    }

                    if let Some(ParamClass::Sse) = field_classes.first() {
                        classes.push(ParamClass::Sse);
                    } else {
                        classes.push(ParamClass::Integer);
                    }

                    if struct_def.size > 8 {
                        if let Some(ParamClass::Sse) = field_classes.last() {
                            classes.push(ParamClass::Sse);
                        } else {
                            classes.push(ParamClass::Integer);
                        }
                    }
                }
                Type::Array(inner_ty, ..) => {
                    let size = ty.size(semantics);
                    if let Type::Double = &**inner_ty {
                        classes.push(ParamClass::Sse);
                        if size > 8 {
                            classes.push(ParamClass::Sse);
                        }
                    } else {
                        classes.push(ParamClass::Integer);
                        if size > 8 {
                            classes.push(ParamClass::Integer);
                        }
                    }
                }
                Type::Union(name) => {
                    let union_def = semantics.get_aggregate(name);
                    let mut field_classes = Vec::with_capacity(2);
                    let mut first_class: Option<ParamClass> = None;
                    let mut second_class: Option<ParamClass> = None;
                    for field in &union_def.fields {
                        classify_inner(&mut field_classes, &field.ty, semantics);
                        use ParamClass::*;
                        first_class = match (first_class, field_classes.first()) {
                            (None | Some(Sse), Some(Sse)) => Some(Sse),
                            (_, Some(Integer)) => Some(Integer),
                            (Some(Integer), _) => Some(Integer),
                            _ => unreachable!("Invalid class"),
                        };
                        second_class = match (second_class, field_classes.get(1)) {
                            (old, None) => old,
                            (None | Some(Sse), Some(Sse)) => Some(Sse),
                            (_, Some(Integer)) => Some(Integer),
                            (Some(Integer), _) => Some(Integer),
                            _ => unreachable!("Invalid class"),
                        };
                        field_classes.clear();
                    }
                    classes.push(first_class.unwrap());
                    if let Some(second_class) = second_class {
                        classes.push(second_class);
                    }
                }

                Type::Function(_) | Type::Void => {}
            }
        }
        classify_inner(&mut classes, ty, &self.semantics);
        classes
    }

    fn replace_pseudo_operands(
        &mut self,
        function: &mut Function,
        symbols: &BackendSymbolTable,
    ) -> usize {
        let mut stack_size: usize = if self.does_return_in_memory(&function.name) {
            8
        } else {
            0
        };
        let mut stack_vars = HashMap::new();

        let mut update_operand = |operand: &mut Operand| {
            if let Operand::Pseudo(name) | Operand::PseudoMem(name, ..) = operand {
                let Some(&BackendSymbolData::Obj {
                    ty,
                    is_static,
                    is_const,
                }) = symbols.get(name)
                else {
                    panic!("Operand '{name}' without symbol data")
                };
                let offset = if let Some(saved) = stack_vars.get(name).copied() {
                    saved
                } else if is_static {
                    let name = name.clone();
                    let offset = if let Operand::PseudoMem(_name, array_offset) = operand {
                        *array_offset
                    } else {
                        0
                    };
                    *operand = Operand::Data {
                        is_static: is_const,
                        name,
                        offset,
                    };
                    return;
                } else {
                    match ty {
                        AsmType::Byte => stack_size += 1,
                        AsmType::Longword => {
                            stack_size = align_offset(stack_size + 4, 8);
                        }
                        AsmType::Quadword | AsmType::Double => {
                            stack_size = align_offset(stack_size + 8, 8);
                        }
                        AsmType::ByteArray { size, alignment } => {
                            stack_size = align_offset(stack_size + size, alignment);
                        }
                    }
                    stack_vars.insert(name.clone(), stack_size);
                    stack_size
                };
                let offset = if let Operand::PseudoMem(_name, array_offset) = operand {
                    (offset as i64) - *array_offset
                } else {
                    offset as i64
                };
                *operand = Operand::Memory(Reg::BP, -offset);
            }
        };

        for instruction in &mut function.instructions {
            match instruction {
                Instruction::Mov(_, src, dst)
                | Instruction::Movsx(_, src, _, dst)
                | Instruction::MovZeroExtend(_, src, _, dst)
                | Instruction::Binary(_, _, src, dst)
                | Instruction::Cvttsd2si(_, src, dst)
                | Instruction::Cvtsi2sd(_, src, dst)
                | Instruction::Lea(src, dst)
                | Instruction::Cmp(_, src, dst) => {
                    update_operand(src);
                    update_operand(dst);
                }
                Instruction::Unary(_, _, src)
                | Instruction::Push(src)
                | Instruction::Idiv(_, src)
                | Instruction::Div(_, src)
                | Instruction::SetCC(_, src) => update_operand(src),

                Instruction::Cdq(_)
                | Instruction::Jmp(_)
                | Instruction::JmpCC(_, _)
                | Instruction::Label(_)
                | Instruction::Pop(_)
                | Instruction::Call(_)
                | Instruction::Ret => {}
            }
        }

        stack_size
    }

    fn fixup_instructions(&mut self, function: &mut Function, stack_size: usize) {
        let instructions = std::mem::take(&mut function.instructions);
        let mut fixed = Vec::with_capacity(instructions.len() + 1);

        // Trick to align to the next multiple of 16 or same value if it's already there:
        // https://math.stackexchange.com/a/291494
        let stack_size = ((stack_size as i64 - 1) | 15) + 1;
        fixed.push(Instruction::Binary(
            AsmType::Quadword,
            BinaryOp::Sub,
            Operand::Imm(stack_size),
            Reg::SP.into(),
        ));

        fn src_register(ty: AsmType) -> Reg {
            if let AsmType::Double = ty {
                Reg::XMM14
            } else {
                Reg::R10
            }
        }

        fn dst_register(ty: AsmType) -> Reg {
            if let AsmType::Double = ty {
                Reg::XMM15
            } else {
                Reg::R11
            }
        }

        for instruction in instructions.into_iter() {
            match instruction {
                Instruction::Mov(ty, src, dst) => {
                    let src = if let Operand::Imm(v) = src {
                        if i32::try_from(v).is_err() && !dst.is_reg() {
                            let value = match ty {
                                AsmType::Byte | AsmType::Longword => (v as i32) as i64,
                                AsmType::Quadword => v,
                                AsmType::Double => panic!("Immediate values, can't be double"),
                                AsmType::ByteArray { .. } => {
                                    panic!("Immediate values can't be byte array")
                                }
                            };
                            fixed.push(Instruction::Mov(ty, Operand::Imm(value), Reg::R10.into()));
                            Reg::R10.into()
                        } else {
                            src
                        }
                    } else if src.is_mem() && dst.is_mem() {
                        let reg = src_register(ty);
                        fixed.push(Instruction::Mov(ty, src, reg.into()));
                        reg.into()
                    } else {
                        src
                    };
                    fixed.push(Instruction::Mov(ty, src, dst));
                }
                Instruction::Movsx(src_ty, src, dst_ty, dst) => {
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
                        fixed.push(Instruction::Movsx(src_ty, src, dst_ty, Reg::R11.into()));
                        fixed.push(Instruction::Mov(dst_ty, Reg::R11.into(), dst));
                    } else {
                        fixed.push(Instruction::Movsx(src_ty, src, dst_ty, dst));
                    }
                }
                Instruction::Binary(ty, op, left, right)
                    if matches!(
                        op,
                        BinaryOp::Add
                            | BinaryOp::Sub
                            | BinaryOp::Mul
                            | BinaryOp::DivDouble
                            | BinaryOp::And
                            | BinaryOp::Or
                            | BinaryOp::Xor
                    ) =>
                {
                    let left = if let Operand::Imm(v) = left {
                        if i32::try_from(v).is_err() {
                            fixed.push(Instruction::Mov(ty, left, Reg::R10.into()));
                            Reg::R10.into()
                        } else {
                            left
                        }
                    } else if left.is_mem() && right.is_mem() {
                        let src_reg = src_register(ty);
                        fixed.push(Instruction::Mov(ty, left, src_reg.into()));
                        src_reg.into()
                    } else {
                        left
                    };
                    if matches!(op, BinaryOp::Mul)
                        || matches!(ty, AsmType::Double) && right.is_mem()
                    {
                        let dst_reg = dst_register(ty);
                        fixed.push(Instruction::Mov(ty, right.clone(), dst_reg.into()));
                        fixed.push(Instruction::Binary(ty, op, left, dst_reg.into()));
                        fixed.push(Instruction::Mov(ty, dst_reg.into(), right));
                    } else {
                        fixed.push(Instruction::Binary(ty, op, left, right));
                    }
                }
                Instruction::Cmp(AsmType::Double, left, right) => {
                    let right = if let Operand::Reg(_) = right {
                        right
                    } else {
                        let dst_reg = dst_register(AsmType::Double);
                        fixed.push(Instruction::Mov(AsmType::Double, right, dst_reg.into()));
                        dst_reg.into()
                    };
                    fixed.push(Instruction::Cmp(AsmType::Double, left, right));
                }
                Instruction::Cmp(ty, left, right) => {
                    let left = if let Operand::Imm(v) = left {
                        if i32::try_from(v).is_err() {
                            fixed.push(Instruction::Mov(ty, left, Reg::R10.into()));
                            Reg::R10.into()
                        } else {
                            left
                        }
                    } else if left.is_mem() && right.is_mem() {
                        let src_reg = src_register(ty);
                        fixed.push(Instruction::Mov(ty, left, src_reg.into()));
                        src_reg.into()
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
                Instruction::Idiv(ty, Operand::Imm(value)) => {
                    fixed.push(Instruction::Mov(ty, Operand::Imm(value), Reg::R10.into()));
                    fixed.push(Instruction::Idiv(ty, Reg::R10.into()));
                }
                Instruction::Div(ty, Operand::Imm(value)) => {
                    fixed.push(Instruction::Mov(ty, Operand::Imm(value), Reg::R10.into()));
                    fixed.push(Instruction::Div(ty, Reg::R10.into()));
                }
                Instruction::Push(Operand::Imm(value)) => {
                    let value = if i32::try_from(value).is_err() {
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

                Instruction::MovZeroExtend(src_ty, src, dst_ty, dst)
                    if matches!(src_ty, AsmType::Byte) =>
                {
                    // TODO: generalize this pattern that is repeated all over the fixup phase.
                    let src = if let Operand::Imm(v) = src
                        && i32::try_from(v).is_err()
                    {
                        fixed.push(Instruction::Mov(src_ty, src, Reg::R10.into()));
                        Reg::R10.into()
                    } else {
                        src
                    };

                    if let Operand::Reg(_) = dst {
                        fixed.push(Instruction::MovZeroExtend(src_ty, src, dst_ty, dst))
                    } else {
                        fixed.push(Instruction::MovZeroExtend(
                            src_ty,
                            src,
                            dst_ty,
                            Reg::R11.into(),
                        ));
                        fixed.push(Instruction::Mov(dst_ty, Reg::R11.into(), dst));
                    };
                }
                Instruction::MovZeroExtend(_src_ty, src, _dst_ty, dst) => {
                    if let Operand::Reg(_) = dst {
                        fixed.push(Instruction::Mov(AsmType::Longword, src, dst));
                    } else {
                        fixed.push(Instruction::Mov(AsmType::Longword, src, Reg::R11.into()));
                        fixed.push(Instruction::Mov(AsmType::Quadword, Reg::R11.into(), dst));
                    };
                }
                Instruction::Cvttsd2si(ty, src, Operand::Memory(mem_reg, offset)) => {
                    fixed.push(Instruction::Cvttsd2si(ty, src, Reg::R11.into()));
                    fixed.push(Instruction::Mov(
                        ty,
                        Reg::R11.into(),
                        Operand::Memory(mem_reg, offset),
                    ));
                }
                Instruction::Lea(src, Operand::Memory(mem_reg, offset)) => {
                    fixed.push(Instruction::Lea(src, Reg::R11.into()));
                    fixed.push(Instruction::Mov(
                        AsmType::Quadword,
                        Reg::R11.into(),
                        Operand::Memory(mem_reg, offset),
                    ));
                }
                Instruction::Cvtsi2sd(ty, src, dst) => {
                    let src = if let Operand::Imm(_) = src {
                        fixed.push(Instruction::Mov(ty, src, src_register(ty).into()));
                        src_register(ty).into()
                    } else {
                        src
                    };
                    if dst.is_mem() {
                        let dst_reg = dst_register(AsmType::Double);
                        fixed.push(Instruction::Cvtsi2sd(ty, src, dst_reg.into()));
                        fixed.push(Instruction::Mov(AsmType::Double, dst_reg.into(), dst));
                    } else {
                        fixed.push(Instruction::Cvtsi2sd(ty, src, dst));
                    }
                }
                other => fixed.push(other),
            }
        }
        function.instructions = fixed
    }

    fn generate_val(&mut self, val: &tacky::Val) -> Operand {
        match val {
            tacky::Val::Constant(value) => {
                if let Constant::Double(double) = value {
                    self.make_double_constant(*double)
                } else {
                    Operand::Imm(value.as_u64() as i64)
                }
            }
            tacky::Val::Var(name) => match self.semantics.val_asm_ty(val) {
                AsmType::ByteArray { .. } => Operand::PseudoMem(name.clone(), 0),
                _ => Operand::Pseudo(name.clone()),
            },
        }
    }

    fn make_double_constant(&mut self, value: f64) -> Operand {
        let key = value.to_bits();
        let existing_constant = self.doubles.get(&key);
        let name = match existing_constant {
            Some(name) => name.clone(),
            None => {
                let name = format!("_double_{}", self.doubles.len());
                self.doubles.insert(key, name.clone());
                name
            }
        };
        Operand::Data {
            is_static: true,
            name,
            offset: 0,
        }
    }

    fn make_label(&mut self, prefix: &str) -> Symbol {
        let label = format!("{prefix}_{}", self.label_counter);
        self.label_counter += 1;
        label
    }
}

impl tacky::UnaryOp {
    fn to_asm(&self) -> UnaryOp {
        match self {
            tacky::UnaryOp::Complement => UnaryOp::Not,
            tacky::UnaryOp::Negate => UnaryOp::Neg,
            tacky::UnaryOp::Increment | tacky::UnaryOp::Decrement | tacky::UnaryOp::Not => {
                unreachable!()
            }
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
            tacky::BinaryOp::Divide => BinaryOp::DivDouble,

            tacky::BinaryOp::Reminder
            | tacky::BinaryOp::ShiftLeft
            | tacky::BinaryOp::ShiftRight
            | tacky::BinaryOp::Equal
            | tacky::BinaryOp::NotEqual
            | tacky::BinaryOp::LessThan
            | tacky::BinaryOp::LessOrEqual
            | tacky::BinaryOp::GreaterThan
            | tacky::BinaryOp::GreaterOrEqual => unreachable!(),
        }
    }
}

impl AsmType {
    fn size(&self) -> usize {
        match self {
            AsmType::Byte => 1,
            AsmType::Longword => 4,
            AsmType::Quadword | AsmType::Double => 8,
            AsmType::ByteArray { size, .. } => *size,
        }
    }

    fn alignment(&self) -> u8 {
        match self {
            AsmType::Byte => 1,
            AsmType::Longword => 4,
            AsmType::Quadword | AsmType::Double => 8,
            AsmType::ByteArray { alignment, .. } => *alignment,
        }
    }

    fn is_scalar(&self) -> bool {
        match self {
            AsmType::Byte | AsmType::Longword | AsmType::Quadword | AsmType::Double => true,
            AsmType::ByteArray { .. } => false,
        }
    }
}

impl tacky::Val {
    fn as_var(&self) -> Symbol {
        match self {
            tacky::Val::Constant(_) => panic!("Constant value can't be a variable"),
            tacky::Val::Var(symbol) => symbol.clone(),
        }
    }

    fn as_aggregate<'a>(&self, semantics: &'a SemanticData) -> &'a AggregateType {
        let tacky::Val::Var(value_name) = self else {
            panic!("Non-scalar value that is not a struct");
        };
        let (Type::Struct(type_name) | Type::Union(type_name)) = semantics.symbol_ty(value_name)
        else {
            panic!("Non-scalar value that is not a struct");
        };
        semantics.get_aggregate(type_name)
    }
}

impl Operand {
    fn is_reg(&self) -> bool {
        matches!(self, Operand::Reg(_))
    }
    fn is_mem(&self) -> bool {
        matches!(
            self,
            Operand::Memory(..) | Operand::Data { .. } | Operand::Indexed(..)
        )
    }

    fn add_offset(&self, bytes: i64) -> Operand {
        match self {
            Operand::Memory(reg, offset) => Operand::Memory(*reg, *offset + bytes),
            Operand::PseudoMem(name, offset) => Operand::PseudoMem(name.clone(), *offset + bytes),
            _ => panic!("Can't add offset to non-memory operand"),
        }
    }
}

pub fn generate(program: &tacky::Program) -> Program {
    let mut compiler = Compiler {
        doubles: HashMap::new(),
        label_counter: 0,
        semantics: program.semantics.clone(),
    };
    compiler.generate(program)
}
