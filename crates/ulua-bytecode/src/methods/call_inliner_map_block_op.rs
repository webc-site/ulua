use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  pub fn map_block_op(&mut self, target_block: BcOp) -> BcOp {
    LUAU_ASSERT!(target_block.kind == BcOpKind::Block);
    BcOp::bc_op_bc_op_kind_u32(
      BcOpKind::Block,
      self.caller_blocks_size_before_inline + target_block.index,
    )
  }
}
