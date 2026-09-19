use crate::records::{bc_inst_helper::BcInstHelper, bc_op::BcOp};

impl BcInstHelper<'_> {
  /// cpp `BcInstHelper::op()`：本 helper 指向的指令的 `BcOp` 句柄。
  pub fn op(&self) -> BcOp {
    self.inst
  }
}
