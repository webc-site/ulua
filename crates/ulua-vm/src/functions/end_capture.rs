use crate::{
  functions::{capture_to_close::capture_to_close, r#match::match_item},
  macros::cap_unfinished::CAP_UNFINISHED,
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp end_capture`：闭合最近未闭合捕获并继续匹配，偏移游标版。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；
/// `s <= ms.src.len()`、`p <= ms.pat.len()`；`capture_to_close` 返回的槽其
/// `init` 偏移不大于 `s`（匹配游标只前进，cpp 指针差值 `s - init` 非负）。
pub(crate) unsafe fn end_capture(ms: &mut MatchState, s: usize, p: usize) -> Option<usize> {
  // Safety: 契约保证捕获索引在 captures 数组界内，回填起止偏移与后续匹配推进均在串界内
  unsafe {
    let l = capture_to_close(ms) as usize;

    // cpp: ms->capture[l].len = s - ms->capture[l].init;
    // —— 指针差值直译为偏移差值（wrapping 语义与原逐点位一致）
    ms.capture[l].len = (s as isize).wrapping_sub(ms.capture[l].init as isize);

    let res = match_item(ms, s, p);

    if res.is_none() {
      // cpp: ms->capture[l].len = CAP_UNFINISHED;  undo capture
      ms.capture[l].len = CAP_UNFINISHED as isize;
    }

    res
  }
}
