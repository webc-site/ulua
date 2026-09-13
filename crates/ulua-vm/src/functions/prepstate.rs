use core::ffi::c_char;

use crate::{
  macros::luai_maxccalls::LUAI_MAXCCALLS, records::match_state::MatchState,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn prepstate(
  ms: *mut MatchState,
  l: *mut lua_State,
  s: *const c_char,
  ls: usize,
  p: *const c_char,
  lp: usize,
) {
  unsafe {
    (*ms).l = l;
    (*ms).matchdepth = LUAI_MAXCCALLS;
    (*ms).src_init = s;
    (*ms).src_end = s.add(ls);
    (*ms).p_end = p.add(lp);
  }
}
