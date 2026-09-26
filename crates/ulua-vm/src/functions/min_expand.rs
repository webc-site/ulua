use crate::{
  functions::{r#match::match_item as match_fn, singlematch::singlematch},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp min_expand`：非贪婪展开，偏移游标版。
/// 返回 `Some(结束 s 偏移)`；`None` 即原 NULL 失败哨兵。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；
/// `s <= ms.src.len()`、类窗口 `[p, ep]` 在 pattern 界内；推进循环由
/// `singlematch` 在 `src.len()` 哨兵处收敛保证不越源串。
pub(crate) unsafe fn min_expand(
  ms: &mut MatchState,
  mut s: usize,
  p: usize,
  ep: usize,
) -> Option<usize> {
  // Safety: 契约保证偏移在源串和捕获数组界内，最小展开推进不越过源串剩余长度
  unsafe {
    loop {
      // cpp: res = match(ms, s, ep + 1)
      if let Some(res) = match_fn(ms, s, ep + 1) {
        return Some(res);
      } else if !singlematch(ms, s, p, ep) {
        return None;
      }
      s += 1; // try with one more repetition
    }
  }
}
