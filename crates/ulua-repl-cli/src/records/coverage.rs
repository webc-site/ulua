use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_vm::type_aliases::lua_state::lua_State;

#[derive(Debug, Clone)]
pub struct Coverage {
  pub(crate) l: *mut lua_State,
  pub(crate) functions: Vec<i32>,
}

impl Default for Coverage {
  fn default() -> Self {
    Self {
      l: null_mut(),
      functions: Vec::new(),
    }
  }
}
