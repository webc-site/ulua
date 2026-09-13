use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_vm::type_aliases::lua_state::lua_State;

use crate::records::module_counters::ModuleCounters;

#[derive(Debug, Clone)]
pub struct Counters {
  pub(crate) l: *mut lua_State,
  pub(crate) module_refs: Vec<i32>,
  pub(crate) module_counters: Vec<ModuleCounters>,
}

impl Default for Counters {
  fn default() -> Self {
    Self {
      l: null_mut(),
      module_refs: Vec::new(),
      module_counters: Vec::new(),
    }
  }
}
