use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_inst::BcInst, bc_op::BcOp, bc_ref::BcRef},
};

impl BcFunction {
  pub fn inst<'a>(&'a self, op: BcOp) -> BcRef<'a, BcInst> {
    LUAU_ASSERT!(op.kind == BcOpKind::Inst);
    BcRef {
      vec: &self.instructions,
      op,
    }
  }
}
