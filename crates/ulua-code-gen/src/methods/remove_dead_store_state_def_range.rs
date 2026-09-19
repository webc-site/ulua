use crate::records::remove_dead_store_state::RemoveDeadStoreState;

impl RemoveDeadStoreState {
  pub fn def_range(&mut self, start: i32, count: i32) {
    if count == -1 {
      self.def_varargs(start as u8);
    } else {
      for i in start..(start + count) {
        self.def_reg(i as u8);
      }
    }
  }
}
