use crate::records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_tag_op(&self, op: IrOp) -> u8 {
    unsafe { (*self.function).tag_op(op) }
  }
}
