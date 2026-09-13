use crate::records::{bc_inst_helper::BcInstHelper, bc_op::BcOp};

impl BcInstHelper<'_> {
  pub fn op(&self) -> BcOp {
    self.inst.op
  }
}
