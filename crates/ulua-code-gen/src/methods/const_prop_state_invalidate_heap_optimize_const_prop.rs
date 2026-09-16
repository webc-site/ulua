use crate::records::const_prop_state::ConstPropState;
impl ConstPropState {
  pub fn invalidate_heap(&mut self) {
    // cpp 无条件清理基于指令的堆状态缓存（OptimizeConstProp.cpp:316-323）
    self.inst_not_readonly.clear();
    self.inst_no_metatable.clear();
    self.inst_array_size.clear();

    self.invalidate_heap_table_data();

    // Buffer length checks are not invalidated since buffer size is immutable
    self.buffer_load_store_info.clear();
  }
}
