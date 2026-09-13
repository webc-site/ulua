use core::ffi::c_char;

use crate::{
  functions::{lua_isnumber::lua_isnumber, lua_rawgetfield::lua_rawgetfield},
  macros::{lua_l_error::luaL_error, lua_pop::lua_pop, lua_tointeger::lua_tointeger},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getfield(l: *mut lua_State, key: &str, d: i32) -> i32 {
  let key_bytes = key.as_bytes();
  let mut buf = key_bytes.to_vec();
  buf.push(0);
  let key_c: *const c_char = buf.as_ptr() as *const c_char;

  unsafe {
    lua_rawgetfield(l, -1, key_c);

    if lua_isnumber(l, -1) != 0 {
      let res = lua_tointeger!(l, -1) as i32;
      lua_pop(l, 1);
      res
    } else {
      if d < 0 {
        luaL_error!(l, "field '{}' missing in date table", key);
      }
      lua_pop(l, 1);
      d
    }
  }
}
