use crate::records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_import_op(&self, op: IrOp) -> u32 {
    unsafe { (*self.function).import_op(op) }
  }
}
