use crate::records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_uint_op(&self, op: IrOp) -> u32 {
    // 转发到 IrFunction 的常量取值(原为自递归存根,运行即栈溢出)
    unsafe { (*self.function).uint_op(op) }
  }
}
