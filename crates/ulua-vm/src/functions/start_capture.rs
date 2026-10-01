use crate::{
  functions::r#match::match_item,
  macros::{lua_l_error::luaL_error, lua_maxcaptures::LUA_MAXCAPTURES},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp start_capture`：开捕获槽并递归匹配，偏移游标版。
/// `capture[level].init` 存 `s` 源偏移（原为源串指针）。
///
/// 前置条件（由 `prepstate` 建立的 `MatchState` 对象不变式承载，非调用方裸指针契约）：
/// `s <= ms.src.len()`、`p <= ms.pat.len()`；捕获索引在 `LUA_MAXCAPTURES` 界内否则报错，
/// 配对递归复用同一无关区。槽位写入为界内数组索引，故签名安全；`unsafe` 收窄到报错与
/// 递归匹配两个内核。
pub(crate) fn start_capture(ms: &mut MatchState, s: usize, p: usize, what: i32) -> Option<usize> {
  let level = ms.level;
  if level >= LUA_MAXCAPTURES {
    // SAFETY: 对象不变式（`prepstate` 建立的存活 `lua_State`）保证 `ms.l` 可抛错
    unsafe { luaL_error!(ms.l, "too many captures") };
  }
  ms.capture[level as usize].init = s; // cpp: ms->capture[level].init = s（偏移化）
  ms.capture[level as usize].len = what as isize;
  ms.level += 1;
  // SAFETY: `match_item` 要求 `ms` 处于本次匹配调用中且 `ms.l` 存活可解引用，
  // 该不变式由 `prepstate`（unsafe 入口）建立、并沿匹配递归一路透传同一 `ms`
  let res = unsafe { match_item(ms, s, p) };
  if res.is_none() {
    ms.level -= 1; // cpp: undo capture
  }
  res
}
