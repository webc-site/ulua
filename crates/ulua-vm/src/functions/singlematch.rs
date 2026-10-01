use crate::{
  functions::{match_class::match_class, matchbracketclass::matchbracketclass},
  macros::l_esc::L_ESC,
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp singlematch`（lstrlib.cpp:317）：源游标 `s` 偏移处的单字符
/// 是否匹配模式类 `[p, ep)`（`ep` 为 `classend` 结果，右开）。返回 bool 取代原
/// i32 0/1。
///
/// 全部字节读取走 [`MatchState`] 的安全切片门面，界外偏移由门面按 cpp 终止 NUL
/// 语义取 0，故本函数无 `unsafe`：`s == ms.src.len()` 走下方 `src_end` 哨兵分支
/// （cpp `if (s >= ms->src_end) return 0;`），`p + 1`/类窗口 `[p, ep)` 的越界角点
/// 等价于 cpp 读模式串终止 NUL（`pat_byte` 契约）。
pub(crate) fn singlematch(ms: &MatchState, s: usize, p: usize, ep: usize) -> bool {
  debug_assert!(p < ms.pat.len() && p < ep && ep <= ms.pat.len());
  // cpp: if (s >= ms->src_end) return 0; —— 偏移 == src.len() 即原 src_end 哨兵
  if s >= ms.src.len() {
    return false;
  }
  let c = ms.src_byte(s); // cpp `uchar(*s)`：u8 读取即 uchar 语义
  match ms.pat_byte(p) {
    b'.' => true, // matches any char
    x if x == L_ESC as u8 => match_class(c as i32, ms.pat_byte(p + 1) as i32) != 0,
    b'[' => matchbracketclass(c as i32, ms.pat_slice(p, ep - p)) != 0,
    other => other == c, // cpp: return (uchar(*p) == c)
  }
}
