use crate::{
  macros::{cap_unfinished::CAP_UNFINISHED, lua_l_error::luaL_error},
  records::match_state::MatchState,
};

/// # Safety
/// `ms` 须指向存活的 `MatchState`：其 `capture[..ms.level]` 为可访问的捕获数组、`ms.l` 为处于可抛错
/// 受保护帧的 lua_State（越界时经 `luaL_error` 抛错并 unwind，不返回）。`l` 为待解析的捕获序号字符。
/// cpp/VM/src/lstrlib.cpp:198 check_capture。
pub(crate) unsafe fn check_capture(ms: &mut MatchState, mut l: i32) -> i32 {
  unsafe {
    l -= '1' as i32;
    if l < 0 || l >= ms.level || ms.capture[l as usize].len == CAP_UNFINISHED as isize {
      // cpp `check_capture`：lua 错误面文本不变，`luaL_error!` 走 format_args 通道
      luaL_error!(ms.l, "invalid capture index %{}", l + 1);
    }
    l
  }
}
