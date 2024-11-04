pub mod ir;

use crate::asm::ir::{
    AsmType, BinaryOp, CondCode, Function, Instruction, Operand, Program, Reg, StaticConstant,
    StaticVariable, TopLevel, UnaryOp,
};
use crate::ast::{Constant, Type};
use crate::semantic::{Attributes, SemanticData, StaticInit};
use crate::symbol::Symbol;
use crate::tacky;
use std::collections::HashMap;

const INT_ARG_REGISTERS: [Reg; 6] = [Reg::Di, Reg::Si, Reg::Dx, Reg::Cx, Reg::R8, Reg::R9];
const DOUBLE_ARG_REGISTERS: [Reg; 8] = [
    Reg::XMM0,
    Reg::XMM1,
    Reg::XMM2,
    Reg::XMM3,
    Reg::XMM4,
    Reg::XMM5,
    Reg::XMM6,
    Reg::XMM7,
];

enum BackendSymbolData {
    Obj {
        ty: AsmType,
        is_static: bool,
        is_const: bool,
    },
    Fn {
        _defined: bool,
    },
}

type BackendSymbolTable = HashMap<Symbol, BackendSymbolData>;

impl SemanticData {
    fn val_asm_ty(&self, val: &tacky::Val) -> AsmType {
        match val {
            tacky::Val::Constant(Constant::Int(_) | Constant::UInt(_)) => AsmType::Longword,
            tacky::Val::Constant(Constant::Long(_) | Constant::ULong(_)) => AsmType::Quadword,
            tacky::Val::Constant(Constant::Double(_)) => AsmType::Double,
            tacky::Val::Var(name) => self.symbol_asm_ty(name),
        }
    }

    fn symbol_asm_ty(&self, symbol: &Symbol) -> AsmType {
        match self.symbol_ty(symbol) {
            Type::Int | Type::UInt => AsmType::Longword,
            Type::Long | Type::ULong => AsmType::Quadword,
            Type::Double => AsmType::Double,
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
            tacky::Val::Constant(Constant::Double(_)) => true,
            tacky::Val::Var(name) => match self.symbol_ty(name) {
                Type::Int | Type::Long => true,
                Type::UInt | Type::ULong => false,
                Type::Double => true,
                Type::Function(_) => unreachable!(),
            },
        }
    }
}

struct TypedOperand {
    operand: Operand,
    ty: AsmType,
}

struct FnArgs {
    int_reg_args: Vec<TypedOperand>,
    double_reg_args: Vec<TypedOperand>,
    stack_args: Vec<TypedOperand>,
}

// TODO: Maybe this is not necessary?
struct Compiler {
    doubles: HashMap<u64, Symbol>,
    label_counter: u64,
}

impl Compiler {
    fn generate(&mut self, program: &tacky::Program) -> Program {
        let mut top_level = Vec::new();
        for element in &program.top_level {
            match element {
                tacky::TopLevel::Function(f) => top_level.push(TopLevel::Function(
                    self.generate_function(f, &program.semantics),
                )),
                tacky::TopLevel::Variable(v) => top_level.push(TopLevel::Variable(
                    self.generate_static_variable(v, &program.semantics),
                )),
            }
        }

        let mut backend_symbols = Self::make_backend_symbols(&program.semantics);

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
                let stack_size =
                    self.replace_pseudo_registers(&mut function.instructions, &backend_symbols);
                self.fixup_instructions(function, stack_size);
            }
        }

        Program { top_level }
    }

    fn generate_function(
        &mut self,
        function: &tacky::Function,
        semantic: &SemanticData,
    ) -> Function {
        let mut instructions = Vec::new();
        let args: Vec<tacky::Val> = function
            .params
            .iter()
            .cloned()
            .map(tacky::Val::Var)
            .collect();
        let FnArgs {
            int_reg_args,
            double_reg_args,
            stack_args,
        } = self.classify_parameters(&args, semantic);

        for (reg, TypedOperand { ty, operand }) in INT_ARG_REGISTERS.iter().zip(int_reg_args) {
            instructions.push(Instruction::Mov(ty, Operand::Reg(*reg), operand));
        }

        for (reg, TypedOperand { ty, operand }) in DOUBLE_ARG_REGISTERS.iter().zip(double_reg_args)
        {
            instructions.push(Instruction::Mov(ty, Operand::Reg(*reg), operand));
        }

        let mut offset = 16;
        for TypedOperand { ty, operand } in stack_args {
            instructions.push(Instruction::Mov(ty, Operand::Stack(offset), operand));
            offset += 8;
        }

        for tacky_instruction in &function.body {
            match tacky_instruction {
                tacky::Instruction::Return(val) => {
                    let reg = match semantic.val_asm_ty(val) {
                        AsmType::Double => Reg::XMM0,
                        _ => Reg::Ax
                    };
                    instructions.push(Instruction::Mov(
                        semantic.val_asm_ty(val),
                        self.generate_val(val),
                        reg.into(),
                    ));
                    instructions.push(Instruction::Ret);
                }

                tacky::Instruction::Unary { op, src, dst } => {
                    let ty = semantic.val_asm_ty(src);
                    let is_double = matches!(ty, AsmType::Double);
                    match op {
                        // tacky::UnaryOp::Increment if is_double => {
                        //     let one = self.make_double_constant(1.0);
                        //     instructions.push(Instruction::Mov(
                        //         semantic.val_asm_ty(src),
                        //         self.generate_val(src),
                        //         Reg::XMM0.into(),
                        //     ));
                        //     instructions.push(Instruction::Binary(
                        //         AsmType::Double,
                        //         BinaryOp::Add,
                        //         one,
                        //         self.generate_val(src),
                        //     ));
                        // }
                        // tacky::UnaryOp::Decrement if is_double => {
                        //     let one = self.make_double_constant(1.0);
                        //     instructions.push(Instruction::Binary(
                        //         AsmType::Double,
                        //         BinaryOp::Sub,
                        //         one,
                        //         self.generate_val(dst),
                        //     ));
                        // }
                        tacky::UnaryOp::Not if is_double => {
                            instructions.push(Instruction::Binary(
                                AsmType::Double,
                                BinaryOp::Xor,
                                Reg::XMM0.into(),
                                Reg::XMM0.into(),
                            ));
                            instructions.push(Instruction::Cmp(
                                ty,
                                Reg::XMM0.into(),
                                self.generate_val(src),
                            ));
                            instructions.push(Instruction::Mov(
                                semantic.val_asm_ty(dst),
                                Reg::XMM0.into(),
                                self.generate_val(dst),
                            ));
                            instructions
                                .push(Instruction::SetCC(CondCode::E, self.generate_val(dst)));
                        }
                        tacky::UnaryOp::Not => {
                            instructions.push(Instruction::Cmp(
                                ty,
                                Operand::Imm(0),
                                self.generate_val(src),
                            ));
                            instructions.push(Instruction::Mov(
                                semantic.val_asm_ty(dst),
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

                        tacky::UnaryOp::Complement
                        | tacky::UnaryOp::Negate
                        | tacky::UnaryOp::Increment
                        | tacky::UnaryOp::Decrement => {
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
                    let ty = semantic.val_asm_ty(src1);
                    let is_double = matches!(ty, AsmType::Double);
                    match op {
                        tacky::BinaryOp::Divide if !is_double => {
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src1),
                                Reg::Ax.into(),
                            ));
                            if semantic.is_signed(src1) {
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
                            if semantic.is_signed(src1) {
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
                        tacky::BinaryOp::ShiftLeft => {
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src1),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src2),
                                Reg::Cx.into(),
                            ));
                            if semantic.is_signed(src1) {
                                instructions.push(Instruction::Sal(ty, self.generate_val(dst)));
                            } else {
                                instructions.push(Instruction::Shl(ty, self.generate_val(dst)));
                            }
                        }
                        tacky::BinaryOp::ShiftRight => {
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src1),
                                self.generate_val(dst),
                            ));
                            instructions.push(Instruction::Mov(
                                ty,
                                self.generate_val(src2),
                                Reg::Cx.into(),
                            ));
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
                                ty,
                                self.generate_val(src2),
                                self.generate_val(src1),
                            ));
                            instructions.push(Instruction::Mov(
                                semantic.val_asm_ty(dst),
                                Operand::Imm(0),
                                self.generate_val(dst),
                            ));

                            let unsigned_or_double = !semantic.is_signed(src1) || matches!(ty, AsmType::Double);
                            let cond = match (op, unsigned_or_double) {
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
                            instructions.push(Instruction::SetCC(cond, self.generate_val(dst)));
                        }

                        tacky::BinaryOp::Add
                        | tacky::BinaryOp::Subtract
                        | tacky::BinaryOp::Multiply
                        | tacky::BinaryOp::Divide // Covers only if double
                        | tacky::BinaryOp::BinAnd
                        | tacky::BinaryOp::BinOr
                        | tacky::BinaryOp::BinXor => {
                            let ty = semantic.val_asm_ty(src1);
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
                    if let AsmType::Double = semantic.val_asm_ty(cond) {
                        instructions.push(Instruction::Binary(
                            AsmType::Double,
                            BinaryOp::Xor,
                            Reg::XMM0.into(),
                            Reg::XMM0.into(),
                        ));
                        instructions.push(Instruction::Cmp(
                            semantic.val_asm_ty(cond),
                            Reg::XMM0.into(),
                            self.generate_val(cond),
                        ));
                    } else {
                        instructions.push(Instruction::Cmp(
                            semantic.val_asm_ty(cond),
                            Operand::Imm(0),
                            self.generate_val(cond),
                        ));
                    }
                    instructions.push(Instruction::JmpCC(CondCode::E, target.clone()));
                }
                tacky::Instruction::JumpIfNotZero { cond, target } => {
                    if let AsmType::Double = semantic.val_asm_ty(cond) {
                        instructions.push(Instruction::Binary(
                            AsmType::Double,
                            BinaryOp::Xor,
                            Reg::XMM0.into(),
                            Reg::XMM0.into(),
                        ));
                        instructions.push(Instruction::Cmp(
                            semantic.val_asm_ty(cond),
                            Reg::XMM0.into(),
                            self.generate_val(cond),
                        ));
                    } else {
                        instructions.push(Instruction::Cmp(
                            semantic.val_asm_ty(cond),
                            Operand::Imm(0),
                            self.generate_val(cond),
                        ));
                    }
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
                    instructions.push(Instruction::Movsx(
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::Truncate { src, dst } => {
                    instructions.push(Instruction::Mov(
                        AsmType::Longword,
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::ZeroExtend { src, dst } => {
                    instructions.push(Instruction::MovZeroExtend(
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }

                tacky::Instruction::DoubleToInt { src, dst } => {
                    instructions.push(Instruction::Cvttsd2si(
                        semantic.val_asm_ty(dst),
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::IntToDouble { src, dst } => {
                    instructions.push(Instruction::Cvtsi2sd(
                        semantic.val_asm_ty(src),
                        self.generate_val(src),
                        self.generate_val(dst),
                    ));
                }
                tacky::Instruction::DoubleToUInt { src, dst } => match semantic.val_asm_ty(dst) {
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
                            Operand::Imm(upper_bound_u64),
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
                    AsmType::Double => unreachable!(),
                },
                tacky::Instruction::UIntToDouble { src, dst } => match semantic.val_asm_ty(src) {
                    AsmType::Longword => {
                        instructions.push(Instruction::MovZeroExtend(
                            self.generate_val(src),
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
                        instructions.push(Instruction::Unary(
                            AsmType::Quadword,
                            UnaryOp::Shr,
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
                    AsmType::Double => unreachable!(),
                },
            }
        }

        Function {
            name: function.name.clone(),
            global: function.global,
            instructions,
        }
    }

    fn make_backend_symbols(semantic: &SemanticData) -> BackendSymbolTable {
        let mut backend_symbols = HashMap::new();
        for (symbol, data) in semantic.symbols.iter() {
            match data.attrs {
                Attributes::Function { defined, .. } => {
                    backend_symbols
                        .insert(symbol.clone(), BackendSymbolData::Fn { _defined: defined });
                }
                Attributes::Static { .. } => {
                    backend_symbols.insert(
                        symbol.clone(),
                        BackendSymbolData::Obj {
                            ty: semantic.symbol_asm_ty(symbol),
                            is_static: true,
                            is_const: false,
                        },
                    );
                }
                Attributes::Local => {
                    backend_symbols.insert(
                        symbol.clone(),
                        BackendSymbolData::Obj {
                            ty: semantic.symbol_asm_ty(symbol),
                            is_static: false,
                            is_const: false,
                        },
                    );
                }
            }
        }
        backend_symbols
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
        let FnArgs {
            int_reg_args,
            double_reg_args,
            stack_args,
        } = self.classify_parameters(args, semantic);

        let padding = if stack_args.len() % 2 == 0 { 0 } else { 8 };
        if padding != 0 {
            instructions.push(Instruction::Binary(
                AsmType::Quadword,
                BinaryOp::Sub,
                Operand::Imm(padding),
                Reg::SP.into(),
            ))
        }
        let bytes_to_remove = 8 * stack_args.len() as u64 + padding;

        for (reg, TypedOperand { ty, operand }) in INT_ARG_REGISTERS.iter().zip(int_reg_args) {
            instructions.push(Instruction::Mov(ty, operand, Operand::Reg(*reg)));
        }

        for (reg, TypedOperand { ty, operand }) in DOUBLE_ARG_REGISTERS.iter().zip(double_reg_args)
        {
            instructions.push(Instruction::Mov(ty, operand, Operand::Reg(*reg)));
        }

        for TypedOperand { ty, operand } in stack_args.into_iter().rev() {
            if matches!(operand, Operand::Imm(_) | Operand::Reg(_))
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

        let return_reg = match semantic.val_asm_ty(dst) {
            AsmType::Longword | AsmType::Quadword => Reg::Ax,
            AsmType::Double => Reg::XMM0,
        };

        instructions.push(Instruction::Mov(
            semantic.val_asm_ty(dst),
            return_reg.into(),
            self.generate_val(dst),
        ));
    }

    fn classify_parameters(&mut self, values: &[tacky::Val], semantic: &SemanticData) -> FnArgs {
        let mut int_reg_args = Vec::new();
        let mut double_reg_args = Vec::new();
        let mut stack_args = Vec::new();

        for value in values {
            let operand = self.generate_val(value);
            let ty = semantic.val_asm_ty(value);
            let operand = TypedOperand { operand, ty };
            if let AsmType::Double = ty {
                if double_reg_args.len() < DOUBLE_ARG_REGISTERS.len() {
                    double_reg_args.push(operand);
                } else {
                    stack_args.push(operand);
                }
            } else if int_reg_args.len() < INT_ARG_REGISTERS.len() {
                int_reg_args.push(operand);
            } else {
                stack_args.push(operand);
            }
        }
        FnArgs {
            int_reg_args,
            double_reg_args,
            stack_args,
        }
    }

    fn replace_pseudo_registers(
        &mut self,
        instructions: &mut Vec<Instruction>,
        symbols: &BackendSymbolTable,
    ) -> u64 {
        let mut stack_size = 0u64;
        let mut stack_vars = HashMap::new();

        let mut update_operand = |operand: &mut Operand| {
            if let Operand::Pseudo(name) = operand {
                let Some(&BackendSymbolData::Obj {
                    ty,
                    is_static,
                    is_const,
                }) = symbols.get(name)
                else {
                    panic!("Operand without symbol data")
                };
                let offset = if let Some(saved) = stack_vars.get(name).copied() {
                    saved
                } else if is_static {
                    *operand = Operand::Data(is_const, name.clone());
                    return;
                } else {
                    match ty {
                        AsmType::Longword => {
                            stack_size += 4;
                        }
                        AsmType::Quadword | AsmType::Double => {
                            stack_size += 8 + stack_size % 8;
                        }
                    }
                    stack_vars.insert(name.clone(), stack_size);
                    stack_size
                };
                *operand = Operand::Stack(-(offset as i64));
            }
        };

        for instruction in instructions {
            match instruction {
                Instruction::Mov(_, src, dst)
                | Instruction::Movsx(src, dst)
                | Instruction::MovZeroExtend(src, dst)
                | Instruction::Binary(_, _, src, dst)
                | Instruction::Cvttsd2si(_, src, dst)
                | Instruction::Cvtsi2sd(_, src, dst)
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

                Instruction::Cdq(_)
                | Instruction::Jmp(_)
                | Instruction::JmpCC(_, _)
                | Instruction::Label(_)
                | Instruction::Call(_)
                | Instruction::Ret => {}
            }
        }

        stack_size
    }

    fn fixup_instructions(&mut self, function: &mut Function, stack_size: u64) {
        let instructions = std::mem::take(&mut function.instructions);
        let mut fixed = Vec::with_capacity(instructions.len() + 1);

        // Trick to align to the next multiple of 16 or same value if it's already there:
        // https://math.stackexchange.com/a/291494
        let stack_size = ((stack_size as i64 - 1) | 15) + 1;
        fixed.push(Instruction::Binary(
            AsmType::Quadword,
            BinaryOp::Sub,
            Operand::Imm(stack_size as u64),
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
                        if v > i32::MAX as u64 {
                            let value = match ty {
                                AsmType::Longword => (v as i32) as u64,
                                AsmType::Quadword => v,
                                AsmType::Double => panic!("Immediate values, can't be double"),
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
                            | BinaryOp::DivDouble
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
                        fixed.push(Instruction::Mov(
                            AsmType::Double,
                            right,
                            dst_reg.into(),
                        ));
                        dst_reg.into()
                    };
                    fixed.push(Instruction::Cmp(
                        AsmType::Double,
                        left,
                        right,
                    ));
                }
                Instruction::Cmp(ty, left, right) => {
                    let left = if let Operand::Imm(v) = left {
                        if v > i32::MAX as u64 {
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
                Instruction::Idiv(ty, Operand::Imm(value))
                | Instruction::Div(ty, Operand::Imm(value)) => {
                    fixed.push(Instruction::Mov(ty, Operand::Imm(value), Reg::R10.into()));
                    fixed.push(Instruction::Idiv(ty, Reg::R10.into()));
                }
                Instruction::Push(Operand::Imm(value)) => {
                    let value = if value > i32::MAX as u64 {
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
                Instruction::MovZeroExtend(src, Operand::Reg(reg)) => {
                    fixed.push(Instruction::Mov(AsmType::Longword, src, Operand::Reg(reg)));
                }
                Instruction::MovZeroExtend(src, dst) => {
                    fixed.push(Instruction::Mov(AsmType::Longword, src, Reg::R11.into()));
                    fixed.push(Instruction::Mov(AsmType::Quadword, Reg::R11.into(), dst));
                }
                Instruction::Cvttsd2si(ty, src, Operand::Stack(offset)) => {
                    fixed.push(Instruction::Cvttsd2si(ty, src, Reg::R11.into()));
                    fixed.push(Instruction::Mov(
                        ty,
                        Reg::R11.into(),
                        Operand::Stack(offset),
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
                    Operand::Imm(value.as_u64())
                }
            }
            tacky::Val::Var(name) => Operand::Pseudo(name.clone()),
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
        Operand::Data(true, name)
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

pub fn generate(program: &tacky::Program) -> Program {
    let mut compiler = Compiler {
        doubles: HashMap::new(),
        label_counter: 0,
    };
    compiler.generate(program)
}
