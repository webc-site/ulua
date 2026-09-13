use core::ffi::{c_char, c_int};

use crate::{
  functions::{lua_l_checkstack::lua_l_checkstack, push_onecapture::push_onecapture},
  records::match_state::MatchState,
};

pub(crate) unsafe fn push_captures(
  ms: *mut MatchState,
  s: *const c_char,
  e: *const c_char,
) -> c_int {
  unsafe {
    let nlevels = if (*ms).level == 0 && !s.is_null() {
      1
    } else {
      (*ms).level
    };

    lua_l_checkstack((*ms).l, nlevels, "too many captures");

    for i in 0..nlevels {
      push_onecapture(ms, i, s, e);
    }

    nlevels
  }
}
