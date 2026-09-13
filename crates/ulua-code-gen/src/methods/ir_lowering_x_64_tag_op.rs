use crate::records::{ir_lowering_x_64::IrLoweringX64, ir_op::IrOp};

impl IrLoweringX64 {
  pub fn tag_op(&self, op: IrOp) -> u8 {
    unsafe { &mut *self.function }.tag_op(op)
  }
}
