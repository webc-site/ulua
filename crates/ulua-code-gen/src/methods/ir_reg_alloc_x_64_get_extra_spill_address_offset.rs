use crate::records::{emit_common_x_64::K_SPILL_SLOTS, ir_reg_alloc_x_64::IrRegAllocX64};

impl IrRegAllocX64 {
  pub fn get_extra_spill_address_offset(&self, slot: u32) -> i32 {
    debug_assert!(self.is_extra_spill_slot(slot));
    ((slot - K_SPILL_SLOTS * 2) * 4) as i32
  }
}
