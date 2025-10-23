use crate::optimization::cfg::{GenericCfg, GenericInstruction, GenericNode, InstructionKind};
use crate::tacky::Instruction;
use crate::tacky::pretty::pp_instruction;
use std::fmt::Formatter;

impl GenericInstruction for Instruction {
    fn kind(&self) -> InstructionKind {
        match self {
            Instruction::Return(_) => InstructionKind::Return,
            Instruction::Jump { target } => InstructionKind::Jump {
                label: target.clone(),
            },
            Instruction::JumpIfZero { target, .. } | Instruction::JumpIfNotZero { target, .. } => {
                InstructionKind::ConditionalJump {
                    label: target.clone(),
                }
            }
            Instruction::Label(label) => InstructionKind::Label(label.clone()),
            _ => InstructionKind::Other,
        }
    }

    fn pp(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        pp_instruction(f, self)
    }
}

pub type Cfg = GenericCfg<Instruction>;
pub type CfgNode = GenericNode<Instruction>;
