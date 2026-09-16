use alloc::vec::Vec;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  ir_function::IrFunction, remove_dead_store_state::RemoveDeadStoreState,
  store_reg_info::StoreRegInfo,
};

impl RemoveDeadStoreState {
  pub fn remove_dead_store_state_remove_dead_store_state(
    function: &mut IrFunction,
    remaining_uses: &mut Vec<u32>,
  ) -> Self {
    let max_reg = if unsafe { (*function).proto }.is_null() {
      255
    } else {
      unsafe { (*(*function).proto).maxstacksize as i32 }
    };

    Self {
      function: function as *mut IrFunction,
      remaining_uses: remaining_uses as *mut Vec<u32>,
      info: [StoreRegInfo::default(); 256],
      max_reg,
      has_gco_to_clear: false,
      has_allocations: false,
      non_propagating_store: DenseHashSet::new(0),
      recorded_vm_exit_syncs: Vec::new(),
    }
  }
}
