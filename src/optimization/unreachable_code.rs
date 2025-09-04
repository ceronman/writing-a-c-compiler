use crate::optimization::cfg::{Cfg, GenericInstruction, InstructionKind, NodeId, TackyCfg};
use crate::tacky;
use std::collections::{HashSet, VecDeque};

pub fn remove_unreachable_code(instructions: &[tacky::Instruction]) -> Vec<tacky::Instruction> {
    let cfg = Cfg::new(instructions);
    let cfg = remove_unreachable_blocks(cfg);
    let cfg = remove_useless_jumps(cfg);
    let cfg = remove_useless_labels(cfg);
    let cfg = remove_empty_blocks(cfg);
    cfg.dump()
}

pub fn remove_unreachable_blocks(mut cfg: TackyCfg) -> TackyCfg {
    let mut dead_nodes: HashSet<NodeId> = HashSet::from_iter(cfg.all_ids());
    let mut queue = VecDeque::with_capacity(cfg.size());
    queue.push_front(cfg.entry_id());
    while let Some(node_id) = queue.pop_front() {
        if dead_nodes.remove(&node_id) {
            queue.extend(cfg.get_node(node_id).successors.iter());
        }
    }
    for node_id in dead_nodes {
        cfg.remove_node(node_id);
    }

    cfg
}

pub fn remove_useless_jumps(mut cfg: TackyCfg) -> TackyCfg {
    let ids = Vec::from_iter(cfg.all_ids());
    let mut ids = ids.into_iter().peekable();
    while let Some(node_id) = ids.next() {
        let node = cfg.get_node(node_id);

        if let Some(last_instruction) = node.instructions.last().map(|i| i.kind())
            && let InstructionKind::Jump { .. } | InstructionKind::ConditionalJump { .. } =
                last_instruction
            && let Some(next_node_id) = ids.peek()
            && node.successors.iter().all(|s| s == next_node_id)
        {
            cfg.get_node_mut(node_id).instructions.pop();
        }
    }

    cfg
}

pub fn remove_useless_labels(mut cfg: TackyCfg) -> TackyCfg {
    let ids = Vec::from_iter(cfg.all_ids());
    let mut ids = ids.into_iter().rev().peekable();
    while let Some(node_id) = ids.next() {
        let node = cfg.get_node(node_id);

        if let Some(first_instruction) = node.instructions.first().map(|i| i.kind())
            && let InstructionKind::Label(..) = first_instruction
            && let Some(prev_node_id) = ids.peek()
            && node.predecessors.iter().all(|s| s == prev_node_id)
        {
            cfg.get_node_mut(node_id).instructions.remove(0);
        }
    }
    cfg
}

pub fn remove_empty_blocks(mut cfg: TackyCfg) -> TackyCfg {
    let ids = Vec::from_iter(cfg.all_ids());
    for node_id in ids {
        let node = cfg.get_node(node_id);
        if node.instructions.is_empty() {
            cfg.remove_node(node_id);
        }
    }
    cfg
}
