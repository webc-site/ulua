use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_vm::records::lua_state::LuaState;

use crate::records::module_counters::ModuleCounters;

#[derive(Debug, Clone, Default)]
pub(crate) struct Counters {
  /// cpp `Counters::L`（可空）：未 init 即 `None`，null 哨兵用 Option 表达。
  pub(crate) l: Option<NonNull<LuaState>>,
  pub(crate) module_refs: Vec<i32>,
  pub(crate) module_counters: Vec<ModuleCounters>,
}
