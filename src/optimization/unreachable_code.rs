use crate::optimization::cfg::{GenericInstruction, InstructionKind, NodeId, TackyCfg};
use std::collections::{HashSet, VecDeque};

pub fn remove_unreachable_code(cfg: &mut TackyCfg, trace: bool) {
    if trace {
        println!("=======================");
        println!("Unreachable code");
        println!("=======================");
        println!("INITIAL\n {:#?}", cfg);
    }
    cfg.remove_unreachable_blocks();
    if trace {
        println!("UNREACHABLE BLOCKS\n {:#?}", cfg);
    }
    cfg.remove_useless_jumps();
    if trace {
        println!("USELESS JUMPS\n {:#?}", cfg);
    }
    cfg.remove_useless_labels();
    if trace {
        println!("USELESS LABELS\n {:#?}", cfg);
    }
    cfg.remove_empty_blocks();
    if trace {
        println!("EMPTY BLOCKS\n {:#?}", cfg);
    }
}

impl TackyCfg {
    fn remove_unreachable_blocks(&mut self) {
        let mut dead_nodes: HashSet<NodeId> = HashSet::from_iter(self.all_ids());
        let mut queue = VecDeque::with_capacity(self.size());
        queue.push_front(self.entry_id());
        while let Some(node_id) = queue.pop_front() {
            if dead_nodes.remove(&node_id) {
                queue.extend(self.get_node(node_id).successors.iter());
            }
        }
        for node_id in dead_nodes {
            self.remove_node(node_id);
        }
    }

    fn remove_useless_jumps(&mut self) {
        let ids = Vec::from_iter(self.all_ids());
        let mut ids = ids.into_iter().peekable();
        while let Some(node_id) = ids.next() {
            let node = self.get_node(node_id);

            if let Some(last_instruction) = node.instructions.last().map(|i| i.kind())
                && let InstructionKind::Jump { .. } | InstructionKind::ConditionalJump { .. } =
                    last_instruction
                && let Some(next_node_id) = ids.peek()
                && node.successors.iter().all(|s| s == next_node_id)
            {
                self.get_node_mut(node_id).instructions.pop();
            }
        }
    }

    fn remove_useless_labels(&mut self) {
        let ids = Vec::from_iter(self.all_ids());
        let mut ids = ids.into_iter().rev().peekable();
        while let Some(node_id) = ids.next() {
            let node = self.get_node(node_id);

            if let Some(first_instruction) = node.instructions.first().map(|i| i.kind())
                && let InstructionKind::Label(..) = first_instruction
                && let Some(prev_node_id) = ids.peek()
                && node.predecessors.iter().all(|s| s == prev_node_id)
            {
                self.get_node_mut(node_id).instructions.remove(0);
            }
        }
    }

    fn remove_empty_blocks(&mut self) {
        let ids = Vec::from_iter(self.all_ids());
        for node_id in ids {
            if node_id == self.entry_id() || node_id == self.exit_id() {
                continue;
            }
            let node = self.get_node(node_id);
            if node.instructions.is_empty() {
                self.remove_node(node_id);
            }
        }
    }
}
