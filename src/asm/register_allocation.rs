use crate::asm::cfg::{Cfg, CfgNode};
use crate::asm::ir::{Instruction, Operand, Reg};
use crate::asm::{BackendSymbolData, BackendSymbolTable};
use crate::optimization::cfg::Annotation;
use crate::symbol::Symbol;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Register {
    Hard(Reg),
    Pseudo(Symbol),
}

struct InterferenceNode {
    id: Register,
    neighbors: Vec<Register>,
    spill_cost: f64,
    color: Option<i32>,
    pruned: bool,
}

struct InterferenceGraph {
    nodes: Vec<InterferenceNode>,
}

impl InterferenceGraph {
    fn contains(&self, reg: &Register) -> bool {
        self.nodes.iter().any(|n| n.id == *reg)
    }

    fn get_node(&self, id: &Register) -> &InterferenceNode {
        self.nodes.iter().find(|n| n.id == *id).unwrap()
    }

    fn get_node_mut(&mut self, id: &Register) -> &mut InterferenceNode {
        self.nodes.iter_mut().find(|n| n.id == *id).unwrap()
    }

    fn add_edge(&mut self, from: &Register, to: &Register) {
        self.get_node_mut(from)
            .neighbors
            .push(to.clone());
        self.get_node_mut(to)
            .neighbors
            .push(from.clone());
    }

    fn unpruned_nodes(&self) -> Vec<&InterferenceNode> {
        self.nodes.iter().filter(|n| !n.pruned).collect()
    }

    fn num_unpruned_neighbors(&self, node: &InterferenceNode) -> usize {
       node.neighbors.
           iter()
           .filter(|neighbor| !self.get_node(neighbor).pruned)
           .count()
    }
}

fn allocate_registers(instructions: &[Instruction], symbols: &BackendSymbolTable) {
    let mut interference_graph = build_interference_graph(instructions);
    let cfg = Cfg::new(instructions);
    let liveness = analyze_liveness(&cfg, symbols);
    add_edges(&mut interference_graph, &cfg, &liveness, symbols);
}

fn build_interference_graph(instructions: &[Instruction]) -> InterferenceGraph {
    let mut nodes = Vec::new();
    let mut spill_costs = HashMap::new();
    add_hard_registers(&mut nodes);
    add_pseudo_registers(&mut nodes, &mut spill_costs, instructions);

    let cfg = Cfg::new(instructions);

    InterferenceGraph { nodes }
}

const AVAILABLE_REGS: [Reg; 12] = [
    Reg::Ax,
    Reg::Bx,
    Reg::Cx,
    Reg::Dx,
    Reg::Di,
    Reg::Si,
    Reg::R8,
    Reg::R9,
    Reg::R12,
    Reg::R13,
    Reg::R14,
    Reg::R15,
];

fn add_hard_registers(nodes: &mut Vec<InterferenceNode>) {
    for reg in AVAILABLE_REGS {
        let neighbors = AVAILABLE_REGS
            .iter()
            .filter(|&r| r != &reg)
            .map(|r| Register::Hard(*r))
            .collect();
        nodes.push(InterferenceNode {
            id: Register::Hard(reg),
            neighbors,
            spill_cost: 0.0,
            color: None,
            pruned: false,
        });
    }
}

fn add_pseudo_registers(
    nodes: &mut Vec<InterferenceNode>,
    spill_costs: &mut HashMap<Symbol, f64>,
    instructions: &[Instruction],
) {
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
                add_pseudo_reg(nodes, spill_costs, op1);
                add_pseudo_reg(nodes, spill_costs, op2);
            }
            Instruction::Unary(_, _, op)
            | Instruction::Idiv(_, op)
            | Instruction::Div(_, op)
            | Instruction::SetCC(_, op) => {
                add_pseudo_reg(nodes, spill_costs, op);
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

fn add_pseudo_reg(
    nodes: &mut Vec<InterferenceNode>,
    spill_costs: &mut HashMap<Symbol, f64>,
    op: &Operand,
) {
    if let Operand::Pseudo(name) = op {
        let sp = spill_costs.entry(name.clone()).or_insert(0.0);
        *sp += 1.0;
        nodes.push(InterferenceNode {
            id: Register::Pseudo(name.clone()),
            neighbors: vec![],
            spill_cost: 0.0,
            color: None,
            pruned: false,
        })
    };
}

fn add_spill_costs(nodes: &mut Vec<InterferenceNode>, spill_costs: &HashMap<Symbol, f64>) {
    for node in nodes {
        match &node.id {
            Register::Pseudo(name) => {
                let spill_cost = spill_costs
                    .get(name)
                    .expect("Missing spill cost for pseudo register");
                node.spill_cost = *spill_cost;
            }
            Register::Hard(_) => {
                node.spill_cost = f64::INFINITY;
            }
        }
    }
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

fn find_used_and_updated(
    instruction: &Instruction,
    symbols: &BackendSymbolTable,
) -> (Vec<Operand>, Vec<Operand>) {
    match instruction {
        Instruction::Mov(_, src, dst) => (vec![src.clone()], vec![dst.clone()]),
        Instruction::Movsx(_, _, _, _) => todo!(),
        Instruction::MovZeroExtend(_, _, _, _) => todo!(),
        Instruction::Lea(_, _) => todo!(),
        Instruction::Cvttsd2si(_, _, _) => todo!(),
        Instruction::Cvtsi2sd(_, _, _) => todo!(),
        Instruction::Unary(_, _, dst) => (vec![dst.clone()], vec![dst.clone()]),
        Instruction::Binary(_, _, src, dst) => (vec![src.clone(), dst.clone()], vec![dst.clone()]),
        Instruction::Cmp(_, v1, v2) => (vec![v1.clone(), v2.clone()], vec![]),
        Instruction::Idiv(_, divisor) => (
            vec![divisor.clone(), Reg::Ax.into(), Reg::Dx.into()],
            vec![Reg::Ax.into(), Reg::Dx.into()],
        ),
        Instruction::Div(_, _) => todo!(),
        Instruction::Cdq(_) => (vec![Reg::Ax.into()], vec![Reg::Dx.into()]),
        Instruction::SetCC(_, dst) => (vec![], vec![dst.clone()]),
        Instruction::Push(v) => (vec![v.clone()], vec![]),
        Instruction::Pop(_) => todo!(),
        Instruction::Call(name) => {
            let Some(BackendSymbolData::Fn { param_registers }) = symbols.get(name) else {
                panic!("Function {} does not have symbol data", name);
            };
            let used: Vec<Operand> = param_registers.iter().map(|&reg| reg.into()).collect();
            (
                used,
                vec![
                    Reg::Di.into(),
                    Reg::Si.into(),
                    Reg::Dx.into(),
                    Reg::Cx.into(),
                    Reg::R8.into(),
                    Reg::R9.into(),
                    Reg::Ax.into(),
                ],
            )
        }
        Instruction::Jmp(_)
        | Instruction::Label(_)
        | Instruction::Ret
        | Instruction::JmpCC(_, _) => (vec![], vec![]),
    }
}

fn liveness_transfer_function(
    annotations: &mut Annotation<RegSet>,
    node: &CfgNode,
    symbols: &BackendSymbolTable,
    end_live_registers: RegSet,
) {
    let mut current_live_registers = end_live_registers.clone();
    for (i, instruction) in node.instructions.iter().enumerate().rev() {
        annotations.annotate_instruction(node.id, i, current_live_registers.clone());
        let (used, updated) = find_used_and_updated(instruction, symbols);
        for operand in updated {
            if let Operand::Reg(reg) = operand {
                current_live_registers.remove(&Register::Hard(reg));
            }
        }

        for operand in used {
            if let Operand::Reg(reg) = operand {
                current_live_registers.insert(Register::Hard(reg));
            }
        }
    }
    annotations.annotate_block(node.id, current_live_registers);
}

fn analyze_liveness(cfg: &Cfg, symbols: &BackendSymbolTable) -> Annotation<RegSet> {
    let live_registers = RegSet::new();
    let mut annotations = Annotation::empty();

    let mut worklist = VecDeque::new();
    for node_id in cfg.all_ids() {
        if node_id == cfg.exit_id() || node_id == cfg.entry_id() {
            continue;
        }
        worklist.push_back(node_id);
        annotations.annotate_block(node_id, live_registers.clone());
    }

    while let Some(node_id) = worklist.pop_back() {
        let old_registers = &annotations.get_block_annotation(&node_id).clone();
        let node = cfg.get_node(node_id);
        let incoming_registers = liveness_meet_operator(&mut annotations, cfg, node);
        liveness_transfer_function(&mut annotations, node, symbols, incoming_registers);
        if old_registers != annotations.get_block_annotation(&node_id) {
            for pred_id in &node.predecessors {
                if pred_id == &cfg.entry_id() {
                    continue;
                }
                if !worklist.contains(pred_id) {
                    worklist.push_back(*pred_id);
                }
            }
        }
    }
    annotations
}

fn add_edges(
    interference_graph: &mut InterferenceGraph,
    cfg: &Cfg,
    liveness: &Annotation<RegSet>,
    symbols: &BackendSymbolTable,
) {
    for node_id in cfg.all_ids() {
        if node_id == cfg.exit_id() || node_id == cfg.entry_id() {
            continue;
        }

        let node = cfg.get_node(node_id);

        for (i, instruction) in node.instructions.iter().enumerate() {
            let (used, updated) = find_used_and_updated(instruction, symbols);
            let live_registers = liveness.get_instruction_annotation(node_id, i);
            for l in live_registers {
                if let Instruction::Mov(_, src, _) = instruction
                    && let Operand::Reg(src) = src
                    && *l == Register::Hard(*src)
                {
                    continue;
                }
                for u in &updated {
                    if let Some(u) = &u.as_register()
                        && l != u
                        && interference_graph.contains(l)
                        && interference_graph.contains(u)
                    {
                        interference_graph.add_edge(l, u);
                    }
                }
            }
        }
    }
}

impl Operand {
    fn as_register(&self) -> Option<Register> {
        if let Operand::Reg(reg) = self {
            Some(Register::Hard(*reg))
        } else {
            None
        }
    }
}

const CALLEE_SAVED_REGS: [Register; 4] = [
    Register::Hard(Reg::R12),
    Register::Hard(Reg::R13),
    Register::Hard(Reg::R14),
    Register::Hard(Reg::R15)
];

fn color_graph(interference_graph: &mut InterferenceGraph, registers: &[Reg]) {
    let mut chosen_node = None;
    let remaining = interference_graph.unpruned_nodes();
    for node in &remaining {
        let degree = interference_graph.num_unpruned_neighbors(node);
        if degree < registers.len() {
            chosen_node = Some(node.id.clone());
        }
    }

    if chosen_node.is_none() {
        let mut best_spill_metric = f64::INFINITY;
        for node in remaining {
            if node.spill_cost < best_spill_metric {
                let degree = interference_graph.num_unpruned_neighbors(&node);
                let spill_metric = node.spill_cost / (degree as f64);
                if spill_metric < best_spill_metric {
                    chosen_node = Some(node.id.clone());
                    best_spill_metric = spill_metric;
                }
            }
        }
    }

    let chosen_node = chosen_node.expect("One node must have been chosen");
    interference_graph.get_node_mut(&chosen_node).pruned = true;
    color_graph(interference_graph, registers);

    let mut colors: HashSet<i32> = (0..registers.len() as i32).collect();
    for neighbor_id in &interference_graph.get_node(&chosen_node).neighbors {
        let neighbor = interference_graph.get_node(neighbor_id);
        if let Some(color) = neighbor.color {
            colors.remove(&color);
        }
    }

    if !colors.is_empty() {
        let color = if CALLEE_SAVED_REGS.contains(&chosen_node) {
            colors.iter().copied().max().unwrap()
        } else{
            colors.iter().copied().min().unwrap()
        };
        interference_graph.get_node_mut(&chosen_node).color = Some(color);
    }
}

struct RegisterMap {
    register_map: HashMap<Symbol, Reg>,
    callee_saved_regs: HashSet<Reg>,
}

fn create_register_map(interference_graph: &InterferenceGraph) -> RegisterMap {
    let mut color_map = HashMap::new();
    for node in interference_graph.nodes.iter() {
        if let Register::Hard(reg) = node.id {
            color_map.insert(node.color.unwrap(), reg);
        }
    }

    let mut register_map = HashMap::new();
    let mut callee_saved_regs = HashSet::new();
    for node in interference_graph.nodes.iter() {
        if let Register::Pseudo(name) = &node.id
        && let Some(color) = node.color{
            let hard_reg = *color_map.get(&color).unwrap();
            register_map.insert(name.clone(), hard_reg);
            if CALLEE_SAVED_REGS.contains(&Register::Hard(hard_reg)) {
                callee_saved_regs.insert(hard_reg);
            }
        }
    }

    RegisterMap {
        register_map,
        callee_saved_regs,
    }
}
