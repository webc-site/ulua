use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    buffer_errors::{buffer_bitcount_error, buffer_oob_error},
    load_bits_u64::load_bits_u64,
    lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checknumber::lua_l_checknumber,
    lua_pushunsigned::lua_pushunsigned,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_readbits(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let buf = lua_l_checkbuffer(l, 1, &mut len) as *mut c_char;
    let bitoffset = lua_l_checknumber(l, 2) as i64;
    let bitcount = lua_l_checkinteger(l, 3);

    if bitoffset < 0 {
      buffer_oob_error(l);
    }

    if bitcount > 32 {
      buffer_bitcount_error(l);
    }

    if bitoffset as u64 + bitcount as u64 > len as u64 * 8 {
      buffer_oob_error(l);
    }

    let startbyte = (bitoffset / 8) as usize;
    let endbyte = ((bitoffset + bitcount as i64 + 7) / 8) as usize;

    // 字节区间装入 u64（大端逐字节移位 / 小端整块拷贝，与 cpp 一致）
    let data = load_bits_u64(buf, startbyte, endbyte);

    let subbyteoffset = (bitoffset & 0x7) as u64;
    let mask = (1u64 << bitcount as u64) - 1;

    let result = ((data >> subbyteoffset) & mask) as u32;
    lua_pushunsigned(l, result);

    1
  }
}
