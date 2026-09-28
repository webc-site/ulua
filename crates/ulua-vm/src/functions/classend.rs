use crate::{
  macros::{l_esc::L_ESC, lua_l_error::luaL_error},
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp classend`：返回模式单元素（转义对或字符类）之后的偏移。
/// `p` 为 pattern 偏移，须 `p < ms.pat.len()`（分派读 `pat_byte(p)`）。
///
/// 前置条件：`ms` 由 `prepstate` 建立、仍处本次匹配调用中（畸形模式经 `luaL_error`
/// 抛错并 unwind，要求 `ms.l` 存活可抛错）；读终止 NUL 仅发生在 cpp 同点位
/// （`p_end` 判界先于读）。字节读取全部走 `MatchState` 的安全切片门面，故签名安全，
/// `unsafe` 仅留在各报错内核。
pub(crate) fn classend(ms: &mut MatchState, p: usize) -> usize {
  let mut q = p + 1; // cpp `switch (*p++)`
  match ms.pat_byte(p) {
    x if x == L_ESC as u8 => {
      if q == ms.pat.len() {
        // SAFETY: 对象不变式（`prepstate` 建立的存活 `lua_State`）保证 `ms.l` 可抛错
        unsafe { luaL_error!(ms.l, "malformed pattern (ends with '%')") };
      }
      q + 1
    }
    b'[' => {
      if ms.pat_byte(q) == b'^' {
        q += 1;
      }
      loop {
        // cpp do-while：先判 `p == ms->p_end` 报错，再消费一字节
        if q == ms.pat.len() {
          // SAFETY: 同上，`ms.l` 为可抛错的存活 `lua_State`
          unsafe { luaL_error!(ms.l, "malformed pattern (missing ']')") };
        }
        let c = ms.pat_byte(q);
        q += 1;
        if c == L_ESC as u8 && q < ms.pat.len() {
          q += 1; // skip escapes (e.g. `%]')
        }
        // q == pat.len() 时读到终止 NUL，必不为 ']'，下一轮由上方判界报错（cpp 同点位）
        if ms.pat_byte(q) == b']' {
          break;
        }
      }
      q + 1
    }
    _ => q,
  }
}
