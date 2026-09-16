use core::ffi::c_int;

use crate::{
  macros::{
    api_check::api_check, expandstacklimit::expandstacklimit, lua_d_checkstack::luaD_checkstack,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_rawcheckstack(l: *mut lua_State, size: c_int) {
  api_check!(l, size >= 0);

  unsafe {
    luaD_checkstack!(l, size);
    expandstacklimit!(l, (*l).top.wrapping_add(size as usize));
  }
}
