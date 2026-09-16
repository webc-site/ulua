use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_block::BcBlock, bc_op::BcOp},
};

impl BcBlock {
  pub fn append_instruction(&mut self, inst: BcOp) {
    LUAU_ASSERT!(inst.kind == BcOpKind::Inst);
    self.ops.push_back(inst);
  }
}
