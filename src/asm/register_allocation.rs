use crate::asm::cfg::Cfg;
use crate::asm::ir::{Instruction, Operand, Reg};
use crate::optimization::cfg;
use crate::symbol::Symbol;

enum NodeId {
    Reg(Reg),
    PseudoReg(Symbol),
}

struct Node {
    id: NodeId,
    neighbors: Vec<NodeId>,
    spill_cost: f64,
    color: Option<i32>,
    pruned: bool,
}

struct InferenceGraph {
    nodes: Vec<Node>,
}

fn allocate_registers(instructions: &[Instruction]) {
    let inference_graph = build_inference_graph(instructions);
}

fn build_inference_graph(instructions: &[Instruction]) -> InferenceGraph {
    let mut nodes = Vec::new();

    add_hard_registers(&mut nodes);
    add_pseudo_registers(&mut nodes, instructions);

    let cfg = Cfg::new(instructions);

    InferenceGraph {
        nodes,
    }
}

const AVAILABLE_REGS: [Reg; 12] = [Reg::Ax, Reg::Bx, Reg::Cx, Reg::Dx, Reg::Di, Reg::Si, Reg::R8, Reg::R9, Reg::R12, Reg::R13, Reg::R14, Reg::R15];

fn add_hard_registers(nodes: &mut Vec<Node>) {
    for reg in AVAILABLE_REGS {
        let neighbors = AVAILABLE_REGS.iter().filter(|&r| r != &reg).map(|r| NodeId::Reg(*r)).collect();
        nodes.push(Node {
            id: NodeId::Reg(reg),
            neighbors,
            spill_cost: 0.0,
            color: None,
            pruned: false,
        });
    }
}

fn add_pseudo_registers(nodes: &mut Vec<Node>, instructions: &[Instruction]) {
    for instruction in instructions {
        match instruction {
            Instruction::Mov(_, op1, op2)
            | Instruction::Movsx(_, op1, _, op2)
            | Instruction::MovZeroExtend(_, op1, _, op2)
            | Instruction::Lea(op1, op2)
            | Instruction::Cvttsd2si(_, op1, op2)
            | Instruction::Cvtsi2sd(_, op1, op2)
            | Instruction::Binary(_, _, op1, op2)
            | Instruction::Cmp(_, op1, op2) => {
                add_pseudo_reg(nodes, op1);
                add_pseudo_reg(nodes, op2);
            }
            Instruction::Unary(_, _, op)
            | Instruction::Idiv(_, op )
            | Instruction::Div(_, op)
            | Instruction::SetCC(_, op) => {
                add_pseudo_reg(nodes, op);
            }
            Instruction::Cdq(_)
            | Instruction::Jmp(_)
            | Instruction::JmpCC(_, _)
            | Instruction::Label(_)
            | Instruction::Push(_)
            | Instruction::Pop(_)
            | Instruction::Call(_)
            | Instruction::Ret => {}
        }
    }
}

fn add_pseudo_reg(nodes: &mut Vec<Node>, op: &Operand) {
    match op {
        Operand::Pseudo(name) => {
            nodes.push(Node {
                id: NodeId::PseudoReg(name.clone()),
                neighbors: vec![],
                spill_cost: 0.0,
                color: None,
                pruned: false,
            })
        },
        _ => {}
    };
}