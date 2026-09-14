use core::ffi::{c_char, c_int};

use crate::{
  functions::lua_l_error_l::lua_l_error_l, macros::cap_unfinished::CAP_UNFINISHED,
  records::match_state::MatchState,
};

pub(crate) unsafe fn check_capture(ms: *mut MatchState, mut l: c_int) -> c_int {
  unsafe {
    l -= '1' as c_int;
    if l < 0 || l >= (*ms).level || (*ms).capture[l as usize].len == CAP_UNFINISHED as isize {
      let fmt = "invalid capture index %d";
      let fmt_ptr = fmt.as_ptr() as *const c_char;
      lua_l_error_l((*ms).l, fmt_ptr, core::format_args!("{}", l + 1));
    }
    l
  }
}
