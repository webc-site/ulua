use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkbuffer::lua_l_checkbuffer, lua_pushnumber::lua_pushnumber},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_len(l: *mut lua_State) -> c_int {
  let mut len: usize = 0;
  unsafe {
    lua_l_checkbuffer(l, 1, &mut len);
    lua_pushnumber(l, len as f64);
  }
  1
}
