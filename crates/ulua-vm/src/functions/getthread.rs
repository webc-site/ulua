use core::ffi::c_int;

use crate::{
  functions::lua_tothread::lua_tothread, macros::lua_isthread::lua_isthread,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getthread(l: *mut lua_State, arg: *mut c_int) -> *mut lua_State {
  if lua_isthread!(l, 1) {
    unsafe {
      *arg = 1;
      lua_tothread(l, 1)
    }
  } else {
    unsafe {
      *arg = 0;
    }
    l
  }
}
