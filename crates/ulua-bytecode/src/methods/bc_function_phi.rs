use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_op::BcOp, bc_phi::BcPhi, bc_ref::BcRef},
};

impl BcFunction {
  pub fn phi<'a>(&'a self, op: BcOp) -> BcRef<'a, BcPhi> {
    LUAU_ASSERT!(op.kind == BcOpKind::Phi);
    BcRef {
      vec: &self.phis,
      op,
    }
  }
}
