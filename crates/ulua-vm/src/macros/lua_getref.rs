use core::ffi::c_int;

use crate::{
  functions::lua_rawgeti::lua_rawgeti, macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::lua_State,
};

#[inline(always)]
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_getref(l: *mut lua_State, ref_: c_int) {
  unsafe {
    lua_rawgeti(l, LUA_REGISTRYINDEX, ref_);
  }
}
