use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_vm::{functions::lua_mainthread::lua_mainthread, type_aliases::lua_state::lua_State};

use crate::records::coverage::Coverage;

pub(crate) static mut G_COVERAGE: Coverage = Coverage {
  l: null_mut(),
  functions: Vec::new(),
};

pub fn coverage_init(l: *mut lua_State) {
  unsafe {
    (*core::ptr::addr_of_mut!(G_COVERAGE)).l = lua_mainthread(l);
  }
}
