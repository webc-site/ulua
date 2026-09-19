use core::ffi::{c_char, c_int};

use crate::{
  functions::r#match::match_item,
  macros::{lua_l_error::luaL_error, lua_maxcaptures::LUA_MAXCAPTURES},
  records::match_state::MatchState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn start_capture(
  ms: *mut MatchState,
  s: *const c_char,
  p: *const c_char,
  what: c_int,
) -> *const c_char {
  unsafe {
    let level = (*ms).level;
    if level >= LUA_MAXCAPTURES {
      luaL_error!((*ms).l, "too many captures");
    }
    (*ms).capture[level as usize].init = s;
    (*ms).capture[level as usize].len = what as isize;
    (*ms).level = level + 1;
    let res = match_item(ms, s, p);
    if res.is_null() {
      (*ms).level -= 1;
    }
    res
  }
}
