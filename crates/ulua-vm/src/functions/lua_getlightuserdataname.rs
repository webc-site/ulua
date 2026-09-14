use core::{ffi::c_char, ptr::null};

use crate::{
  macros::{api_check::api_check, getstr::getstr, lua_lutag_limit::LUA_LUTAG_LIMIT},
  type_aliases::{lua_state::lua_State, t_string::tstring},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_getlightuserdataname(l: *mut lua_State, tag: i32) -> *const c_char {
  api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);

  unsafe {
    let global = (*l).global;
    let name = (*global).lightuserdataname[tag as usize];
    if name.is_null() {
      null()
    } else {
      getstr(name as *const tstring)
    }
  }
}
