use core::{
  ffi::{c_char, c_int},
  slice::from_raw_parts_mut,
};

use crate::{
  functions::{lua_l_checkinteger::luaL_checkinteger, lua_o_utf_8_esc::lua_o_utf_8_esc},
  macros::{cast_to::cast_to, lua_l_argcheck::luaL_argcheck},
  type_aliases::lua_state::lua_State,
};

const MAXUNICODE: i32 = 0x10FFFF;
const UTF8BUFFSZ: usize = 8;

/// # Safety
///
/// `l`, `buff`, and `charstr` must be valid, properly aligned, non-null pointers.
pub(crate) unsafe fn buffutfchar(
  l: *mut lua_State,
  arg: c_int,
  buff: *mut c_char,
  charstr: *mut *const c_char,
) -> c_int {
  unsafe {
    let code = luaL_checkinteger(l, arg);
    luaL_argcheck!(
      l,
      (0..=MAXUNICODE).contains(&code),
      arg,
      "value out of range"
    );

    let buff_slice = from_raw_parts_mut(buff, UTF8BUFFSZ);
    let lval = lua_o_utf_8_esc(
      buff_slice.try_into().expect("UTF8BUFFSZ mismatch"),
      cast_to!(i64, code) as u32,
    );

    *charstr = buff.add(UTF8BUFFSZ).wrapping_sub(lval as usize) as *const c_char;

    lval
  }
}
