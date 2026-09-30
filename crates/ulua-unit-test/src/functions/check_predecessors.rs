use ulua_analysis::records::{block::Block, control_flow_graph::ControlFlowGraph};

use crate::functions::block_index::block_index;

/// 断言 `block` 的前驱块索引与 `expected` 一致（块顺序按 CFG 发射序）。
pub fn check_predecessors(cfg: &ControlFlowGraph, block: &Block, expected: &[usize]) {
  let preds = block.get_predecessors();
  assert_eq!(expected.len(), preds.len());

  for (pred, expected_index) in preds.iter().zip(expected.iter()) {
    assert_eq!(*expected_index, block_index(cfg, *pred));
  }
}
