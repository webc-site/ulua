use crate::{
  functions::{r#match::match_item as match_fn, singlematch::singlematch},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp min_expand`：非贪婪展开，偏移游标版。
/// 返回 `Some(结束 s 偏移)`；`None` 即原 NULL 失败哨兵。
///
/// cpp `lstrlib.cpp min_expand`：非贪婪展开，偏移游标版。
/// 返回 `Some(结束 s 偏移)`；`None` 即原 NULL 失败哨兵。
///
/// 前置条件（由 `prepstate` 建立的 `MatchState` 对象不变式承载）：`s <= ms.src.len()`、
/// 类窗口 `[p, ep]` 在 pattern 界内；推进循环由 `singlematch` 在 `src.len()` 哨兵处收敛
/// 保证不越源串。展开推进本身无指针运算，故签名安全；`unsafe` 仅留在递归匹配内核。
pub(crate) fn min_expand(ms: &mut MatchState, mut s: usize, p: usize, ep: usize) -> Option<usize> {
  loop {
    // cpp: res = match(ms, s, ep + 1)
    // SAFETY: `match_item` 要求 `ms` 处于本次匹配调用中且 `ms.l` 存活可解引用，
    // 该不变式由 `prepstate`（unsafe 入口）建立、沿匹配递归透传同一 `ms`
    if let Some(res) = unsafe { match_fn(ms, s, ep + 1) } {
      return Some(res);
    } else if !singlematch(ms, s, p, ep) {
      return None;
    }
    s += 1; // try with one more repetition
  }
}
