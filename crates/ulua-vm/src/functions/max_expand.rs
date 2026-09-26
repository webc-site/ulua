use crate::{
  functions::{r#match::match_item, singlematch::singlematch},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp max_expand`：贪婪展开，偏移游标版。
/// 返回 `Some(结束 s 偏移)`；`None` 即原 NULL 失败哨兵。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；
/// `s <= ms.src.len()`、类窗口 `[p, ep]` 在 pattern 界内。展开计数受
/// `singlematch` 在 `src.len()` 哨兵处必然收敛的保证约束，`s + i` 不越界。
pub(crate) unsafe fn max_expand(
  ms: &mut MatchState,
  s: usize,
  p: usize,
  ep: usize,
) -> Option<usize> {
  // Safety: 契约保证偏移在源串和捕获数组界内，展开/回退循环受源串剩余长度约束
  unsafe {
    // cpp: ptrdiff_t i = 0; while (singlematch(ms, s + i, p, ep)) i++;
    let mut i: usize = 0; // counts maximum expand for item
    while singlematch(ms, s + i, p, ep) {
      i += 1;
    }

    // cpp: keeps trying to match with the maximum repetitions
    loop {
      if let Some(res) = match_item(ms, s + i, ep + 1) {
        return Some(res);
      }
      if i == 0 {
        return None; // cpp: while (i >= 0) 耗尽，NULL 失败
      }
      i -= 1; // else didn't match; reduce 1 repetition to try again
    }
  }
}
