use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{bc_block_edge::BcBlockEdge, bc_op::BcOp, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  /// cpp `addSuccessor(BcRef<BcBlock> from, BcRef<BcBlock> to, Kind)`。
  ///
  /// cpp 靠 `BcRef::operator->` 从共享借用上取裸指针来写 `successors`/`predecessors`；
  /// Rust 侧可变访问必须由持有图的一方（`self.caller`）提供，因此入参改为纯 `BcOp` 句柄
  /// （`BcRef` 只保留只读语义）。
  pub fn add_successor(&mut self, from_op: BcOp, to_op: BcOp, kind: BcBlockEdgeKind) {
    LUAU_ASSERT!(
      kind != BcBlockEdgeKind::Fallthrough
        || (!self.has_edge(
          &self.caller.block(from_op).operator_deref().successors,
          BcBlockEdgeKind::Fallthrough
        ) && !self.has_edge(
          &self.caller.block(to_op).operator_deref().predecessors,
          BcBlockEdgeKind::Fallthrough
        ))
    );

    self
      .caller
      .block_op(from_op)
      .successors
      .push_back(BcBlockEdge {
        kind,
        target: to_op,
      });

    self
      .caller
      .block_op(to_op)
      .predecessors
      .push_back(BcBlockEdge {
        kind,
        target: from_op,
      });
  }
}
