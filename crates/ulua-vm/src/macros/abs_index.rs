use core::ffi::c_int;

use crate::{
  functions::lua_gettop::lua_gettop, macros::lua_registryindex::LUA_REGISTRYINDEX,
  type_aliases::lua_state::lua_State,
};
#[inline(always)]
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn abs_index(l: *mut lua_State, i: c_int) -> c_int {
  if i > 0 || i <= LUA_REGISTRYINDEX {
    i
  } else {
    unsafe { lua_gettop(l) + i + 1 }
  }
}
