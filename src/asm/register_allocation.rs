use std::collections::HashSet;
use crate::asm::cfg::{Cfg, CfgNode};
use crate::asm::ir::{Instruction, Operand, Reg};
use crate::optimization::cfg::Annotation;
use crate::symbol::Symbol;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Register {
    Hard(Reg),
    Pseudo(Symbol),
}

struct Node {
    id: Register,
    neighbors: Vec<Register>,
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
        let neighbors = AVAILABLE_REGS.iter().filter(|&r| r != &reg).map(|r| Register::Hard(*r)).collect();
        nodes.push(Node {
            id: Register::Hard(reg),
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
                id: Register::Pseudo(name.clone()),
                neighbors: vec![],
                spill_cost: 0.0,
                color: None,
                pruned: false,
            })
        },
        _ => {}
    };
}

type RegSet = HashSet<Register>;

fn liveness_meet_operator(
    annotations: &mut Annotation<RegSet>,
    cfg: &Cfg,
    node: &CfgNode,
) -> RegSet {
    let mut live_registers = RegSet::new();
    for succ_id in &node.successors {
        if succ_id == &cfg.exit_id() {
            live_registers.insert(Register::Hard(Reg::Ax));
        } else {
            let succ_live_registers = annotations.get_block_annotation(succ_id);
            live_registers.extend(succ_live_registers.iter().cloned());
        }
    }
    live_registers
}

fn find_used_and_updated(instruction: &Instruction) -> (Vec<Operand>, Vec<Operand>) {
    match instruction {
        Instruction::Mov(_, src, dst) => {
            (vec![src.clone()], vec![dst.clone()])
        }
        Instruction::Movsx(_, _, _, _) => todo!(),
        Instruction::MovZeroExtend(_, _, _, _) => todo!(),
        Instruction::Lea(_, _) => todo!(),
        Instruction::Cvttsd2si(_, _, _) => todo!(),
        Instruction::Cvtsi2sd(_, _, _) => todo!(),
        Instruction::Unary(_, _, dst) => {
            (vec![dst.clone()], vec![dst.clone()])
        }
        Instruction::Binary(_, _, src, dst) => {
            (vec![src.clone(), dst.clone()], vec![dst.clone()])
        }
        Instruction::Cmp(_, v1, v2) => {
            (vec![v1.clone(), v2.clone()], vec![])
        }
        Instruction::Idiv(_, divisor) => {
            (vec![divisor.clone(), Reg::Ax.into(), Reg::Dx.into()], vec![Reg::Ax.into(), Reg::Dx.into()])
        }
        Instruction::Div(_, _) => todo!(),
        Instruction::Cdq(_) => {
            (vec![Reg::Ax.into()], vec![Reg::Dx.into()])
        }
        Instruction::SetCC(_, dst) => {
            (vec![], vec![dst.clone()])
        }
        Instruction::Push(v) => {
            (vec![v.clone()], vec![])
        }
        Instruction::Pop(_) => todo!(),
        Instruction::Call(_) => {
            todo!()
        }
        Instruction::Jmp(_)
        | Instruction::Label(_)
        | Instruction::Ret
        | Instruction::JmpCC(_, _) => {
            (vec![], vec![])
        }
    }
}