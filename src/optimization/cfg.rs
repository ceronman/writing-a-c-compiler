use crate::symbol::Symbol;
use crate::tacky;
use crate::tacky::pretty::pp_instruction;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub trait GenericInstruction: Clone {
    fn kind(&self) -> InstructionKind;
    fn pp(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

#[derive(Clone)]
pub enum InstructionKind {
    Return,
    Jump { label: Symbol },
    ConditionalJump { label: Symbol },
    Label(Symbol),
    Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
pub struct NodeId(usize);

impl NodeId {
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

pub struct GenericNode<T: GenericInstruction> {
    pub id: NodeId,
    pub instructions: Vec<T>,
    pub successors: Vec<NodeId>,
    pub predecessors: Vec<NodeId>,
}

impl<T: GenericInstruction> GenericNode<T> {
    fn new(id: NodeId, instructions: Vec<T>) -> Self {
        Self {
            id,
            instructions,
            successors: vec![],
            predecessors: vec![],
        }
    }
}

pub struct GenericCfg<T: GenericInstruction> {
    nodes: Vec<GenericNode<T>>,
    removed: Vec<NodeId>,
    entry_id: NodeId,
    exit_id: NodeId,
    by_label: HashMap<Symbol, NodeId>,
}

impl<T: GenericInstruction> GenericCfg<T> {
    pub fn new(instructions: &[T]) -> GenericCfg<T> {
        let mut cfg = GenericCfg {
            nodes: Vec::new(),
            removed: vec![],
            entry_id: NodeId(0),
            exit_id: NodeId(0),
            by_label: HashMap::new(),
        };
        cfg.build(instructions);
        cfg
    }

    fn build(&mut self, instructions: &[T]) {
        self.nodes = Vec::new();
        self.entry_id = self.add_node(vec![]);
        let mut current_block = Vec::new();
        for instruction in instructions {
            match &instruction.kind() {
                InstructionKind::Label(label) => {
                    if !current_block.is_empty() {
                        self.add_node(current_block);
                    }
                    self.by_label
                        .insert(label.clone(), NodeId(self.nodes.len()));
                    current_block = vec![instruction.clone()]
                }
                InstructionKind::Jump { .. }
                | InstructionKind::ConditionalJump { .. }
                | InstructionKind::Return => {
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
        self.nodes.push(GenericNode::new(id, instructions));
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
        self.nodes[from.0].successors.push(to);
        self.nodes[to.0].predecessors.push(from);
    }

    fn get_by_label(&self, label: &Symbol) -> NodeId {
        *self.by_label.get(label).unwrap()
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn all_ids(&self) -> impl Iterator<Item = NodeId> {
        self.nodes
            .iter()
            .map(|node| node.id)
            .filter(|id| !self.removed.contains(id))
    }

    pub fn nodes_mut(&mut self) -> impl Iterator<Item = &mut GenericNode<T>> {
        self.nodes
            .iter_mut()
            .filter(|node| !self.removed.contains(&node.id))
    }

    pub fn entry_id(&self) -> NodeId {
        self.entry_id
    }

    pub fn exit_id(&self) -> NodeId {
        self.exit_id
    }

    pub fn get_node(&self, id: NodeId) -> &GenericNode<T> {
        assert!(!self.removed.contains(&id));
        &self.nodes[id.0]
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> &mut GenericNode<T> {
        assert!(!self.removed.contains(&id));
        &mut self.nodes[id.0]
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.removed.push(id);
        let predecessors = self.nodes[id.0].predecessors.clone();
        let successors = self.nodes[id.0].successors.clone();
        for pred_id in predecessors.iter().copied() {
            let succ = &mut self.nodes[pred_id.0].successors;
            succ.retain(|&node_id| node_id != id);
            succ.extend(successors.iter());
            succ.sort();
            succ.dedup();
        }

        for succ_id in successors.iter().copied() {
            let pred = &mut self.nodes[succ_id.0].predecessors;
            pred.retain(|&node_id| node_id != id);
            pred.extend(predecessors.iter());
            pred.sort();
            pred.dedup();
        }
        self.nodes[id.0].predecessors.clear();
        self.nodes[id.0].successors.clear()
    }

    pub fn dump(&self) -> Vec<T> {
        let mut result = Vec::new();
        for node_id in self.all_ids() {
            result.extend(self.get_node(node_id).instructions.iter().cloned());
        }
        result
    }
}

#[derive(Debug)]
pub struct Annotation<T> {
    block: HashMap<NodeId, T>,
    instructions: HashMap<(NodeId, usize), T>,
}

impl<T> Annotation<T> {
    pub fn empty() -> Self {
        Self {
            block: HashMap::new(),
            instructions: HashMap::new(),
        }
    }

    pub fn annotate_block(&mut self, node_id: NodeId, value: T) {
        self.block.insert(node_id, value);
    }

    pub fn annotate_instruction(&mut self, node_id: NodeId, instruction_index: usize, value: T) {
        self.instructions
            .insert((node_id, instruction_index), value);
    }

    pub fn get_block_annotation(&self, node_id: &NodeId) -> &T {
        self.block.get(node_id).expect("Block not annotated")
    }

    pub fn get_instruction_annotation(&self, node_id: NodeId, instruction_index: usize) -> &T {
        self.instructions
            .get(&(node_id, instruction_index))
            .expect("Instruction not annotated")
    }
}

impl<T: GenericInstruction> Debug for GenericCfg<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in self.nodes.iter() {
            match node.id {
                id if id == self.entry_id() => writeln!(f, "  ##ENTRY: ")?,
                id if id == self.exit_id() => writeln!(f, "  ##EXIT: ")?,
                id if self.removed.contains(&id) => writeln!(f, "  ##[REMOVED {}]", node.id.0)?,
                _ => writeln!(f, "  ##Node {}:", node.id.0)?,
            };

            for ins in node.instructions.iter() {
                ins.pp(f)?;
            }
            let successors = node
                .successors
                .iter()
                .map(|&id| id.0.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if !successors.is_empty() {
                writeln!(f, "    Successors: {successors}")?;
            }
            let predecessors = node
                .predecessors
                .iter()
                .map(|&id| id.0.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if !predecessors.is_empty() {
                writeln!(f, "    Predecessors: {predecessors}")?;
            }
        }
        Ok(())
    }
}
