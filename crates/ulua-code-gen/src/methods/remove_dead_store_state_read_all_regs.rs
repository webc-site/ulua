use crate::records::remove_dead_store_state::RemoveDeadStoreState;
impl RemoveDeadStoreState {
  pub fn read_all_regs(&mut self) {
    for i in 0..=self.max_reg {
      self.use_reg(i as u8);
    }

    self.has_gco_to_clear = false;
  }
}
