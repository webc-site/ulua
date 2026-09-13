use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checklstring::lua_l_checklstring,
    lua_l_error_l::lua_l_error_l, lua_l_optinteger::lua_l_optinteger,
    lua_pushinteger::lua_pushinteger, lua_pushnil::lua_pushnil, u_posrelat::u_posrelat,
  },
  macros::{iscont::iscont, lua_l_argcheck::luaL_argcheck},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_byteoffset")]
pub(crate) unsafe extern "C-unwind" fn byteoffset(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    let mut n = lua_l_checkinteger(l, 2);
    let mut posi = if n >= 0 { 1 } else { len as i32 + 1 };
    posi = u_posrelat(lua_l_optinteger(l, 3, posi), len);
    luaL_argcheck!(
      l,
      1 <= posi && posi <= len as i32 + 1,
      3,
      "position out of range"
    );
    posi -= 1;

    if n == 0 {
      // find beginning of current byte sequence
      while posi > 0 && iscont(s.add(posi as usize)) {
        posi -= 1;
      }
    } else {
      if iscont(s.add(posi as usize)) {
        lua_l_error_l(
          l,
          c"initial position is a continuation byte".as_ptr(),
          core::format_args!("initial position is a continuation byte"),
        );
      }
      if n < 0 {
        while n < 0 && posi > 0 {
          // move back
          loop {
            // find beginning of previous character
            posi -= 1;
            if !(posi > 0 && iscont(s.add(posi as usize))) {
              break;
            }
          }
          n += 1;
        }
      } else {
        n -= 1; // do not move for 1st character
        while n > 0 && posi < len as i32 {
          loop {
            // find beginning of next character
            posi += 1;
            if !iscont(s.add(posi as usize)) {
              break;
            }
          } // (cannot pass final '\0')
          n -= 1;
        }
      }
    }

    if n == 0 {
      // did it find given character?
      lua_pushinteger(l, posi + 1);
    } else {
      // no such character
      lua_pushnil(l);
    }
    1
  }
}
