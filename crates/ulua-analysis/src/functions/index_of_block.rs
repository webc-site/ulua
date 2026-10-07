use crate::{records::control_flow_graph::ControlFlowGraph, type_aliases::block_id::BlockId};

pub fn index_of_block(cfg: &ControlFlowGraph, block: BlockId) -> usize {
  cfg.blocks.iter().position(|&b| b == block).unwrap_or(0)
}
