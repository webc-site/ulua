use crate::records::{ir_block::IrBlock, ir_lowering_a_64::IrLoweringA64};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_jump_or_fallthrough(&mut self, target: &mut IrBlock, next: &IrBlock) {
    if !self.ir_lowering_a_64_is_fallthrough_block(target, next) {
      unsafe {
        (*self.build).bl(&mut target.label);
      }
    }
  }
}
