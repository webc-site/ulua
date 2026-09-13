use crate::records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_int_64_op(&self, op: IrOp) -> i64 {
    unsafe { (*self.function).int64_op(op) }
  }
}
