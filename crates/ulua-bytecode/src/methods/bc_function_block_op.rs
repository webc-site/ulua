use ulua_common::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_block::BcBlock, bc_function::BcFunction, bc_op::BcOp},
};

impl BcFunction {
  pub fn block_op(&mut self, op: BcOp) -> &mut BcBlock {
    LUAU_ASSERT!(op.kind == BcOpKind::Block);
    &mut self.blocks[op.index as usize]
  }
}
