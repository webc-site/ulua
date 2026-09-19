use crate::{
  macros::{cap_unfinished::CAP_UNFINISHED, lua_l_error::luaL_error},
  records::match_state::MatchState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn capture_to_close(ms: *mut MatchState) -> i32 {
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
