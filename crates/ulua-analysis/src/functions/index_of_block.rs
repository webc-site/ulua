use crate::{records::control_flow_graph::ControlFlowGraph, type_aliases::block_id::BlockId};

pub fn index_of_block(cfg: &ControlFlowGraph, block: BlockId) -> usize {
  for (i, b) in cfg.blocks.iter().enumerate() {
    if *b == block {
      return i;
    }
  }
  0
}
