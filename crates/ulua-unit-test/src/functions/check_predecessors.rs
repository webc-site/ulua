use ulua_analysis::records::{block::Block, control_flow_graph::ControlFlowGraph};

use crate::functions::block_index::block_index;
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check_predecessors(cfg: &ControlFlowGraph, block: *mut Block, expected: &[usize]) {
  let preds = unsafe { (*block).get_predecessors() };
  assert_eq!(expected.len(), preds.len());

  for (pred, expected_index) in preds.iter().zip(expected.iter()) {
    assert_eq!(*expected_index, block_index(cfg, *pred));
  }
}
