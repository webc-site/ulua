use core::ffi::{CStr, c_int};

use crate::{
  functions::{lua_pushinteger::lua_pushinteger, lua_setfield::lua_setfield},
  type_aliases::lua_state::lua_State,
};

/// `key` 由 `&CStr` 承担 NUL 结尾契约（调用点为 `c"..."` 字面量，零分配）。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn setfield(l: *mut lua_State, key: &CStr, value: i32) {
  unsafe {
    lua_pushinteger(l, value as c_int);

    lua_setfield(l, -2, key.as_ptr());
  }
}
