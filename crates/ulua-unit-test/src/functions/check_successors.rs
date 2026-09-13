use ulua_analysis::records::{block::Block, control_flow_graph::ControlFlowGraph};

use crate::functions::block_index::block_index;
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check_successors(cfg: &ControlFlowGraph, block: *mut Block, expected: &[usize]) {
  let succs = unsafe { (*block).get_successors() };
  assert_eq!(expected.len(), succs.len());

  for (succ, expected_index) in succs.iter().zip(expected.iter()) {
    assert_eq!(*expected_index, block_index(cfg, *succ));
  }
}
