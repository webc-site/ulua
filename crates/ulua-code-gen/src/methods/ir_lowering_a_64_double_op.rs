use crate::records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_double_op(&self, op: IrOp) -> f64 {
    unsafe { (*self.function).double_op(op) }
  }
}
