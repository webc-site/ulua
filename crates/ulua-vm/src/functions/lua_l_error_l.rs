use core::{ffi::c_char, fmt::Arguments};

use crate::{
  functions::{
    lua_concat::lua_concat, lua_error::lua_error, lua_l_where::lua_l_where,
    lua_pushvfstring::lua_pushvfstring,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_l_error_l(l: *mut lua_State, fmt: *const c_char, args: Arguments<'_>) {
  unsafe {
    lua_l_where(l, 1);
    lua_pushvfstring(l, fmt, args);
    lua_concat(l, 2);
    lua_error(l);
  }
}
