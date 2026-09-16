use core::ffi::c_char;

use crate::{
  functions::{lua_pushboolean::lua_pushboolean, lua_setfield::lua_setfield},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn setboolfield(l: *mut lua_State, key: &str, value: i32) {
  if value < 0 {
    return;
  }

  unsafe {
    lua_pushboolean(l, value);

    let key_bytes = key.as_bytes();
    let mut buf = key_bytes.to_vec();
    buf.push(0);
    let key_c: *const c_char = buf.as_ptr() as *const c_char;

    lua_setfield(l, -2, key_c);
  }
}
