use crate::records::{ir_block::IrBlock, ir_lowering_a_64::IrLoweringA64};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_is_fallthrough_block(&self, target: &IrBlock, next: &IrBlock) -> bool {
    target.start == next.start
  }
}
