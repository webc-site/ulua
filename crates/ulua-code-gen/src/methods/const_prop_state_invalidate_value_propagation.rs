use ulua_common::FFlag::LuauCodegenExtraTableOpts;

use crate::records::{const_prop_state::ConstPropState, ir_data::K_INVALID_INST_IDX};
impl ConstPropState {
  pub fn invalidate_value_propagation(&mut self) {
    self.value_map.clear();
    self.upvalue_map.clear();

    self.try_num_to_index_cache.clear();

    self.buffer_load_store_info.clear();

    self.hash_value_cache.clear();
    self.array_value_cache.clear();

    // While other map clears already prevent instValue keys from matching again, this saves memory and map size
    self.inst_value.clear();

    if LuauCodegenExtraTableOpts.get() {
      self.load_env_idx = K_INVALID_INST_IDX;
    }
  }
}
