use crate::{functions::check_capture::check_capture, records::match_state::MatchState};

/// cpp `lstrlib.cpp match_capture`：`%1`-`%9` 回填捕获文本，偏移游标版。
/// 返回 `Some(捕获文本之后的 s 偏移)`；`None` 即原 NULL 失配。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；`s <= ms.src.len()`；
/// 捕获序号 `l` 越界/未完成时 `check_capture` 抛 "invalid capture index"、不返回，
/// 返回后捕获槽 `init + len <= ms.src.len()` 界内成立。cpp lstrlib.cpp:417 `match_capture`。
pub(crate) unsafe fn match_capture(ms: &mut MatchState, s: usize, l: i32) -> Option<usize> {
  // Safety: 契约保证捕获序号小于 captures 上限且偏移落在源串界内
  unsafe {
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
}
