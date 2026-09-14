use core::ffi::{c_char, c_int};

use crate::{
  functions::{match_class::match_class, matchbracketclass::matchbracketclass},
  macros::{l_esc::L_ESC, uchar::uchar},
  records::match_state::MatchState,
};

pub(crate) unsafe fn singlematch(
  ms: *mut MatchState,
  s: *const c_char,
  p: *const c_char,
  ep: *const c_char,
) -> c_int {
  unsafe {
    if s >= (*ms).src_end {
      0
    } else {
      let c = uchar(*s as c_int);
      let p_char = *p;
      if p_char == b'.' as c_char {
        1
      } else if p_char == L_ESC {
        match_class(c as c_int, uchar(*(p.add(1)) as c_int) as c_int)
      } else if p_char == b'[' as c_char {
        matchbracketclass(c as i32, p, ep.offset(-1))
      } else {
        (uchar(p_char as c_int) == c) as c_int
      }
    }
  }
}
