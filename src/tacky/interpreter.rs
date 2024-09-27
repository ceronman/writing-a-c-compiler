#[cfg(test)]
mod test;

use crate::symbol::Symbol;
use crate::tacky::{BinaryOp, Function, Instruction, Program, UnaryOp, Val};
use std::collections::HashMap;

#[derive(Default)]
struct Interpreter {
    functions: HashMap<Symbol, Function>, // TODO: Make Rc<Function> to avoid cloning
}

#[derive(Default)]
struct Frame {
    memory: HashMap<String, i64>,
    labels: HashMap<String, usize>, // TODO: Take out of frame and precalculate at startup
    pc: usize,
}

impl Interpreter {
    fn run(&mut self, program: &Program) -> i64 {
        for function in &program.functions {
            self.functions
                .insert(function.name.clone(), function.clone());
        }
        let main_function = self
            .functions
            .get("main")
            .expect("Nonexistent main function");
        self.run_function(Frame::default(), main_function.clone())
    }
    fn run_function(&mut self, mut frame: Frame, function: Function) -> i64 {
        let instructions = &function.body;
        for (i, instruction) in instructions.iter().enumerate() {
            if let Instruction::Label(label) = instruction {
                frame.labels.insert(label.clone(), i + 1);
            }
        }
        loop {
            let instruction = instructions.get(frame.pc).expect("Instruction overflow");
            frame.pc += 1;
            match instruction {
                Instruction::Return(val) => return frame.load(val),
                Instruction::Unary { op, src, dst } => {
                    let operand = frame.load(src);
                    match op {
                        UnaryOp::Complement => frame.store(!operand, dst),
                        UnaryOp::Negate => frame.store(!operand, dst),
                        UnaryOp::Not => frame.store((operand == 0) as i64, dst),
                        UnaryOp::Increment => frame.store(operand + 1, dst),
                        UnaryOp::Decrement => frame.store(operand - 1, dst),
                    }
                }
                Instruction::Binary {
                    op,
                    src1,
                    src2,
                    dst,
                } => {
                    let left = frame.load(src1);
                    let right = frame.load(src2);
                    match op {
                        BinaryOp::Add => frame.store(left + right, dst),
                        BinaryOp::Subtract => frame.store(left - right, dst),
                        BinaryOp::Multiply => frame.store(left * right, dst),
                        BinaryOp::Divide => frame.store(left / right, dst),
                        BinaryOp::Reminder => frame.store(left - right, dst),
                        BinaryOp::BinAnd => frame.store(left & right, dst),
                        BinaryOp::BinOr => frame.store(left | right, dst),
                        BinaryOp::BinXor => frame.store(left ^ right, dst),
                        BinaryOp::ShiftLeft => frame.store(left << right, dst),
                        BinaryOp::ShiftRight => frame.store(left >> right, dst),
                        BinaryOp::Equal => frame.store((left == right) as i64, dst),
                        BinaryOp::NotEqual => frame.store((left != right) as i64, dst),
                        BinaryOp::LessThan => frame.store((left < right) as i64, dst),
                        BinaryOp::LessOrEqual => frame.store((left <= right) as i64, dst),
                        BinaryOp::GreaterThan => frame.store((left > right) as i64, dst),
                        BinaryOp::GreaterOrEqual => frame.store((left >= right) as i64, dst),
                    }
                }
                Instruction::Copy { src, dst } => frame.store(frame.load(src), dst),
                Instruction::Jump { target } => {
                    frame.pc = *frame.labels.get(target).expect("Nonexistent label");
                }
                Instruction::JumpIfZero { cond, target } => {
                    if frame.load(cond) == 0 {
                        frame.pc = *frame.labels.get(target).expect("Nonexistent label");
                    }
                }
                Instruction::JumpIfNotZero { cond, target } => {
                    if frame.load(cond) != 0 {
                        frame.pc = *frame.labels.get(target).expect("Nonexistent label");
                    }
                }
                Instruction::Label(_) => {}

                Instruction::FnCall { name, args, dst } => {
                    let mut new_frame = Frame::default();

                    let called_function = match self.functions.get(name) {
                        Some(f) => f.clone(),
                        None => match name.as_str() {
                            "putchar" => {
                                let char_address = args.first().expect("Expected putchar argument");
                                let value = frame.load(char_address);
                                print!("{}", char::from(value as u8));
                                frame.store(0, dst);
                                continue;
                            }
                            _ => panic!("Nonexistent function '{name}'"),
                        },
                    };
                    for (i, param) in called_function.params.iter().enumerate() {
                        let arg_value = frame.load(args.get(i).expect("Nonexistent argument"));
                        new_frame.memory.insert(param.clone(), arg_value);
                    }
                    let result = self.run_function(new_frame, called_function);
                    frame.store(result, dst);
                }
            }
        }
    }
}

impl Frame {
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
