use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{emit_common_a_64::K_SPILL_SLOTS, ir_reg_alloc_a_64::IrRegAllocA64},
};

impl IrRegAllocA64 {
  pub fn get_extra_spill_address_offset(&self, slot: u32) -> i32 {
    CODEGEN_ASSERT!(
      self.is_extra_spill_slot(slot),
      b"slot is not an extra spill slot\0".as_ptr() as *const i8
    );
    ((slot - K_SPILL_SLOTS) * 8) as i32
  }
}
