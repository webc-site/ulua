use core::ffi::{c_char, c_int};

use crate::{
  functions::{
    lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_addvalue::lua_l_addvalue, lua_l_prepbuffsize::lua_l_prepbuffsize,
    lua_tolstring::lua_tolstring, push_onecapture::push_onecapture,
  },
  macros::{l_esc::L_ESC, lua_l_error::luaL_error, uchar::uchar},
  records::{lua_l_strbuf::LuaLStrbuf, match_state::MatchState},
};

pub(crate) unsafe fn add_s(
  ms: *mut MatchState,
  b: *mut LuaLStrbuf,
  s: *const c_char,
  e: *const c_char,
) {
  unsafe {
    let mut l: usize = 0;
    let news = lua_tolstring((*ms).l, 3, &mut l);

    lua_l_prepbuffsize(b, l);

    let mut i: usize = 0;
    while i < l {
      if *news.add(i) != L_ESC {
        lua_l_addchar(b, *news.add(i));
      } else {
        i += 1; // skip ESC
        let next = uchar(*news.add(i) as c_int) as u8;
        if !next.is_ascii_digit() {
          if *news.add(i) != L_ESC {
            luaL_error!(
              (*ms).l,
              "invalid use of '{}' in replacement string",
              L_ESC as u8 as char
            );
          }
          lua_l_addchar(b, *news.add(i));
        } else if next == b'0' {
          lua_l_addlstring(b, s, (e as usize).wrapping_sub(s as usize));
        } else {
          push_onecapture(ms, (next - b'1') as i32, s, e);
          lua_l_addvalue(b); // add capture to accumulated result
        }
      }
      i += 1;
    }
  }
}
