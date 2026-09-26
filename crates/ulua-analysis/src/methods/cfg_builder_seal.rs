use alloc::vec::Vec;

use crate::{records::cfg_builder::CfgBuilder, type_aliases::block_id::BlockId};

impl CfgBuilder {
  /// `void CFGBuilder::seal(Block* b)`. Reference: `ControlFlowGraph.cpp`.
  pub(crate) fn seal(&mut self, b: BlockId) {
    // C++:
    //   auto joinsToFill = incompleteJoins.find(b);
    //   if (joinsToFill != nullptr)
    //       for (auto j : *joinsToFill) fillJoinOperands(b, j);
    //   sealedBlocks.insert(b);
    if let Some(joins) = self.incomplete_joins.find(&b) {
      let joins: Vec<_> = joins.iter().copied().collect();
      for j in joins {
        self.fill_join_operands(b, j);
      }
    }
    self.sealed_blocks.insert(b);
  }
}
