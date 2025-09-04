use crate::symbol::Symbol;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::tacky;

trait GenericInstruction: Clone + Debug {
    fn kind(&self) -> InstructionKind;
}

#[derive(Clone)]
enum InstructionKind {
    Return,
    Jump { label: Symbol },
    ConditionalJump { label: Symbol },
    Label(Symbol),
    Other
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
struct NodeId(usize);

impl NodeId {
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

struct Node<T: GenericInstruction> {
    id: NodeId,
    instructions: Vec<T>,
}

impl<T: GenericInstruction> Node<T> {
    fn new(id: NodeId, instructions: Vec<T>) -> Self {
        Self {
            id,
            instructions,
        }
    }
}

pub struct Cfg<T: GenericInstruction> {
    nodes: Vec<Node<T>>,
    entry_id: NodeId,
    exit_id: NodeId,
    by_label: HashMap<Symbol, NodeId>,
    successors: HashMap<NodeId, Vec<NodeId>>,
    predecessors: HashMap<NodeId, Vec<NodeId>>,
}

impl<T: GenericInstruction> Cfg<T> {
    fn build(&mut self, instructions: &[T]) {
        self.nodes = Vec::new();
        self.entry_id = self.add_node(vec![]);
        let mut current_block = Vec::new();
        for instruction in instructions {
            match &instruction.kind() {
                InstructionKind::Label(label) => {
                    self.by_label.insert(label.clone(), NodeId(self.nodes.len()));
                    if !current_block.is_empty() {
                        self.add_node(current_block);
                    }
                    current_block = vec![instruction.clone()]
                }
                InstructionKind::Jump { .. } | InstructionKind::ConditionalJump { .. } | InstructionKind::Return => {
                    current_block.push(instruction.clone());
                    self.add_node(current_block);
                    current_block = vec![];
                }
                InstructionKind::Other => {
                    current_block.push(instruction.clone());
                }
            }
        }
        if !current_block.is_empty() {
            self.add_node(current_block);
        }
        self.exit_id = self.add_node(vec![]);
        self.add_edges();
    }

    fn add_node(&mut self, instructions: Vec<T>) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Node::new(id, instructions));
        id
    }

    fn add_edges(&mut self) {
        self.add_edge(self.entry_id, self.entry_id.next());
        for i in 0..self.nodes.len() {
            let node_id = self.nodes[i].id;
            if node_id == self.entry_id || node_id == self.exit_id {
                continue;
            }
            let next_id = node_id.next();
            let kind = self.nodes[i].instructions.last().unwrap().kind();
            match kind {
                InstructionKind::Return => self.add_edge(node_id, self.exit_id),
                InstructionKind::Jump { label } => {
                    let target_id = self.get_by_label(&label);
                    self.add_edge(node_id, target_id);
                }
                InstructionKind::ConditionalJump { label } => {
                    let target_id = self.get_by_label(&label);
                    self.add_edge(node_id, target_id);
                    self.add_edge(node_id, next_id);
                }
                _ => self.add_edge(node_id, next_id),
            }
        }
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.successors.entry(from).or_default().push(to);
        self.predecessors.entry(to).or_default().push(from);
    }

    fn get_by_label(&self, label: &Symbol) -> NodeId {
        *self.by_label.get(label).unwrap()
    }
}

impl GenericInstruction for tacky::Instruction {
    fn kind(&self) -> InstructionKind {
        match self {
            tacky::Instruction::Return(_) => InstructionKind::Return,
            tacky::Instruction::Jump { target } => InstructionKind::Jump { label: target.clone() },
            tacky::Instruction::JumpIfZero { target, .. }
            | tacky::Instruction::JumpIfNotZero { target, .. } => InstructionKind::ConditionalJump { label: target.clone() },
            tacky::Instruction::Label(label) => InstructionKind::Label(label.clone()),
            _ => InstructionKind::Other,
        }
    }
}

pub fn tacky_to_cfg(instructions: &[tacky::Instruction]) -> Cfg<tacky::Instruction> {
    let mut cfg = Cfg {
        nodes: Vec::new(),
        entry_id: NodeId(0),
        exit_id: NodeId(0),
        by_label: HashMap::new(),
        successors: HashMap::new(),
        predecessors: HashMap::new(),
    };
    cfg.build(instructions);
    cfg
}

impl<T: GenericInstruction> Debug for Cfg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.nodes.len() {
            writeln!(f, "  Node {}:", i)?;
            for ins in self.nodes[i].instructions.iter() {
                writeln!(f, "    {:?}", ins)?;
            }
            let successors = self.successors.get(&NodeId(i)).unwrap_or(&vec![]).iter().map(|&id| id.0.to_string()).collect::<Vec<_>>().join(", ");
            if !successors.is_empty() {
                writeln!(f, "    Successors: {successors}")?;
            }
            let predecesors = self.predecessors.get(&NodeId(i)).unwrap_or(&vec![]).iter().map(|&id| id.0.to_string()).collect::<Vec<_>>().join(", ");
            if !predecesors.is_empty() {
                writeln!(f, "    Predecessors: {predecesors}")?;
            }
        }
        Ok(())
    }
}

