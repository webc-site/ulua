use core::ffi::c_int;

use crate::{
  functions::lua_d_reallocstack::lua_d_reallocstack, macros::getgrownstacksize::getgrownstacksize,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_d_growstack(l: *mut lua_State, n: c_int) {
  unsafe {
    lua_d_reallocstack(l, getgrownstacksize(l, n), 0);
  }
}
