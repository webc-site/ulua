use alloc::vec::Vec;

use crate::records::{block::Block, cfg_builder::CfgBuilder};
impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `b` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  /// `void CFGBuilder::seal(Block* b)`. Reference: `ControlFlowGraph.cpp`.
  pub(crate) fn seal(&mut self, b: *mut Block) {
    {
      // C++:
      //   auto joinsToFill = incompleteJoins.find(b);
      //   if (joinsToFill != nullptr)
      //       for (auto j : *joinsToFill) fillJoinOperands(b, j);
      //   sealedBlocks.insert(b);
      if let Some(joins) = self.incomplete_joins.find(&b) {
        let joins: Vec<_> = joins.iter().copied().collect();
        for j in joins {
          unsafe { self.fill_join_operands(b, j) };
        }
      }
      self.sealed_blocks.insert(b);
    }
  }
}
