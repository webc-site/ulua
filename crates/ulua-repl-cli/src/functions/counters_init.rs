use alloc::vec::Vec;
use core::ptr::{addr_of_mut, null_mut};

use ulua_vm::{functions::lua_mainthread::lua_mainthread, type_aliases::lua_state::lua_State};

use crate::records::counters::Counters;

pub(crate) static mut G_COUNTERS: Counters = Counters {
  l: null_mut(),
  module_refs: Vec::new(),
  module_counters: Vec::new(),
};

/// 对应 cpp `Counters.cpp` 的 `void countersInit(lua_State* L)`。
pub fn counters_init(l: *mut lua_State) {
  unsafe {
    (*addr_of_mut!(G_COUNTERS)).l = lua_mainthread(l);
  }
}
