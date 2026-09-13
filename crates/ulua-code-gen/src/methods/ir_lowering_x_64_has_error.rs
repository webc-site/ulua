use crate::records::ir_lowering_x_64::IrLoweringX64;

impl IrLoweringX64 {
  pub fn has_error(&self) -> bool {
    self.regs.max_used_slot > Self::k_spill_slots() + Self::k_extra_spill_slots()
  }

  const fn k_spill_slots() -> u32 {
    13
  }

  const fn k_extra_spill_slots() -> u32 {
    64
  }
}
