use crate::{
  functions::r#match::match_item,
  macros::{lua_l_error::luaL_error, lua_maxcaptures::LUA_MAXCAPTURES},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp start_capture`：开捕获槽并递归匹配，偏移游标版。
/// `capture[level].init` 存 `s` 源偏移（原为源串指针）。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；
/// `s <= ms.src.len()`、`p <= ms.pat.len()`；捕获索引在 `LUA_MAXCAPTURES` 界内
/// 否则报错，配对递归复用同一无关区。
pub(crate) unsafe fn start_capture(
  ms: &mut MatchState,
  s: usize,
  p: usize,
  what: i32,
) -> Option<usize> {
  // Safety: 契约保证捕获索引未越 captures 上限且 s 偏移在源串界内
  unsafe {
    let level = ms.level;
    if level >= LUA_MAXCAPTURES {
      luaL_error!(ms.l, "too many captures");
    }
    ms.capture[level as usize].init = s; // cpp: ms->capture[level].init = s（偏移化）
    ms.capture[level as usize].len = what as isize;
    ms.level += 1;
    let res = match_item(ms, s, p);
    if res.is_none() {
      ms.level -= 1; // cpp: undo capture
    }
    res
  }
}
