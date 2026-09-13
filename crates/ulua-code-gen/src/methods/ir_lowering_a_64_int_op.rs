use crate::records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_int_op(&self, op: IrOp) -> i32 {
    // 转发到 IrFunction 的常量取值(原为自递归存根,运行即栈溢出)
    unsafe { (*self.function).int_op(op) }
  }
}
