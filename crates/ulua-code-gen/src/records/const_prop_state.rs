use alloc::vec::Vec;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::records::{
  array_value_entry::ArrayValueEntry, buffer_load_store_info::BufferLoadStoreInfo,
  ir_builder::IrBuilder, ir_function::IrFunction, ir_inst::IrInst, ir_inst_eq::IrInstEq,
  ir_inst_hash::IrInstHash, ir_op::IrOp, node_slot_state::NodeSlotState,
  numbered_instruction::NumberedInstruction, register_info::RegisterInfo,
  register_link::RegisterLink,
};

#[derive(Debug)]
pub struct ConstPropState {
  pub build: *mut IrBuilder,
  pub function: *mut IrFunction,
  pub regs: [RegisterInfo; 256],
  pub max_reg: i32,
  pub inst_pos: u32,
  pub in_safe_env: bool,
  pub checked_gc: bool,
  pub inst_link: DenseHashMap<u32, RegisterLink>,
  pub inst_tag: DenseHashMap<u32, u8>,
  pub inst_value: DenseHashMap<u32, IrOp>,
  pub value_map: DenseHashMap<IrInst, u32, IrInstHash, IrInstEq>,
  pub upvalue_map: DenseHashMap<u8, u32>,
  pub hash_value_cache: DenseHashMap<u32, u32>,
  pub array_value_cache: Vec<ArrayValueEntry>,
  pub try_num_to_index_cache: Vec<u32>,
  pub get_slot_node_cache: Vec<NumberedInstruction>,
  pub check_slot_match_cache: Vec<NodeSlotState>,
  pub get_arr_addr_cache: Vec<u32>,
  pub check_array_size_cache: Vec<u32>,
  pub check_buffer_len_cache: Vec<u32>,
  pub useradata_tag_cache: Vec<u32>,
  pub buffer_load_store_info: Vec<BufferLoadStoreInfo>,
  pub load_env_idx: u32,
  pub inst_not_readonly: DenseHashSet<u32>,
  pub inst_no_metatable: DenseHashSet<u32>,
  pub inst_array_size: DenseHashMap<u32, i32>,
  pub range_end_temp: Vec<u32>,
}

impl ConstPropState {
  pub fn clear(&mut self) {
    for i in 0..=self.max_reg as usize {
      self.regs[i] = RegisterInfo::default();
    }
    self.max_reg = 0;
    self.inst_pos = 0;
    self.in_safe_env = false;
    self.checked_gc = false;
    self.inst_link.clear();
    self.inst_tag.clear();
    self.inst_value.clear();
    self.value_map.clear();
    self.upvalue_map.clear();
    self.hash_value_cache.clear();
    self.array_value_cache.clear();
    self.try_num_to_index_cache.clear();
    self.get_slot_node_cache.clear();
    self.check_slot_match_cache.clear();
    self.get_arr_addr_cache.clear();
    self.check_array_size_cache.clear();
    self.check_buffer_len_cache.clear();
    self.useradata_tag_cache.clear();
    self.buffer_load_store_info.clear();
    self.load_env_idx = !0;
    self.inst_not_readonly.clear();
    self.inst_no_metatable.clear();
    self.inst_array_size.clear();
    self.range_end_temp.clear();
  }
}
