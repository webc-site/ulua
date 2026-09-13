use crate::records::{ir_block::IrBlock, ir_lowering_x_64::IrLoweringX64};

impl IrLoweringX64 {
  pub fn jump_or_fallthrough(&mut self, target: &mut IrBlock, next: &IrBlock) {
    if !self.is_fallthrough_block(target, next) {
      unsafe {
        (*self.build).jmp_label(&mut target.label);
      }
    }
  }
}
