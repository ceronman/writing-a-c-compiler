use crate::asm::ir::Instruction;
use crate::optimization::cfg::{GenericCfg, GenericInstruction, GenericNode, InstructionKind};
use std::fmt::Formatter;

pub type Cfg = GenericCfg<Instruction>;
pub type CfgNode = GenericNode<Instruction>;

impl GenericInstruction for Instruction {
    fn kind(&self) -> InstructionKind {
        match self {
            Instruction::Ret => InstructionKind::Return,
            Instruction::Jmp(target) => InstructionKind::Jump {
                label: target.clone(),
            },
            Instruction::JmpCC(_, target) => InstructionKind::ConditionalJump {
                label: target.clone(),
            },
            Instruction::Label(label) => InstructionKind::Label(label.clone()),
            _ => InstructionKind::Other,
        }
    }

    fn pp(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}
