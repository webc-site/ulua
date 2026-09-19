use core::ffi::{c_char, c_int};

use crate::{
  functions::{lua_l_checkbuffer::lua_l_checkbuffer, lua_pushlstring::lua_pushlstring},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_tostring(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let data = lua_l_checkbuffer(l, 1, &mut len);

    lua_pushlstring(l, data as *const c_char, len);

    1
  }
}
