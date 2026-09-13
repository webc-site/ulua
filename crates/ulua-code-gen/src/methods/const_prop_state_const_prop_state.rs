use alloc::vec::Vec;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
  const_prop_state::ConstPropState, ir_builder::IrBuilder, ir_data::K_INVALID_INST_IDX,
  ir_function::IrFunction, ir_inst::IrInst, register_info::RegisterInfo,
};

impl ConstPropState {
  pub fn const_prop_state_const_prop_state(
    build: &mut IrBuilder,
    function: &mut IrFunction,
  ) -> Self {
    Self {
      build: build as *mut IrBuilder,
      function: function as *mut IrFunction,
      regs: [RegisterInfo::default(); 256],
      max_reg: 0,
      inst_pos: 0,
      in_safe_env: false,
      checked_gc: false,
      inst_link: DenseHashMap::new(K_INVALID_INST_IDX),
      inst_tag: DenseHashMap::new(K_INVALID_INST_IDX),
      inst_value: DenseHashMap::new(K_INVALID_INST_IDX),
      value_map: DenseHashMap::new(IrInst::default()),
      upvalue_map: DenseHashMap::new(0xff),
      hash_value_cache: DenseHashMap::new(K_INVALID_INST_IDX),
      array_value_cache: Vec::new(),
      try_num_to_index_cache: Vec::new(),
      get_slot_node_cache: Vec::new(),
      check_slot_match_cache: Vec::new(),
      get_arr_addr_cache: Vec::new(),
      check_array_size_cache: Vec::new(),
      check_buffer_len_cache: Vec::new(),
      useradata_tag_cache: Vec::new(),
      buffer_load_store_info: Vec::new(),
      load_env_idx: K_INVALID_INST_IDX,
      inst_not_readonly: DenseHashSet::new(K_INVALID_INST_IDX),
      inst_no_metatable: DenseHashSet::new(K_INVALID_INST_IDX),
      inst_array_size: DenseHashMap::new(K_INVALID_INST_IDX),
      range_end_temp: Vec::new(),
    }
  }
}
