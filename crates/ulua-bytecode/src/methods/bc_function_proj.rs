use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_op::BcOp, bc_proj::BcProj, bc_ref::BcRef},
};

impl BcFunction {
  pub fn proj<'a>(&'a self, op: BcOp) -> BcRef<'a, BcProj> {
    LUAU_ASSERT!(op.kind == BcOpKind::Proj);
    BcRef {
      vec: &self.projections,
      op,
    }
  }
}
