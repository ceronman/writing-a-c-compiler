use crate::asm::cfg::{Cfg, CfgNode};
use crate::asm::ir::{AsmType, Function, Instruction, Operand, Reg};
use crate::asm::{BackendSymbolData, BackendSymbolTable};
use crate::optimization::cfg::Annotation;
use crate::symbol::Symbol;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

pub(super) fn allocate_registers(function: &mut Function, symbols: &mut BackendSymbolTable) {
    allocate_general_purpose_regs(function, symbols);
    allocate_sse_regs(function, symbols);
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
    let caller_saved_registers = [
        Reg::Di,
        Reg::Si,
        Reg::Dx,
        Reg::Cx,
        Reg::R8,
        Reg::R9,
        Reg::Ax,
    ];
    let mut interference_graph;
    loop {
        interference_graph = build_interference_graph(function, symbols, &GENERAL_PURPOSE_REGS, &[AsmType::Byte, AsmType::Longword, AsmType::Quadword], &caller_saved_registers);
        let coalesced_regs = coalesce(&mut interference_graph, &function.instructions, GENERAL_PURPOSE_REGS.len());
        if coalesced_regs.is_empty() {
            break;
        }
        rewrite_coalesced(&mut function.instructions, &coalesced_regs);
    }
    add_spill_costs(function, &mut interference_graph.nodes);
    color_graph(&mut interference_graph, &GENERAL_PURPOSE_REGS);
    let register_map = create_register_map(&interference_graph);
    replace_pseudo_regs(&mut function.instructions, &register_map.register_map);
    let Some(BackendSymbolData::Fn { callee_saved_registers, .. }) = symbols.get_mut(&function.name) else {
        panic!("Function {} does not have symbol data", function.name);
    };
    assert!(callee_saved_registers.is_empty());
    callee_saved_registers.extend(register_map.callee_saved_regs);
}

fn allocate_sse_regs(function: &mut Function, symbols: &mut BackendSymbolTable) {
    let mut interference_graph;
    loop {
        interference_graph = build_interference_graph(function, symbols, &SSE_REGS, &[AsmType::Double], &SSE_REGS);
        let coalesced_regs = coalesce(&mut interference_graph, &function.instructions, SSE_REGS.len());
        if coalesced_regs.is_empty() {
            break;
        }
        rewrite_coalesced(&mut function.instructions, &coalesced_regs);
    }
    add_spill_costs(function, &mut interference_graph.nodes);
    color_graph(&mut interference_graph, &SSE_REGS);
    let register_map = create_register_map(&interference_graph);
    replace_pseudo_regs(&mut function.instructions, &register_map.register_map);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Register {
    Hard(Reg),
    Pseudo(Symbol),
}

impl Register {
    fn is_hard(&self) -> bool {
        matches!(self, Register::Hard(_))
    }

    fn as_operand(&self) -> Operand {
        match self {
            Register::Hard(r) => Operand::Reg(*r),
            Register::Pseudo(name) => Operand::Pseudo(name.clone()),
        }
    }

    #[allow(dead_code)]
    fn debug_print(&self) -> String {
        match self {
            Register::Hard(reg) => format!("{:?}", reg).to_uppercase(),
            Register::Pseudo(name) => name.replace(".", "_")
        }
    }
}

#[derive(Debug)]
struct InterferenceNode {
    id: Register,
    neighbors: BTreeSet<Register>,
    spill_cost: f64,
    color: Option<i32>,
    pruned: bool,
}

struct DisjointSet(HashMap<Register, Register>);

impl DisjointSet {
    fn new() -> Self {
        DisjointSet(HashMap::new())
    }

    fn union(&mut self, a: &Register, b: &Register) {
        self.0.insert(a.clone(), b.clone());
    }

    fn find(&self, op: &Operand) -> Operand {
        let Some(r) = op.as_register() else { return op.clone() };
        let mut result = &r;
        while let Some(representative) = self.0.get(result) {
            result = representative;
        }
        result.as_operand()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[allow(dead_code)]
    fn debug_print(&self) {
        for (r, representative) in &self.0 {
            println!("{} -> {}", r.debug_print(), representative.debug_print());
        }
    }
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
    fn remove_edge(&mut self, from: &Register, to: &Register) {
        self.get_node_mut(from)
            .neighbors
            .remove(to);
        self.get_node_mut(to)
            .neighbors
            .remove(&from);
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

    fn are_neighbors(&self, reg1: &Register, reg2: &Register) -> bool {
        self.get_node(reg1).neighbors.contains(reg2) || self.get_node(reg2).neighbors.contains(reg1)
    }

    fn conservative_coalesceable(&self, src: &Register, dst: &Register, num_hard_regs: usize) -> bool {
        if self.briggs_test(src, dst, num_hard_regs) {
            return true;
        }
        if src.is_hard() {
            return self.george_test(src, dst, num_hard_regs);
        }
        if dst.is_hard() {
            return self.george_test(dst, src, num_hard_regs);
        }
        false
    }

    fn briggs_test(&self, src: &Register, dst: &Register, num_hard_regs: usize) -> bool {
        let mut significant_neighbors = 0;

        let src_node = self.get_node(src);
        let dst_node = self.get_node(dst);
        let combined_neighbors = src_node.neighbors.union(&dst_node.neighbors);
        for neighbor in combined_neighbors {
            let neighbor_node = self.get_node(neighbor);
            let mut degree = neighbor_node.neighbors.len();
            if self.are_neighbors(neighbor, src) && self.are_neighbors(neighbor, dst) {
                degree -= 1;
            }
            if degree >= num_hard_regs {
                significant_neighbors += 1;
            }
        }

        significant_neighbors < num_hard_regs
    }

    fn george_test(&self, hard_reg: &Register, pseudo_reg: &Register, num_hard_regs: usize) -> bool {
        let pseudo_reg_node = self.get_node(pseudo_reg);
        for neighbor in pseudo_reg_node.neighbors.iter() {
            if self.are_neighbors(neighbor, hard_reg) {
                continue
            }
            if self.get_node(neighbor).neighbors.len() < num_hard_regs {
                continue
            }
            return false
        }
        true
    }

    fn update(&mut self, to_merge: &Register, to_keep: &Register) {
        let neighbors = self.get_node(to_merge).neighbors.clone();
        for neighbor in neighbors.iter() {
            self.add_edge(to_keep, neighbor);
            self.remove_edge(to_merge, neighbor);
        }
        self.nodes.remove(to_merge);
    }

    #[allow(dead_code)]
    fn debug_print(&self) {
        println!("graph {{");
        for node in self.nodes.values() {
            if node.pruned {
                continue
            }

            for neighbor in &node.neighbors {
                let nb_node = self.get_node(neighbor);
                if nb_node.pruned {
                    continue
                }

                if neighbor > &node.id {
                    continue
                }


                let left = node.id.debug_print();
                let right = neighbor.debug_print();
                println!("\t{left} -- {right}")
            }
        }
        println!("}}");
    }
}

fn build_interference_graph(function: &Function, symbols: &BackendSymbolTable, available_regs: &[Reg], allowed_types: &[AsmType], caller_saved_registers: &[Reg]) -> InterferenceGraph {
    let mut nodes = BTreeMap::new();
    add_hard_registers(&mut nodes, available_regs);
    add_pseudo_registers(&mut nodes, function, symbols, allowed_types);
    let mut interference_graph = InterferenceGraph { nodes };
    let cfg = Cfg::new(&function.instructions);
    let liveness = analyze_liveness(function, &cfg, symbols, caller_saved_registers);
    add_edges(&mut interference_graph, &cfg, &liveness, symbols, caller_saved_registers);
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
    symbols: &BackendSymbolTable,
    allowed_types: &[AsmType]
) {
    let Some(BackendSymbolData::Fn { aliased_vars, .. }) = symbols.get(&function.name) else {
        panic!("Function {} does not have symbol data", function.name);
    };
    walk_operands(&function.instructions, |op| {
        if let Operand::Pseudo(name) = op
            && let Some(BackendSymbolData::Obj { is_static, ty, ..}) = symbols.get(name)
            && !is_static
            && allowed_types.contains(ty)
            && !aliased_vars.contains(name) {

            let id = Register::Pseudo(name.clone());
            nodes.insert(id.clone(), InterferenceNode {
                id,
                neighbors: BTreeSet::new(),
                spill_cost: 0.0,
                color: None,
                pruned: false,
            });
        };
    });
}

fn walk_operands(instructions: &[Instruction], mut lambda: impl FnMut(&Operand)) {
    for instruction in instructions {
        match instruction {
            Instruction::Mov(_, op1, op2)
            | Instruction::Movsx(_, op1, _, op2)
            | Instruction::MovZeroExtend(_, op1, _, op2)
            | Instruction::Binary(_, _, op1, op2)
            | Instruction::Cmp(_, op1, op2)
            | Instruction::Lea(op1, op2)
            | Instruction::Cvttsd2si(_, op1, op2)
            | Instruction::Cvtsi2sd(_, op1, op2) => {
                lambda(op1);
                lambda(op2);
            }

            Instruction::Unary(_, _, op)
            | Instruction::Idiv(_, op)
            | Instruction::SetCC(_, op)
            | Instruction::Push(op)
            | Instruction::Div(_, op) => {
                lambda(op);
            }

            Instruction::Cdq(_)
            | Instruction::Jmp(_)
            | Instruction::JmpCC(_, _)
            | Instruction::Label(_)
            | Instruction::Pop(_)
            | Instruction::Call(_)
            | Instruction::Ret => {}
        }
    }
}

fn coalesce(graph: &mut InterferenceGraph, instructions: &[Instruction], num_hard_regs: usize) -> DisjointSet {
    let mut coalesced_regs = DisjointSet::new();

    for instruction in instructions {
        if let Instruction::Mov(_, src, dst) = instruction {
            let src = coalesced_regs.find(&src);
            let Some(src) = src.as_register() else { continue };

            let dst = coalesced_regs.find(&dst);
            let Some(dst) = dst.as_register() else { continue };

            if graph.contains(&src)
                && graph.contains(&dst)
                && src != dst
                && !graph.are_neighbors(&src, &dst)
                && graph.conservative_coalesceable(&src, &dst, num_hard_regs) {

                let (to_keep, to_merge) = if src.is_hard() {
                    (src, dst)
                } else {
                    (dst, src)
                };

                coalesced_regs.union(&to_merge, &to_keep);
                graph.update(&to_merge, &to_keep);
            }
        }
    }

    coalesced_regs
}

// TODO: duplicate code
fn rewrite_coalesced(instructions: &mut Vec<Instruction>, coalesced_regs: &DisjointSet) {
    fn replace_reg(op: &mut Operand, coalesced_regs: &DisjointSet) {
        *op = coalesced_regs.find(op)
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
                replace_reg(op1, coalesced_regs);
                replace_reg(op2, coalesced_regs);
            }
            Instruction::Unary(_, _, op)
            | Instruction::Idiv(_, op)
            | Instruction::Div(_, op)
            | Instruction::Push(op)
            | Instruction::SetCC(_, op) => {
                replace_reg(op, coalesced_regs);
            }
            Instruction::Cdq(_)
            | Instruction::Pop(_)
            | Instruction::Jmp(_)
            | Instruction::JmpCC(_, _)
            | Instruction::Label(_)
            | Instruction::Call(_)
            | Instruction::Ret => {}
        }
    }

    instructions.retain(|instruction| {
        match instruction {
            Instruction::Mov(_, Operand::Reg(src), Operand::Reg(dst)) => {
                src != dst
            }
            Instruction::Mov(_, Operand::Pseudo(src), Operand::Pseudo(dst)) => {
                src != dst
            }
            _ => true,
        }
    });
}

fn add_spill_costs(function: &Function, nodes: &mut BTreeMap<Register, InterferenceNode>) {
    let mut spill_costs = HashMap::new();
    walk_operands(&function.instructions, |op| {
        if let Operand::Pseudo(name) = op {
            let sp = spill_costs.entry(name.clone()).or_insert(0.0);
            *sp += 1.0;
        }
    });
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
    ret_registers: &[Reg]
) -> RegSet {
    let mut live_registers = RegSet::new();
    for succ_id in &node.successors {
        if succ_id == &cfg.exit_id() {
            live_registers.extend(ret_registers.iter().map(|r| Register::Hard(*r)));
        } else {
            let succ_live_registers = annotations.get_block_annotation(succ_id);
            live_registers.extend(succ_live_registers.iter().cloned());
        }
    }
    live_registers
}

struct UsedAndUpdated {
    used: Vec<Operand>,
    updated: Vec<Operand>,
}

fn find_used_and_updated(
    instruction: &Instruction,
    symbols: &BackendSymbolTable,
    caller_saved_registers: &[Reg]
) -> UsedAndUpdated {
    let mut used_and_updated = match instruction {
        Instruction::Mov(_, src, dst)
        | Instruction::Movsx(_, src, _, dst)
        | Instruction::MovZeroExtend(_, src, _, dst)
        | Instruction::Cvttsd2si(_, src, dst)
        | Instruction::Cvtsi2sd(_, src, dst)
        | Instruction::Lea(src, dst) => {
            UsedAndUpdated { used: vec![src.clone()], updated: vec![dst.clone()] }
        },
        Instruction::Unary(_, _, dst) => {
            UsedAndUpdated { used: vec![dst.clone()], updated: vec![dst.clone()] }
        },
        Instruction::Binary(_, _, src, dst) => {
            UsedAndUpdated { used: vec![src.clone(), dst.clone()], updated: vec![dst.clone()] }
        },
        Instruction::Cmp(_, v1, v2) => {
            UsedAndUpdated { used: vec![v1.clone(), v2.clone()], updated: vec![] }
        },
        Instruction::Div(_, divisor)
        | Instruction::Idiv(_, divisor) => UsedAndUpdated {
            used: vec![divisor.clone(), Reg::Ax.into(), Reg::Dx.into()],
            updated: vec![Reg::Ax.into(), Reg::Dx.into()],
        },
        Instruction::Cdq(_) => UsedAndUpdated { used: vec![Reg::Ax.into()], updated: vec![Reg::Dx.into()] },
        Instruction::SetCC(_, dst) => UsedAndUpdated { used: vec![], updated: vec![dst.clone()] },
        Instruction::Push(v) => UsedAndUpdated { used: vec![v.clone()], updated: vec![] },
        Instruction::Pop(reg) => UsedAndUpdated { used: vec![], updated: vec![Operand::Reg(*reg)] },
        Instruction::Call(name) => {
            let Some(BackendSymbolData::Fn { arg_registers: param_registers, .. }) = symbols.get(name) else {
                panic!("Function {} does not have symbol data", name);
            };
            let used: Vec<Operand> = param_registers.iter().map(|&reg| reg.into()).collect();
            UsedAndUpdated {
                used,
                updated: caller_saved_registers.iter().map(|&reg| reg.into()).collect(),
            }
        }
        Instruction::Jmp(_)
        | Instruction::Label(_)
        | Instruction::Ret
        | Instruction::JmpCC(_, _) => UsedAndUpdated { used: vec![], updated: vec![] },
    };

    let mut used_mem_regs = Vec::new();
    for operand in used_and_updated.used.iter().chain(used_and_updated.updated.iter()) {
        match operand {
            Operand::Memory(reg, _) => {
                used_mem_regs.push(Operand::Reg(*reg));
            }
            Operand::Indexed(reg1, reg2, _) => {
                used_mem_regs.push(Operand::Reg(*reg1));
                used_mem_regs.push(Operand::Reg(*reg2));
            }
            _ => continue
        }
    }

    used_and_updated.used.extend(used_mem_regs);

    used_and_updated
}

fn liveness_transfer_function(
    annotations: &mut Annotation<RegSet>,
    node: &CfgNode,
    symbols: &BackendSymbolTable,
    end_live_registers: RegSet,
    caller_saved_registers: &[Reg]
) {
    let mut current_live_registers = end_live_registers.clone();
    for (i, instruction) in node.instructions.iter().enumerate().rev() {
        annotations.annotate_instruction(node.id, i, current_live_registers.clone());
        let uu = find_used_and_updated(instruction, symbols, caller_saved_registers);
        for operand in uu.updated {
            if let Some(reg) = operand.as_register() {
                current_live_registers.remove(&reg);
            }
        }

        for operand in uu.used {
            if let Some(reg) = operand.as_register() {
                current_live_registers.insert(reg);
            }
        }
    }
    annotations.annotate_block(node.id, current_live_registers);
}

fn analyze_liveness(function: &Function, cfg: &Cfg, symbols: &BackendSymbolTable, caller_saved_registers: &[Reg]) -> Annotation<RegSet> {
    let Some(BackendSymbolData::Fn { ret_registers, .. }) = symbols.get(&function.name) else {
        panic!("Function {} does not have symbol data", function.name);
    };

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
        let incoming_registers = liveness_meet_operator(&mut annotations, cfg, node, ret_registers);
        liveness_transfer_function(&mut annotations, node, symbols, incoming_registers, caller_saved_registers);
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
    caller_saved_registers: &[Reg]
) {
    for node_id in cfg.all_ids() {
        if node_id == cfg.exit_id() || node_id == cfg.entry_id() {
            continue;
        }

        let node = cfg.get_node(node_id);

        for (i, instruction) in node.instructions.iter().enumerate() {
            let UsedAndUpdated { updated, .. } = find_used_and_updated(instruction, symbols, caller_saved_registers);
            let live_registers = liveness.get_instruction_annotation(node_id, i);
            for l in live_registers {
                if let Instruction::Mov(_, src, _) = instruction
                    && let Some(src) = src.as_register()
                    && *l == src
                {
                    continue;
                }
                for u in &updated {
                    if let Some(ur) = &u.as_register()
                        && l != ur
                        && interference_graph.contains(l)
                        && interference_graph.contains(ur)
                    {
                        interference_graph.add_edge(l, ur);
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
            | Instruction::Push(op)
            | Instruction::SetCC(_, op) => {
                replace_reg(op, reg_map);
            }
            Instruction::Cdq(_)
            | Instruction::Pop(_)
            | Instruction::Jmp(_)
            | Instruction::JmpCC(_, _)
            | Instruction::Label(_)
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
