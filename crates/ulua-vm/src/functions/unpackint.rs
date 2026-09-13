use core::{
  ffi::{c_char, c_longlong},
  mem::size_of,
};

use crate::{macros::lua_l_error::luaL_error, type_aliases::lua_state::lua_State};

const NB: i32 = 8;
const MC: i32 = 0xff;
const SZINT: i32 = size_of::<c_longlong>() as i32;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn unpackint(
  l: *mut lua_State,
  str: *const c_char,
  islittle: i32,
  size: i32,
  issigned: i32,
) -> i64 {
  unsafe {
    let mut res: u64 = 0;
    let limit = if size <= SZINT { size } else { SZINT };

    for i in (0..limit).rev() {
      res <<= NB as u32;
      let idx = if islittle != 0 { i } else { size - 1 - i };
      let byte = *str.offset(idx as isize) as u8;
      res |= byte as u64;
    }

    if size < SZINT {
      if issigned != 0 {
        let mask = 1u64 << ((size * NB) - 1);
        // C does `(res ^ mask) - mask` in unsigned (wrapping) arithmetic for sign extension.
        res = (res ^ mask).wrapping_sub(mask);
      }
    } else if size > SZINT {
      let mask = if issigned == 0 || res as i64 >= 0 {
        0
      } else {
        MC
      };
      for i in limit..size {
        let idx = if islittle != 0 { i } else { size - 1 - i };
        let byte = *str.offset(idx as isize) as u8;
        if byte as i32 != mask {
          luaL_error!(l, "{}-byte integer does not fit into Lua Integer", size);
        }
      }
    }

    res as i64
  }
}
