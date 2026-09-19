use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_block::BcBlock, bc_function::BcFunction, bc_op::BcOp, bc_ref::BcRef},
};

impl BcFunction {
  pub fn block<'a>(&'a self, op: BcOp) -> BcRef<'a, BcBlock> {
    LUAU_ASSERT!(op.kind == BcOpKind::Block);
    BcRef {
      vec: &self.blocks,
      op,
    }
  }
}
