use crate::{functions::check_capture::check_capture, records::match_state::MatchState};

/// cpp `lstrlib.cpp match_capture`：`%1`-`%9` 回填捕获文本，偏移游标版。
/// 返回 `Some(捕获文本之后的 s 偏移)`；`None` 即原 NULL 失配。
///
/// 前置条件（由 `prepstate` 建立的 `MatchState` 不变式与匹配游标保证，非调用方
/// 裸指针契约）：`ms` 仍处本次匹配调用中、`s <= ms.src.len()`；捕获序号 `l` 越界/
/// 未完成时 `check_capture` 抛 "invalid capture index"、不返回，返回后捕获槽
/// `init + len <= ms.src.len()` 界内成立。函数体仅偏移与切片比较，无 `unsafe`。
pub(crate) fn match_capture(ms: &mut MatchState, s: usize, l: i32) -> Option<usize> {
  let slot = check_capture(ms, l) as usize;
  let len = ms.capture[slot].len as usize;
  // cpp: if ((size_t)(ms->src_end - s) >= len && memcmp(ms->capture[l].init, s, len) == 0)
  //        return s + len;
  // —— src_end - s 的指针差值等价为 src.len() - s；memcmp 等价为源偏移切片比对
  if ms.src.len() - s >= len && ms.src_slice(ms.capture[slot].init, len) == ms.src_slice(s, len) {
    Some(s + len)
  } else {
    None
  }
}
