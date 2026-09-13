use core::{
  ffi::{c_char, c_int},
  ptr::write_bytes,
};

use crate::{
  functions::{
    lua_l_checkbuffer::lua_l_checkbuffer, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkunsigned::lua_l_checkunsigned, lua_l_optinteger::lua_l_optinteger,
  },
  macros::{isoutofbounds::isoutofbounds, lua_l_error::luaL_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_fill(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len);
    let offset = lua_l_checkinteger(l, 2);
    let value = lua_l_checkunsigned(l, 3);
    // C++ evaluates `int(len) - offset` as the default eagerly (signed overflow
    // is UB upstream for offset = INT_MIN); wrapping_sub reproduces the two's-
    // complement value C++ relies on, which the `size < 0` / isoutofbounds checks
    // below then reject. (Upstream UBSan: lbuflib.cpp:278.)
    let size = lua_l_optinteger(l, 4, (len as c_int).wrapping_sub(offset));

    if size < 0 {
      luaL_error!(l, "buffer access out of bounds");
    }

    if isoutofbounds(offset, len, size as usize) {
      luaL_error!(l, "buffer access out of bounds");
    }

    write_bytes(
      (buf as *mut c_char).offset(offset as isize),
      (value & 0xff) as u8,
      size as usize,
    );

    0
  }
}
