use crate::records::{ir_block::IrBlock, ir_lowering_a_64::IrLoweringA64, ir_op::IrOp};

impl IrLoweringA64 {
  /// # Safety
  /// 调用方须保证对 function 的独占访问(lowering 阶段语义上持有)
  pub unsafe fn ir_lowering_a_64_block_op(&self, op: IrOp) -> *mut IrBlock {
    // 内部可变性: lowering 阶段通过裸指针写 function,返回裸指针避免 &self -> &mut(mut_from_ref)
    unsafe {
      let self_mut = self as *const Self as *mut Self;
      (*(*self_mut).function).block_op(op)
    }
  }
}
