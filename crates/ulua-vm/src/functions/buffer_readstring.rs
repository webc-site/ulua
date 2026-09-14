use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    lua_l_checkbuffer::lua_l_checkbuffer, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_error_l::lua_l_error_l, lua_pushlstring::lua_pushlstring,
  },
  macros::{isoutofbounds::isoutofbounds, lua_l_argcheck::luaL_argcheck},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `l` must point to a valid, properly initialized `lua_State`.
pub(crate) unsafe extern "C-unwind" fn buffer_readstring(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len);
    let offset = lua_l_checkinteger(l, 2);
    let size = lua_l_checkinteger(l, 3);

    luaL_argcheck!(l, size >= 0, 3, "size");

    if isoutofbounds(offset, len, size as usize) {
      let msg = b"buffer access out of bounds\0";
      lua_l_error_l(
        l,
        msg.as_ptr() as *const c_char,
        core::format_args!("buffer access out of bounds"),
      );
    }

    let data_ptr = (buf as *const c_char).add(offset as usize);
    lua_pushlstring(l, data_ptr, size as usize);

    1
  }
}
