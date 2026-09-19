use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_inst_helper::BcInstHelper, bc_op::BcOp},
};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::setVMConst(inputIdx, cid)`：把第 `inputIdx` 个输入换成常量。
  pub fn set_vm_const(&mut self, input_idx: u32, cid: u32) {
    LUAU_ASSERT!(cid < self.graph.constants.len() as u32);
    self.set_bc_op(
      input_idx,
      BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmConst, cid),
    );
  }
}
