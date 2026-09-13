use crate::records::const_prop_state::ConstPropState;

impl ConstPropState {
  pub fn invalidate_heap_buffer_data(&mut self) {
    self.check_buffer_len_cache.clear();
    self.buffer_load_store_info.clear();
  }
}
