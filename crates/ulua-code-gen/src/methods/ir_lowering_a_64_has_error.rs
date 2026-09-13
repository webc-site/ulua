use crate::records::ir_lowering_a_64::IrLoweringA64;

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_has_error(&self) -> bool {
    self.error || self.regs.error
  }
}
