use alloc::vec::Vec;

use ulua_vm::type_aliases::lua_state::lua_State;

use crate::records::module_counters::ModuleCounters;

#[derive(Debug, Clone, Default)]
pub struct Counters {
  pub(crate) l: *mut lua_State,
  pub(crate) module_refs: Vec<i32>,
  pub(crate) module_counters: Vec<ModuleCounters>,
}
