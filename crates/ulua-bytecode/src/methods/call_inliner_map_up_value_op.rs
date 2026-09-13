use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  pub fn map_up_value_op(&mut self, target_upval: BcOp) -> BcOp {
    LUAU_ASSERT!(target_upval.kind == BcOpKind::VmUpvalue);
    BcOp::bc_op_bc_op_kind_u32(
      BcOpKind::VmUpvalue,
      self.caller_up_val_size_before_inline as u32 + target_upval.index,
    )
  }
}
