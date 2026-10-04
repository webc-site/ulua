use crate::{macros::lua_l_error::luaL_error, records::match_state::MatchState};

/// cpp `lstrlib.cpp matchbalance`：`%b` 配对扫描，偏移游标版。
/// 返回 `Some(闭合符之后的 s 偏移)`；`None` 即原 NULL（失配/失衡）。
///
/// 前置条件：`ms` 由 `prepstate` 建立、仍处本次匹配调用中（畸形模式经 `luaL_error`
/// 抛错并 unwind，要求 `ms.l` 存活可抛错）；`s <= ms.src.len()`；
/// `p + 1 >= ms.pat.len()` 的畸形模式经判界报错后不读界外，配对扫描窗口止于
/// `src.len()`（cpp `while (++s < ms->src_end)` 同界）。字节读取走安全切片门面，
/// 签名安全，`unsafe` 仅留在报错内核。
pub(crate) fn matchbalance(ms: &mut MatchState, s: usize, p: usize) -> Option<usize> {
  // cpp: if (p >= ms->p_end - 1) —— 指针差值判定等价为偏移 p + 1 >= pat.len()
  if p + 1 >= ms.pat.len() {
    // SAFETY: 对象不变式（`prepstate` 建立的存活 `lua_State`）保证 `ms.l` 可抛错
    unsafe { luaL_error!(&mut *ms.l, "malformed pattern (missing arguments to '%b')") };
  }
  // s == src.len() 时读终止 NUL（cpp 在 src_end 处读串尾终止符同点位）
  if ms.src_byte(s) != ms.pat_byte(p) {
    None
  } else {
    let b = ms.pat_byte(p);
    let e = ms.pat_byte(p + 1);
    let mut cont = 1i32;
    // cpp: while (++s < ms->src_end) { if (*s == e) { if (--cont == 0) return s + 1; } ... }
    // —— [s+1, src.len()) 半开窗口迭代器正序扫描；命中返回 e 所在偏移 + 1
    let start = (s + 1).min(ms.src.len());
    for (i, &c) in ms.src_slice(start, ms.src.len() - start).iter().enumerate() {
      if c == e {
        cont -= 1;
        if cont == 0 {
          return Some(start + i + 1);
        }
      } else if c == b {
        cont += 1;
      }
    }
    None // string ends out of balance
  }
}
