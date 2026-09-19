use core::ffi::CStr;

use crate::{
  functions::{lua_isnumber::lua_isnumber, lua_rawgetfield::lua_rawgetfield},
  macros::{lua_l_error::luaL_error, lua_pop::lua_pop, lua_tointeger::lua_tointeger},
  type_aliases::lua_state::lua_State,
};

/// `lua_rawgetfield` 需要的键名以 `&CStr` 由类型系统给出，调用点用 `c"..."`
/// 字面量（零分配）；原实现为一次 `Vec<u8>` + 补 NUL 的手工收口。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getfield(l: *mut lua_State, key: &CStr, d: i32) -> i32 {
  unsafe {
    lua_rawgetfield(l, -1, key.as_ptr());

    if lua_isnumber(l, -1) != 0 {
      let res = lua_tointeger!(l, -1) as i32;
      lua_pop(l, 1);
      res
    } else {
      if d < 0 {
        luaL_error!(l, "field '{}' missing in date table", key.to_string_lossy());
      }
      lua_pop(l, 1);
      d
    }
  }
}
