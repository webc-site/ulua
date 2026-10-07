use ulua_analysis::records::{block::Block, control_flow_graph::ControlFlowGraph};

use crate::functions::block_index::block_index;

/// 断言 `block` 的后继块索引与 `expected` 一致（块顺序按 CFG 发射序）。
pub fn check_successors(cfg: &ControlFlowGraph, block: &Block, expected: &[usize]) {
  let succs = block.get_successors();
  assert_eq!(expected.len(), succs.len());

  for (succ, expected_index) in succs.iter().zip(expected.iter()) {
    assert_eq!(*expected_index, block_index(cfg, *succ));
  }
}
