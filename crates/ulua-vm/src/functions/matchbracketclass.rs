use crate::{functions::match_class::match_class, macros::l_esc::L_ESC};

/// cpp `VM/src/lstrlib.cpp:matchbracketclass`：判断字符 `c` 是否落在字符类内。
///
/// `class` 自 `[` 起、含结尾 `]`（原版 `ec` 所指字节：转义分支 `%` 紧贴尾 `]`
/// 时 cpp 会读该字节，须留在切片界内）。`class[0]` 为 `[`，`class[1..ec]` 为
/// 类内容（可含 `^` 取反、`%x` 转义类与 `a-z` 区间）。
pub(crate) fn matchbracketclass(c: i32, class: &[u8]) -> i32 {
  // 原版 `ec` 的下标（`]` 所在处），比较界与 cpp 的 `p + 1 < ec` 逐点对齐
  let ec = class.len() - 1;
  let mut sig: i32 = 1;
  let mut i: usize = 0;

  if class.get(1) == Some(&b'^') {
    sig = 0;
    i = 1; // skip the `^`
  }

  while i + 1 < ec {
    i += 1;

    if class[i] == L_ESC as u8 {
      i += 1; // 最远落在 `ec`（']' 字节），与 cpp 读 *ec 行为一致
      if match_class(c, class[i] as i32) != 0 {
        return sig;
      }
    } else if class[i + 1] == b'-' && i + 2 < ec {
      i += 2;
      if (class[i - 2] as i32) <= c && c <= (class[i] as i32) {
        return sig;
      }
    } else if (class[i] as i32) == c {
      return sig;
    }
  }

  if sig == 0 { 1 } else { 0 }
}
