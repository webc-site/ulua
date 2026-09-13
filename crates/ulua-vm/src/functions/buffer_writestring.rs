use core::{
  ffi::{c_char, c_int},
  ptr::copy_nonoverlapping,
};

use crate::{
  functions::{
    lua_l_checkbuffer::lua_l_checkbuffer, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checklstring::lua_l_checklstring, lua_l_optinteger::lua_l_optinteger,
  },
  macros::{isoutofbounds::isoutofbounds, lua_l_argcheck::luaL_argcheck, lua_l_error::luaL_error},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn buffer_writestring(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len);
    let offset = lua_l_checkinteger(l, 2);

    let mut size: usize = 0;
    let val = lua_l_checklstring(l, 3, &mut size);
    let count = lua_l_optinteger(l, 4, size as c_int);

    luaL_argcheck!(l, count >= 0, 4, "count");

    if count as usize > size {
      luaL_error!(l, "string length overflow");
    }

    if isoutofbounds(offset, len, count as usize) {
      luaL_error!(l, "buffer access out of bounds");
    }

    copy_nonoverlapping(
      val,
      (buf as *mut c_char).add(offset as usize),
      count as usize,
    );

    0
  }
}
