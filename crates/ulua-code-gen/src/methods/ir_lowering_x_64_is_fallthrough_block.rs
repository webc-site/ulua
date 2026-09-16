use crate::records::{ir_block::IrBlock, ir_lowering_x_64::IrLoweringX64};

impl IrLoweringX64 {
  pub fn is_fallthrough_block(&self, target: &IrBlock, next: &IrBlock) -> bool {
    target.start == next.start
  }
}
