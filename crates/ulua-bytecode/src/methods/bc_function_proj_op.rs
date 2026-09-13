use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_op::BcOp, bc_proj::BcProj},
};

impl BcFunction {
  pub fn proj_op(&mut self, op: BcOp) -> &mut BcProj {
    LUAU_ASSERT!(op.kind == BcOpKind::Proj);
    &mut self.projections[op.index as usize]
  }
}
