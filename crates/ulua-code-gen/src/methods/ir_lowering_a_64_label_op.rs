use crate::records::{ir_lowering_a_64::IrLoweringA64, ir_op::IrOp, label::Label};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_label_op(&mut self, op: IrOp) -> &mut Label {
    unsafe { &mut *core::ptr::addr_of_mut!((*self.ir_lowering_a_64_block_op(op)).label) }
  }
}
