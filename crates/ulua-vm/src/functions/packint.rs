use core::{
  ffi::{c_char, c_longlong},
  mem::size_of,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{functions::lua_l_addlstring::lua_l_addlstring, records::lua_l_strbuf::LuaLStrbuf};

const MAXINTSIZE: i32 = 16;
const NB: i32 = 8;
const MC: i32 = 0xff;
const SZINT: i32 = size_of::<c_longlong>() as i32;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn packint(b: *mut LuaLStrbuf, mut n: u64, islittle: i32, size: i32, neg: i32) {
  unsafe {
    LUAU_ASSERT!(size <= MAXINTSIZE);
    let mut buff = [0 as c_char; MAXINTSIZE as usize];
    buff[if islittle != 0 { 0 } else { size - 1 } as usize] = (n & MC as u64) as c_char;

    for i in 1..size {
      n >>= NB as u32;
      buff[if islittle != 0 { i } else { size - 1 - i } as usize] = (n & MC as u64) as c_char;
    }

    if neg != 0 && size > SZINT {
      for i in SZINT..size {
        buff[if islittle != 0 { i } else { size - 1 - i } as usize] = MC as c_char;
      }
    }

    lua_l_addlstring(b, buff.as_ptr(), size as usize);
  }
}
