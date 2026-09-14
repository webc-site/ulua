use core::{
  ffi::{c_char, c_int},
  mem::size_of,
  ptr::copy_nonoverlapping,
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_swapbe::buffer_swapbe, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checkinteger_64::lua_l_checkinteger_64,
  },
  macros::{isoutofbounds::isoutofbounds, lua_l_error::luaL_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_writelong(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len) as *mut c_char;
    let offset = lua_l_checkinteger(l, 2);
    let value = lua_l_checkinteger_64(l, 3);

    if isoutofbounds(offset, len, size_of::<i64>()) {
      luaL_error!(l, "buffer access out of bounds");
    }

    let value = if LUAU_BIG_ENDIAN {
      buffer_swapbe(value)
    } else {
      value
    };

    copy_nonoverlapping(
      &value as *const i64 as *const c_char,
      buf.add(offset as usize),
      size_of::<i64>(),
    );

    0
  }
}
