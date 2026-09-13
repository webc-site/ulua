use crate::records::remove_dead_store_state::RemoveDeadStoreState;
impl RemoveDeadStoreState {
  pub fn use_range(&mut self, start: i32, count: i32) {
    if count == -1 {
      self.use_varargs(start as u8);
    } else {
      for i in start..(start + count) {
        self.use_reg(i as u8);
      }
    }
  }
}
