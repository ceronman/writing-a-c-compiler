use crate::asm::cfg::{Cfg, CfgNode};
use crate::asm::ir::{AsmType, Function, Instruction, Operand, Reg};
use crate::asm::{BackendSymbolData, BackendSymbolTable};
use crate::optimization::cfg::Annotation;
use crate::symbol::Symbol;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

pub(super) fn allocate_registers(function: &mut Function, symbols: &mut BackendSymbolTable) {
    allocate_general_purpose_regs(function, symbols);
}

const GENERAL_PURPOSE_REGS: [Reg; 12] = [
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

const SSE_REGS: [Reg; 14] = [
    Reg::XMM0,
    Reg::XMM1,
    Reg::XMM2,
    Reg::XMM3,
    Reg::XMM4,
    Reg::XMM5,
    Reg::XMM6,
    Reg::XMM7,
    Reg::XMM8,
    Reg::XMM9,
    Reg::XMM10,
    Reg::XMM11,
    Reg::XMM12,
    Reg::XMM13,
];

fn allocate_general_purpose_regs(function: &mut Function, symbols: &mut BackendSymbolTable) {
    let mut interference_graph = build_interference_graph(function, symbols, &GENERAL_PURPOSE_REGS, &[AsmType::Byte, AsmType::Longword, AsmType::Quadword]);
    color_graph(&mut interference_graph, &GENERAL_PURPOSE_REGS);
    let register_map = create_register_map(&interference_graph);
    replace_pseudo_regs(&mut function.instructions, &register_map.register_map);
    let Some(BackendSymbolData::Fn { callee_saved_registers, .. }) = symbols.get_mut(&function.name) else {
        panic!("Function {} does not have symbol data", function.name);
    };
    callee_saved_registers.clear(); // just in case
    callee_saved_registers.extend(register_map.callee_saved_regs);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Register {
    Hard(Reg),
    Pseudo(Symbol),
}

#[derive(Debug)]
struct InterferenceNode {
    id: Register,
    neighbors: BTreeSet<Register>,
    spill_cost: f64,
    color: Option<i32>,
    pruned: bool,
}

#[derive(Debug)]
struct InterferenceGraph {
    nodes: BTreeMap<Register, InterferenceNode>,
}

impl InterferenceGraph {
    fn contains(&self, reg: &Register) -> bool {
        self.nodes.contains_key(reg)
    }

    fn get_node(&self, id: &Register) -> &InterferenceNode {
        self.nodes.get(id).unwrap()
    }

    fn get_node_mut(&mut self, id: &Register) -> &mut InterferenceNode {
        self.nodes.get_mut(id).unwrap()
    }

    fn add_edge(&mut self, from: &Register, to: &Register) {
        self.get_node_mut(from)
            .neighbors
            .insert(to.clone());
        self.get_node_mut(to)
            .neighbors
            .insert(from.clone());
    }

    fn unpruned_nodes(&self) -> Vec<&InterferenceNode> {
        self.nodes.values().filter(|n| !n.pruned).collect()
    }

    fn num_unpruned_neighbors(&self, node: &InterferenceNode) -> usize {
       node.neighbors.
           iter()
           .filter(|neighbor| !self.get_node(neighbor).pruned)
           .count()
    }
}

fn build_interference_graph(function: &Function, symbols: &BackendSymbolTable, available_regs: &[Reg], allowed_types: &[AsmType]) -> InterferenceGraph {
    let mut nodes = BTreeMap::new();
    let mut spill_costs = HashMap::new();
    add_hard_registers(&mut nodes, available_regs);
    add_pseudo_registers(&mut nodes, function, &mut spill_costs, symbols, allowed_types);
    let mut interference_graph = InterferenceGraph { nodes };
    add_spill_costs(&mut interference_graph.nodes, &spill_costs);
    let cfg = Cfg::new(&function.instructions);
    let liveness = analyze_liveness(&cfg, symbols);
    add_edges(&mut interference_graph, &cfg, &liveness, symbols);
    interference_graph
}

fn add_hard_registers(nodes: &mut BTreeMap<Register, InterferenceNode>, available_regs: &[Reg]) {
    for reg in available_regs {
        let neighbors = available_regs
            .iter()
            .filter(|&r| r != reg)
            .map(|r| Register::Hard(*r))
            .collect();
        nodes.insert(Register::Hard(*reg), InterferenceNode {
            id: Register::Hard(*reg),
            neighbors,
            spill_cost: 0.0,
            color: None,
            pruned: false,
        });
    }
}

fn add_pseudo_registers(
    nodes: &mut BTreeMap<Register, InterferenceNode>,
    function: &Function,
    spill_costs: &mut HashMap<Symbol, f64>,
    symbols: &BackendSymbolTable,
    allowed_types: &[AsmType]
) {
    let Some(BackendSymbolData::Fn { aliased_vars, .. }) = symbols.get(&function.name) else {
        panic!("Function {} does not have symbol data", function.name);
    };
    for instruction in &function.instructions {
        match instruction {
            Instruction::Mov(ty, op1, op2)
            | Instruction::Cvttsd2si(ty, op1, op2)
            | Instruction::Cvtsi2sd(ty, op1, op2)
            | Instruction::Binary(ty, _, op1, op2)
            | Instruction::Cmp(ty, op1, op2) => {
                add_pseudo_reg(nodes, spill_costs, op1, symbols, ty, allowed_types, aliased_vars);
                add_pseudo_reg(nodes, spill_costs, op2, symbols, ty, allowed_types, aliased_vars);
            }
            Instruction::Movsx(ty1, op1, ty2, op2)
            | Instruction::MovZeroExtend(ty1, op1, ty2, op2) => {
                add_pseudo_reg(nodes, spill_costs, op1, symbols, ty1, allowed_types, aliased_vars);
                add_pseudo_reg(nodes, spill_costs, op2, symbols, ty2, allowed_types, aliased_vars);
            }
            Instruction::Lea(op1, op2) => {
                add_pseudo_reg(nodes, spill_costs, op1, symbols, &AsmType::Quadword, allowed_types, aliased_vars);
                add_pseudo_reg(nodes, spill_costs, op2, symbols, &AsmType::Quadword, allowed_types, aliased_vars);
            }
            Instruction::Unary(ty, _, op)
            | Instruction::Idiv(ty, op)
            | Instruction::Div(ty, op) => {
                add_pseudo_reg(nodes, spill_costs, op, symbols, ty, allowed_types, aliased_vars);
            }
            Instruction::SetCC(_, op) => {
                add_pseudo_reg(nodes, spill_costs, op, symbols, &AsmType::Byte, allowed_types, aliased_vars);
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
    nodes: &mut BTreeMap<Register, InterferenceNode>,
    spill_costs: &mut HashMap<Symbol, f64>,
    op: &Operand,
    symbols: &BackendSymbolTable,
    ty: &AsmType,
    allowed_types: &[AsmType],
    aliased_vars: &HashSet<Symbol>
) {
    if let Operand::Pseudo(name) = op
        && let Some(BackendSymbolData::Obj { is_static, ..}) = symbols.get(name)
        && !is_static
        && allowed_types.contains(ty)
        && !aliased_vars.contains(name) {

        let sp = spill_costs.entry(name.clone()).or_insert(0.0);
        *sp += 1.0;
        let id = Register::Pseudo(name.clone());
        nodes.insert(id.clone(), InterferenceNode {
            id,
            neighbors: BTreeSet::new(),
            spill_cost: 0.0,
            color: None,
            pruned: false,
        });
    };
}

fn add_spill_costs(nodes: &mut BTreeMap<Register, InterferenceNode>, spill_costs: &HashMap<Symbol, f64>) {
    for node in nodes.values_mut() {
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
            let Some(BackendSymbolData::Fn { param_registers, .. }) = symbols.get(name) else {
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
            if let Some(reg) = operand.as_register() {
                current_live_registers.remove(&reg);
            }
        }

        for operand in used {
            if let Some(reg) = operand.as_register() {
                current_live_registers.insert(reg);
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
            let (_used, updated) = find_used_and_updated(instruction, symbols);
            let live_registers = liveness.get_instruction_annotation(node_id, i);
            for l in live_registers {
                if let Instruction::Mov(_, src, _) = instruction
                    && let Some(src) = src.as_register()
                    && *l == src
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
        match self {
            Operand::Reg(reg) => {
                Some(Register::Hard(*reg))
            }
            Operand::Pseudo(name) => {
                Some(Register::Pseudo(name.clone()))
            }
            _ => {
                None
            }
        }
    }
}

const CALLEE_SAVED_REGS: [Register; 5] = [
    Register::Hard(Reg::Bx),
    Register::Hard(Reg::R12),
    Register::Hard(Reg::R13),
    Register::Hard(Reg::R14),
    Register::Hard(Reg::R15)
];

fn color_graph(interference_graph: &mut InterferenceGraph, registers: &[Reg]) {
    let remaining = interference_graph.unpruned_nodes();
    if remaining.is_empty() {
        return
    }

    let mut chosen_node = None;

    for node in &remaining {
        let degree = interference_graph.num_unpruned_neighbors(node);
        if degree < registers.len() {
            chosen_node = Some(node.id.clone());
            break;
        }
    }

    if chosen_node.is_none() {
        let mut best_spill_metric = f64::INFINITY;
        for node in remaining {
            let degree = interference_graph.num_unpruned_neighbors(&node);
            let spill_metric = node.spill_cost / (degree as f64);
            if spill_metric < best_spill_metric {
                chosen_node = Some(node.id.clone());
                best_spill_metric = spill_metric;
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
        let chosen_node = interference_graph.get_node_mut(&chosen_node);
        chosen_node.color = Some(color);
        chosen_node.pruned = false;
    }
}

struct RegisterMap {
    register_map: HashMap<Symbol, Reg>,
    callee_saved_regs: HashSet<Reg>,
}

fn create_register_map(interference_graph: &InterferenceGraph) -> RegisterMap {
    let mut color_map = HashMap::new();
    for node in interference_graph.nodes.values() {
        if let Register::Hard(reg) = node.id {
            color_map.insert(node.color.unwrap(), reg);
        }
    }

    let mut register_map = HashMap::new();
    let mut callee_saved_regs = HashSet::new();
    for node in interference_graph.nodes.values() {
        if let Register::Pseudo(name) = &node.id
        && let Some(color) = node.color {
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

fn replace_pseudo_regs(instructions: &mut Vec<Instruction>, reg_map: &HashMap<Symbol, Reg>) {
    fn replace_reg(op: &mut Operand, reg_map: &HashMap<Symbol, Reg>) {
        if let Operand::Pseudo(name) = op
            && let Some(reg) = reg_map.get(name) {

            *op = Operand::Reg(*reg);
        }
    }
    for instruction in instructions.iter_mut() {
        match instruction {
            Instruction::Mov(_, op1, op2)
            | Instruction::Movsx(_, op1, _, op2)
            | Instruction::MovZeroExtend(_, op1, _, op2)
            | Instruction::Lea(op1, op2)
            | Instruction::Cvttsd2si(_, op1, op2)
            | Instruction::Cvtsi2sd(_, op1, op2)
            | Instruction::Binary(_, _, op1, op2)
            | Instruction::Cmp(_, op1, op2) => {
                replace_reg(op1, reg_map);
                replace_reg(op2, reg_map);
            }
            Instruction::Unary(_, _, op)
            | Instruction::Idiv(_, op)
            | Instruction::Div(_, op)
            | Instruction::SetCC(_, op) => {
                replace_reg(op, reg_map);
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

    instructions.retain(|instruction| {
        match instruction {
            Instruction::Mov(_, Operand::Reg(src), Operand::Reg(dst)) => {
                src != dst
            }
            _ => true,
        }
    });
}
