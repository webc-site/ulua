use crate::records::{remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo};

impl RemoveDeadStoreState {
  pub fn use_reg(&mut self, reg: u8) {
    let reg_info: &mut StoreRegInfo = &mut self.info[reg as usize];

    // Register read doesn't clear the known tag
    reg_info.tag_inst_idx = !0u32;
    reg_info.value_inst_idx = !0u32;
    reg_info.tvalue_inst_idx = !0u32;
    reg_info.maybe_gco = false;
  }
}
