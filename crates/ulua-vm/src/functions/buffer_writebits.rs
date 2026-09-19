use core::{
  ffi::{c_char, c_int},
  ptr::copy_nonoverlapping,
};

use ulua_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

use crate::{
  functions::{
    buffer_errors::{buffer_bitcount_error, buffer_oob_error},
    load_bits_u64::load_bits_u64,
    lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checknumber::lua_l_checknumber,
    lua_l_checkunsigned::lua_l_checkunsigned,
  },
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
      buffer_oob_error(l);
    }

    if !(0..=32).contains(&bitcount) {
      buffer_bitcount_error(l);
    }

    if bitoffset as u64 + bitcount as u64 > len as u64 * 8 {
      buffer_oob_error(l);
    }

    let startbyte = (bitoffset / 8) as usize;
    let endbyte = ((bitoffset + bitcount as i64 + 7) / 8) as usize;

    // 字节区间装入 u64（与 buffer_readbits 共享同一装载逻辑）
    let mut data = load_bits_u64(buf, startbyte, endbyte);

    let subbyteoffset = (bitoffset & 0x7) as u64;
    let mask = (((1u64 << bitcount) - 1) << subbyteoffset) as u64;

    data = (data & !mask) | ((value as u64) << subbyteoffset & mask);

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
