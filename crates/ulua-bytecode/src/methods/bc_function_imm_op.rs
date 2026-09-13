use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_imm::BcImm, bc_op::BcOp},
};

impl BcFunction {
  pub fn imm_op(&mut self, op: BcOp) -> &mut BcImm {
    LUAU_ASSERT!(op.kind == BcOpKind::Imm);
    &mut self.immediates[op.index as usize]
  }
}
