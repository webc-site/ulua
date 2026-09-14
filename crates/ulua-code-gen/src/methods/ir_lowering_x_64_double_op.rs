use crate::records::{ir_lowering_x_64::IrLoweringX64, ir_op::IrOp};

impl IrLoweringX64 {
  pub fn double_op(&self, op: IrOp) -> f64 {
    unsafe { &mut *self.function }.double_op(op)
  }
}
