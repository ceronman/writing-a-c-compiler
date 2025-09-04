use std::collections::{HashSet, VecDeque};
use crate::optimization::cfg::{NodeId, TackyCfg};

pub fn remove_unreachable_blocks(mut cfg: TackyCfg) -> TackyCfg {
    let mut dead_nodes: HashSet<NodeId> = HashSet::from_iter(cfg.all_ids());
    let mut queue = VecDeque::with_capacity(cfg.size());
    queue.push_front(cfg.entry_id());
    while let Some(node_id) = queue.pop_front() {
        dead_nodes.remove(&node_id);
        queue.extend(cfg.get_node(node_id).successors.iter());
    }
    for node_id in dead_nodes {
        cfg.remove_node(node_id);
    }

    cfg
}