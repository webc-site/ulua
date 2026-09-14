use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  pub fn map_proto_op(&self, target_proto_op: BcOp) -> BcOp {
    LUAU_ASSERT!(target_proto_op.kind == BcOpKind::VmProto);
    BcOp::bc_op_bc_op_kind_u32(
      BcOpKind::VmProto,
      self.caller_proto_size_before_inline + target_proto_op.index,
    )
  }
}
