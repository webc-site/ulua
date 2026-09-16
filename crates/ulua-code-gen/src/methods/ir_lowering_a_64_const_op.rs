use crate::records::{ir_const::IrConst, ir_lowering_a_64::IrLoweringA64, ir_op::IrOp};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_const_op(&self, op: IrOp) -> IrConst {
    unsafe { (*self.function).const_op(op) }
  }
}
