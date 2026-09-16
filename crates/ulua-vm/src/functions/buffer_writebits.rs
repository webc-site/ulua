use core::{
  ffi::{c_char, c_int},
  ptr::copy_nonoverlapping,
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    lua_l_checkbuffer::lua_l_checkbuffer, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checknumber::lua_l_checknumber, lua_l_checkunsigned::lua_l_checkunsigned,
  },
  macros::lua_l_error::luaL_error,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_writebits(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len) as *mut c_char;
    let bitoffset = lua_l_checknumber(l, 2) as i64;
    let bitcount = lua_l_checkinteger(l, 3);
    let value = lua_l_checkunsigned(l, 4);

    if bitoffset < 0 {
      luaL_error!(l, "buffer access out of bounds");
    }

    if !(0..=32).contains(&bitcount) {
      luaL_error!(l, "bit count is out of range of [0; 32]");
    }

    if (bitoffset as u64 + bitcount as u64) > (len as u64) * 8 {
      luaL_error!(l, "buffer access out of bounds");
    }

    let startbyte = (bitoffset / 8) as usize;
    let endbyte = ((bitoffset + bitcount as i64 + 7) / 8) as usize;

    let mut data: u64 = 0;

    if LUAU_BIG_ENDIAN {
      for i in (startbyte..endbyte).rev() {
        data = data * 256 + (*buf.add(i) as u8) as u64;
      }
    } else {
      copy_nonoverlapping(
        buf.add(startbyte),
        &mut data as *mut u64 as *mut c_char,
        endbyte - startbyte,
      );
    }

    let subbyteoffset = (bitoffset & 0x7) as u64;
    let mask = (((1u64 << bitcount) - 1) << subbyteoffset) as u64;

    data = (data & !mask) | (((value as u64) << subbyteoffset) & mask);

    if LUAU_BIG_ENDIAN {
      for i in startbyte..endbyte {
        *buf.add(i) = (data & 0xff) as c_char;
        data >>= 8;
      }
    } else {
      copy_nonoverlapping(
        &data as *const u64 as *const c_char,
        buf.add(startbyte),
        endbyte - startbyte,
      );
    }

    0
  }
}
