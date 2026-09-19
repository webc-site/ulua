use ulua_analysis::records::{block::Block, control_flow_graph::ControlFlowGraph};
pub fn block_index(cfg: &ControlFlowGraph, b: *mut Block) -> usize {
  for (i, block) in cfg.blocks.iter().enumerate() {
    if *block == b {
      return i;
    }
  }

  panic!("block was not found in CFG");
}
