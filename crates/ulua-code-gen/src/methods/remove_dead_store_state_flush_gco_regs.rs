use ulua_common::FFlag::LuauCodegenVmExitSync;

use crate::{
  functions::is_gco::is_gco,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo},
};

impl RemoveDeadStoreState {
  // Partial clear of information about registers that might contain a GC object
  pub fn flush_gco_regs(&mut self) {
    let max_reg = self.max_reg;

    for i in 0..=max_reg {
      let reg_info: &mut StoreRegInfo =
        unsafe { &mut *(&mut self.info[i as usize] as *mut StoreRegInfo) };

      if reg_info.maybe_gco {
        // If we happen to know the exact tag, it has to be a GCO, otherwise 'maybeGCO' should be false
        CODEGEN_ASSERT!(reg_info.known_tag == 0xff || is_gco(reg_info.known_tag));

        // If the values stored are still used and might be a GCO object, we have to pin in to the stack
        let tag_used_after =
          reg_info.tag_inst_idx != !0u32 && self.has_remaining_uses(reg_info.tag_inst_idx);
        let value_used_after =
          reg_info.value_inst_idx != !0u32 && self.has_remaining_uses(reg_info.value_inst_idx);
        let tvalue_used_after =
          reg_info.tvalue_inst_idx != !0u32 && self.has_remaining_uses(reg_info.tvalue_inst_idx);

        if tag_used_after || value_used_after || tvalue_used_after {
          reg_info.tag_inst_idx = !0u32;
          reg_info.value_inst_idx = !0u32;
          reg_info.tvalue_inst_idx = !0u32;
        }

        if LuauCodegenVmExitSync.get() {
          // If the GCO values remain, they can no longer be propagated further as that will create a new use
          self.invalidate_value_propagation_store_reg_info(reg_info);
        }

        // Indirect register read by GC doesn't clear the known tag
        reg_info.maybe_gco = false;
      }
    }

    self.has_gco_to_clear = false;
  }
}
