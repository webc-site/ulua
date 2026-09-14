use crate::records::{remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo};

impl RemoveDeadStoreState {
  // When a register value is being defined, it kills previous stores
  pub fn def_reg(&mut self, reg: u8) {
    // Stores to captured registers are not removed since we don't track their uses outside of function
    let function = unsafe { &*self.function };
    if (function.cfg.captured.regs[reg as usize / 64] & (1u64 << (reg as usize % 64))) != 0 {
      return;
    }

    let reg_info: &mut StoreRegInfo =
      unsafe { &mut *(&mut self.info[reg as usize] as *mut StoreRegInfo) };

    self.kill_tag_and_value_store_pair(reg_info);
    self.kill_t_value_store(reg_info);

    reg_info.tag_inst_idx = !0u32;
    reg_info.value_inst_idx = !0u32;
    reg_info.tvalue_inst_idx = !0u32;

    // Opaque register definition removes the knowledge of the actual tag value
    reg_info.known_tag = 0xff;

    // New value defined, before MARK_DEAD is used again, it might be used in a VM exit
    reg_info.ignore_at_exit = false;
  }
}
