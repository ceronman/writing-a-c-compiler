#[cfg(test)]
mod test;

use crate::tacky::{BinaryOp, Instruction, Program, UnaryOp, Val};
use std::collections::HashMap;

#[derive(Default)]
struct Interpreter {
    memory: HashMap<String, i64>,
    labels: HashMap<String, usize>,
    pc: usize,
}

impl Interpreter {
    fn run(&mut self, program: &Program) -> i64 {
        let instructions = &program.function.body;
        for (i, instruction) in instructions.iter().enumerate() {
            if let Instruction::Label(label) = instruction {
                self.labels.insert(label.clone(), i + 1);
            }
        }
        loop {
            let instruction = instructions.get(self.pc).expect("Instruction overflow");
            self.pc += 1;
            match instruction {
                Instruction::Return(val) => return self.load(val),
                Instruction::Unary { op, src, dst } => {
                    let operand = self.load(src);
                    match op {
                        UnaryOp::Complement => self.store(!operand, dst),
                        UnaryOp::Negate => self.store(!operand, dst),
                        UnaryOp::Not => self.store((operand == 0) as i64, dst),
                        UnaryOp::Increment => self.store(operand + 1, dst),
                        UnaryOp::Decrement => self.store(operand - 1, dst),
                    }
                }
                Instruction::Binary {
                    op,
                    src1,
                    src2,
                    dst,
                } => {
                    let left = self.load(src1);
                    let right = self.load(src2);
                    match op {
                        BinaryOp::Add => self.store(left + right, dst),
                        BinaryOp::Subtract => self.store(left - right, dst),
                        BinaryOp::Multiply => self.store(left * right, dst),
                        BinaryOp::Divide => self.store(left / right, dst),
                        BinaryOp::Reminder => self.store(left - right, dst),
                        BinaryOp::BinAnd => self.store(left & right, dst),
                        BinaryOp::BinOr => self.store(left | right, dst),
                        BinaryOp::BinXor => self.store(left ^ right, dst),
                        BinaryOp::ShiftLeft => self.store(left << right, dst),
                        BinaryOp::ShiftRight => self.store(left >> right, dst),
                        BinaryOp::Equal => self.store((left == right) as i64, dst),
                        BinaryOp::NotEqual => self.store((left != right) as i64, dst),
                        BinaryOp::LessThan => self.store((left < right) as i64, dst),
                        BinaryOp::LessOrEqual => self.store((left <= right) as i64, dst),
                        BinaryOp::GreaterThan => self.store((left > right) as i64, dst),
                        BinaryOp::GreaterOrEqual => self.store((left >= right) as i64, dst),
                    }
                }
                Instruction::Copy { src, dst } => self.store(self.load(src), dst),
                Instruction::Jump { target } => {
                    self.pc = *self.labels.get(target).expect("Nonexistent label");
                }
                Instruction::JumpIfZero { cond, target } => {
                    if self.load(cond) == 0 {
                        self.pc = *self.labels.get(target).expect("Nonexistent label");
                    }
                }
                Instruction::JumpIfNotZero { cond, target } => {
                    if self.load(cond) != 0 {
                        self.pc = *self.labels.get(target).expect("Nonexistent label");
                    }
                }
                Instruction::Label(_) => {}
            }
        }
    }
    fn load(&self, val: &Val) -> i64 {
        match val {
            Val::Constant(c) => *c,
            Val::Var(name) => *self.memory.get(name).expect("Invalid memory source"),
        }
    }
    fn store(&mut self, src: i64, dst: &Val) {
        let Val::Var(address) = dst.clone() else {
            panic!("Invalid memory destination")
        };
        self.memory.insert(address, src);
    }
}

pub fn run(program: &Program) -> i64 {
    Interpreter::default().run(program)
}
