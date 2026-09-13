use crate::records::{emit_common_a_64::K_SPILL_SLOTS, ir_reg_alloc_a_64::IrRegAllocA64};

impl IrRegAllocA64 {
  pub fn is_extra_spill_slot(&self, slot: u32) -> bool {
    slot >= K_SPILL_SLOTS
  }
}
