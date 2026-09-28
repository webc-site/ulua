use crate::{
  functions::{capture_to_close::capture_to_close, r#match::match_item},
  macros::cap_unfinished::CAP_UNFINISHED,
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp end_capture`：闭合最近未闭合捕获并继续匹配，偏移游标版。
///
/// 前置条件（由 `prepstate` 建立的 `MatchState` 对象不变式承载）：`s <= ms.src.len()`、
/// `p <= ms.pat.len()`；`capture_to_close` 返回的槽其 `init` 偏移不大于 `s`（匹配游标只
/// 前进，cpp 指针差值 `s - init` 非负）。长度回写为界内数组索引，故签名安全；`unsafe`
/// 仅留在后续递归匹配内核。
pub(crate) fn end_capture(ms: &mut MatchState, s: usize, p: usize) -> Option<usize> {
  let l = capture_to_close(ms) as usize;

  // cpp: ms->capture[l].len = s - ms->capture[l].init;
  // —— 指针差值直译为偏移差值（wrapping 语义与原逐点位一致）
  ms.capture[l].len = (s as isize).wrapping_sub(ms.capture[l].init as isize);

  // SAFETY: `match_item` 要求 `ms` 处于本次匹配调用中且 `ms.l` 存活可解引用，
  // 该不变式由 `prepstate`（unsafe 入口）建立、沿匹配递归透传同一 `ms`
  let res = unsafe { match_item(ms, s, p) };

  if res.is_none() {
    // cpp: ms->capture[l].len = CAP_UNFINISHED;  undo capture
    ms.capture[l].len = CAP_UNFINISHED as isize;
  }

  res
}
