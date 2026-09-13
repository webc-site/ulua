use core::{ffi::c_char, ptr::null};

use crate::{macros::lua_l_error::luaL_error, records::match_state::MatchState};

pub(crate) unsafe fn matchbalance(
  ms: *mut MatchState,
  s: *const c_char,
  p: *const c_char,
) -> *const c_char {
  unsafe {
    if p >= (*ms).p_end.offset(-1) {
      luaL_error!((*ms).l, "malformed pattern (missing arguments to '%%b')");
    }
    if *s != *p {
      null()
    } else {
      let b = *p;
      let e = *p.add(1);
      let mut cont = 1;
      let mut s = s.add(1);
      while s < (*ms).src_end {
        if *s == e {
          cont -= 1;
          if cont == 0 {
            return s.add(1);
          }
        } else if *s == b {
          cont += 1;
        }
        s = s.add(1);
      }
      null()
    }
  }
}
