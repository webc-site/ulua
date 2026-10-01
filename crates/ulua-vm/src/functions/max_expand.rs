use crate::{
  functions::{r#match::match_item, singlematch::singlematch},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp max_expand`：贪婪展开，偏移游标版。
/// 返回 `Some(结束 s 偏移)`；`None` 即原 NULL 失败哨兵。
///
/// 前置条件（由 `prepstate` 建立的 `MatchState` 对象不变式承载）：`s <= ms.src.len()`、
/// 类窗口 `[p, ep]` 在 pattern 界内。展开计数受 `singlematch` 在 `src.len()` 哨兵处必然
/// 收敛的保证约束，`s + i` 不越界。展开/回退本身无指针运算，故签名安全；`unsafe` 仅留在
/// 递归匹配内核。
pub(crate) fn max_expand(ms: &mut MatchState, s: usize, p: usize, ep: usize) -> Option<usize> {
  // cpp: ptrdiff_t i = 0; while (singlematch(ms, s + i, p, ep)) i++;
  let mut i: usize = 0; // counts maximum expand for item
  while singlematch(ms, s + i, p, ep) {
    i += 1;
  }

  // cpp: keeps trying to match with the maximum repetitions
  loop {
    // SAFETY: `match_item` 要求 `ms` 处于本次匹配调用中且 `ms.l` 存活可解引用，
    // 该不变式由 `prepstate`（unsafe 入口）建立、沿匹配递归透传同一 `ms`
    if let Some(res) = unsafe { match_item(ms, s + i, ep + 1) } {
      return Some(res);
    }
    if i == 0 {
      return None; // cpp: while (i >= 0) 耗尽，NULL 失败
    }
    i -= 1; // else didn't match; reduce 1 repetition to try again
  }
}
