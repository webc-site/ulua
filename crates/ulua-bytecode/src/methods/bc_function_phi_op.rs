use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_op::BcOp, bc_phi::BcPhi},
};

impl BcFunction {
  pub fn phi_op(&mut self, op: BcOp) -> &mut BcPhi {
    LUAU_ASSERT!(op.kind == BcOpKind::Phi);
    &mut self.phis[op.index as usize]
  }
}
