use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_block::BcBlock, bc_inst_helper::BcInstHelper, bc_ref::BcRef},
};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::getBlock(inputIdx)`：第 `inputIdx` 个输入对应的块（只读视图）。
  pub fn get_block(&mut self, input_idx: u32) -> BcRef<'_, BcBlock> {
    let block_op = self.get_bc_op(input_idx);
    LUAU_ASSERT!(block_op.kind == BcOpKind::Block);
    self.graph.block(block_op)
  }
}
