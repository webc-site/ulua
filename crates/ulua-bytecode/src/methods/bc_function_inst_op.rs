use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_inst::BcInst, bc_op::BcOp},
};

impl BcFunction {
  pub fn inst_op(&mut self, op: BcOp) -> &mut BcInst {
    LUAU_ASSERT!(op.kind == BcOpKind::Inst);
    &mut self.instructions[op.index as usize]
  }
}
