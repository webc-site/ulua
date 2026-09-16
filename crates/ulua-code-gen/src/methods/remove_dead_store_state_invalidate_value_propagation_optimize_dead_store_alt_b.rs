use crate::records::{remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo};

impl RemoveDeadStoreState {
  pub fn invalidate_value_propagation(&mut self) {
    let max_reg = self.max_reg;
    for i in 0..=max_reg {
      let reg_info: &mut StoreRegInfo =
        unsafe { &mut *(&mut self.info[i as usize] as *mut StoreRegInfo) };
      self.invalidate_value_propagation_store_reg_info(reg_info);
    }
  }
}
