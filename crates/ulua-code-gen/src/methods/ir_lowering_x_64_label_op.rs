use crate::records::{ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, label::Label};

impl IrLoweringX64 {
  /// # Safety
  /// 调用方须保证独占访问返回的 Label
  pub unsafe fn label_op(&self, op: IrOp) -> *mut Label {
    unsafe { core::ptr::addr_of_mut!((*self.block_op(op)).label) }
  }
}
