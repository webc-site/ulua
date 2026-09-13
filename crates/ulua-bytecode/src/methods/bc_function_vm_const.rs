use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{
    bc_function::{BcFunction, VmConst},
    bc_op::BcOp,
    bc_ref::BcRef,
  },
};

impl BcFunction {
  pub fn vm_const<'a>(&'a self, op: BcOp) -> BcRef<'a, VmConst> {
    LUAU_ASSERT!(op.kind == BcOpKind::VmConst);
    BcRef {
      vec: &self.constants,
      op,
    }
  }
}
