use crate::{
  macros::{cap_unfinished::CAP_UNFINISHED, lua_l_error::luaL_error},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp capture_to_close`（lstrlib.cpp:206）：自顶层向下找最近未闭合捕获。
///
/// # Safety
/// `ms.l` 须为处于可抛错受保护帧的存活 `lua_State`（无未闭合捕获时经 `luaL_error`
/// 抛 "invalid pattern capture" 并 unwind，不返回）；`ms.capture[..ms.level]` 可访问。
pub(crate) unsafe fn capture_to_close(ms: &mut MatchState) -> i32 {
  // cpp `for (level = ms->level - 1; level >= 0; level--)`：自顶向下找首个
  // CAP_UNFINISHED 槽——`(0..ms.level).rev()` 即同一区间（level 为负时为空，同走报错）
  unsafe {
    // cpp `for (level = ms->level; level-- >= 0;)` 自 level-1 逆序扫至 0、返回首个
    // CAP_UNFINISHED 层号；rposition 于索引域 [0, level) 逆序求首命中即与之严格同形，
    // 命中即提前终止而不全遍历
    match ms
      .capture
      .iter()
      .take(ms.level as usize)
      .rposition(|c| c.len == CAP_UNFINISHED as isize)
    {
      Some(level) => level as i32,
      // cpp luaL_error 语义：报错后 longjmp，不返回
      None => luaL_error!(ms.l, "invalid pattern capture"),
    }
  }
}
