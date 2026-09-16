use crate::records::{ir_const::IrConst, ir_lowering_x_64::IrLoweringX64, ir_op::IrOp};

impl IrLoweringX64 {
  pub fn ir_lowering_x_64_const_op(&self, op: IrOp) -> IrConst {
    unsafe { &mut *self.function }.const_op(op)
  }
}
