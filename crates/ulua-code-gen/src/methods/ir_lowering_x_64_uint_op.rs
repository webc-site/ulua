use crate::records::{ir_lowering_x_64::IrLoweringX64, ir_op::IrOp};

impl IrLoweringX64 {
  pub fn uint_op(&self, op: IrOp) -> u32 {
    unsafe { &mut *self.function }.uint_op(op)
  }
}
