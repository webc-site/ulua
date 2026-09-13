use core::ffi::c_int;

use crate::{
  functions::resize::resize,
  type_aliases::{lua_state::lua_State, lua_table::LuaTable},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_h_resizehash(l: *mut lua_State, t: *mut LuaTable, nhsize: c_int) {
  unsafe {
    resize(l, t, (*t).sizearray, nhsize);
  }
}
