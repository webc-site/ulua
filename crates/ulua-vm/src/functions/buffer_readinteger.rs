use core::{
  ffi::c_int,
  mem::{size_of, zeroed},
  ptr::copy_nonoverlapping,
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_swapbe::buffer_swapbe, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_pushnumber::lua_pushnumber,
  },
  macros::{isoutofbounds::isoutofbounds, lua_l_error::luaL_error},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn buffer_readinteger<T>(l: *mut lua_State) -> c_int
where
  T: Copy + Into<f64>,
{
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len);
    let offset = lua_l_checkinteger(l, 2);

    if isoutofbounds(offset, len, size_of::<T>()) {
      luaL_error!(l, "buffer access out of bounds");
    }

    let mut val: T = zeroed();
    copy_nonoverlapping(
      (buf as *const u8).add(offset as usize),
      &mut val as *mut T as *mut u8,
      size_of::<T>(),
    );

    if LUAU_BIG_ENDIAN {
      val = buffer_swapbe(val);
    }

    lua_pushnumber(l, val.into());
    1
  }
}
