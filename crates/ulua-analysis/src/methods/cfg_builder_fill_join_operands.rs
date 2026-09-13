//! Source: `Analysis/src/ControlFlowGraph.cpp:488-497` (hand-ported)
//! C++ `void CFGBuilder::fillJoinOperands(Block* block, Join* j)`.
use alloc::vec::Vec;

use crate::{
  records::{block::Block, cfg_builder::CfgBuilder, join::Join},
  type_aliases::block_id::BlockId,
};
impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `block、`j` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn fill_join_operands(&mut self, block: *mut Block, j: *mut Join) {
    unsafe {
      // C++:
      //   for (BlockId pred : block->getPredecessors()) {
      //       auto def = readVariable(pred, j->definition->sym);
      //       j->operands.emplace_back(def);
      //   }
      //   trimTrivialJoin(j);
      // Snapshot predecessors: readVariable recurses and may mutate `block`.
      let preds: Vec<BlockId> = (*block).get_predecessors().clone();
      let sym = (*(*j).definition).sym.clone();
      for pred in preds {
        let def = self.read_variable(pred, sym.clone());
        (*j).operands.push(def);
      }
      self.trim_trivial_join(j);
    }
  }
}
