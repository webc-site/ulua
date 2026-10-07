use crate::{functions::match_class::match_class, macros::l_esc::L_ESC};

/// cpp `VM/src/lstrlib.cpp:matchbracketclass`：判断字符 `c` 是否落在字符类内。
///
/// `class` 自 `[` 起、含结尾 `]`（原版 `ec` 所指字节：转义分支 `%` 紧贴尾 `]`
/// 时 cpp 会读该字节，须留在切片界内）。`class[0]` 为 `[`，`class[1..ec]` 为
/// 类内容（可含 `^` 取反、`%x` 转义类与 `a-z` 区间）。
///
/// 读面：`as_ptr` 走读取代逐字节 `class[i]` 边界检查。循环不变式保证界内——
/// 进入循环体时 `i + 1 < ec`（循环条件），故体首读下标 `i ≤ ec - 1`、`i + 1 ≤
/// ec = len - 1`（`i = ec - 1` 时读 `]` 字节本身，与 cpp `*(p + 1)` 在 `p = ec - 1`
/// 处读 `*ec` 同点位）；转义臂 `i += 1` 后读下标 `≤ ec = len - 1`；区间臂前置
/// `i + 2 < ec`，消费后读下标 `≤ ec - 1`。
pub(crate) fn matchbracketclass(c: i32, class: &[u8]) -> i32 {
  // 原版 `ec` 的下标（`]` 所在处），比较界与 cpp 的 `p + 1 < ec` 逐点对齐
  let ec = class.len() - 1;
  let base = class.as_ptr();
  let mut sig: i32 = 1;
  let mut i: usize = 0;

  if class.get(1) == Some(&b'^') {
    sig = 0;
    i = 1; // skip the `^`
  }

  while i + 1 < ec {
    i += 1;
    // SAFETY: i ≤ ec - 1 = len - 2（循环不变式），界内
    let b = unsafe { *base.add(i) };

    if b == L_ESC as u8 {
      i += 1; // 最远落在 `ec`（']' 字节），与 cpp 读 *ec 行为一致
      // SAFETY: i ≤ ec = len - 1，界内
      if match_class(c, unsafe { *base.add(i) } as i32) != 0 {
        return sig;
      }
    } else {
      // SAFETY: i + 1 ≤ ec = len - 1（循环不变式；i = ec - 1 时读 ']' 字节，
      // 与 cpp 读 *(p+1) = *ec 同点位），界内
      if unsafe { *base.add(i + 1) } == b'-' && i + 2 < ec {
        i += 2;
        // SAFETY: 前置 i + 2 < ec ⇒ i - 2 与 i 均 ≤ ec - 1 < len，界内
        if unsafe { *base.add(i - 2) } as i32 <= c && c <= unsafe { *base.add(i) } as i32 {
          return sig;
        }
      } else if b as i32 == c {
        return sig;
      }
    }
  }

  if sig == 0 { 1 } else { 0 }
}
