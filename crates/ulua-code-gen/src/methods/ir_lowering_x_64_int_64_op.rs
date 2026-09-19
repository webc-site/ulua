use crate::records::{ir_lowering_x_64::IrLoweringX64, ir_op::IrOp};

impl IrLoweringX64 {
  pub fn int64_op(&self, op: IrOp) -> i64 {
    unsafe {
      let self_mut = self as *const Self as *mut Self;
      (*(*self_mut).function).int64_op(op)
    }
  }
}
