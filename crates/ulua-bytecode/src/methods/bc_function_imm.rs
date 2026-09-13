use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_imm::BcImm, bc_op::BcOp, bc_ref::BcRef},
};

impl BcFunction {
  pub fn imm<'a>(&'a self, op: BcOp) -> BcRef<'a, BcImm> {
    LUAU_ASSERT!(op.kind == BcOpKind::Imm);
    BcRef {
      vec: &self.immediates,
      op,
    }
  }
}
