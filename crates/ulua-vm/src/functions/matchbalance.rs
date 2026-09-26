use crate::{macros::lua_l_error::luaL_error, records::match_state::MatchState};

/// cpp `lstrlib.cpp matchbalance`：`%b` 配对扫描，偏移游标版。
/// 返回 `Some(闭合符之后的 s 偏移)`；`None` 即原 NULL（失配/失衡）。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；
/// `s <= ms.src.len()`；`p + 1 >= ms.pat.len()` 的畸形模式经判界报错后不读界外，
/// 配对扫描窗口止于 `src.len()`（cpp `while (++s < ms->src_end)` 同界）。
pub(crate) unsafe fn matchbalance(ms: &mut MatchState, s: usize, p: usize) -> Option<usize> {
  // Safety: 契约保证 p 位于 pattern 串界内且初始字符可读，配对扫描至串尾自然终止
  unsafe {
    // cpp: if (p >= ms->p_end - 1) —— 指针差值判定等价为偏移 p + 1 >= pat.len()
    if p + 1 >= ms.pat.len() {
      luaL_error!(ms.l, "malformed pattern (missing arguments to '%b')");
    }
    // s == src.len() 时读终止 NUL（cpp 读 *s 同点位）
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
}
