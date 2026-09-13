//! Source: `CodeGen/src/OptimizeDeadStore.cpp:96-648`

use alloc::vec::Vec;
use core::array::from_fn;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{ir_function::IrFunction, store_reg_info::StoreRegInfo};

#[derive(Debug, Clone)]
pub struct RemoveDeadStoreState {
  pub(crate) function: *mut IrFunction,
  pub(crate) remaining_uses: *mut Vec<u32>,
  pub(crate) info: [StoreRegInfo; 256],
  pub(crate) max_reg: i32,
  pub(crate) has_gco_to_clear: bool,
  pub(crate) has_allocations: bool,
  pub(crate) non_propagating_store: DenseHashSet<u32>,
  pub(crate) recorded_vm_exit_syncs: Vec<u32>,
}

impl RemoveDeadStoreState {
  pub fn remove_dead_store_state_remove_dead_store_state(
    function: &mut IrFunction,
    remaining_uses: &mut Vec<u32>,
  ) -> Self {
    Self {
      function: function as *mut IrFunction,
      remaining_uses: remaining_uses as *mut Vec<u32>,
      info: from_fn(|_| StoreRegInfo::default()),
      max_reg: 255,
      has_gco_to_clear: false,
      has_allocations: false,
      non_propagating_store: DenseHashSet::new(!0u32),
      recorded_vm_exit_syncs: Vec::new(),
    }
  }
}
