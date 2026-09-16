use crate::records::const_prop_state::ConstPropState;

impl ConstPropState {
  pub fn invalidate_heap_table_data(&mut self) {
    self.get_slot_node_cache.clear();
    self.check_slot_match_cache.clear();
    self.get_arr_addr_cache.clear();
    self.check_array_size_cache.clear();
    self.hash_value_cache.clear();
    self.array_value_cache.clear();
  }
}
