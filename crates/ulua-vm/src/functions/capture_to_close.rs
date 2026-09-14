use core::ffi::c_int;

use crate::{
  macros::{cap_unfinished::CAP_UNFINISHED, lua_l_error::luaL_error},
  records::match_state::MatchState,
};

pub(crate) unsafe fn capture_to_close(ms: *mut MatchState) -> c_int {
  unsafe {
    let mut level = (*ms).level;
    level -= 1;
    while level >= 0 {
      if (*ms).capture[level as usize].len == CAP_UNFINISHED as isize {
        return level;
      }
      level -= 1;
    }

    // cpp luaL_error 语义：报错后 longjmp，不返回
    luaL_error!((*ms).l, "invalid pattern capture")
  }
}
