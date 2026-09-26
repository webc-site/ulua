use core::ffi::c_char;

use crate::functions::printunsignedrev::K_DIGIT_TABLE;

/// cpp `lnumprint.cpp:printexp`：向 `buf` 写 `e±d[dd]` 形式的指数部分。
///
/// 返回写出长度；`buf` 须可写至多 5 字节（`e` + 符号 + 最多 3 位数字）。
pub(crate) fn printexp(buf: &mut [c_char], num: i32) -> usize {
  buf[0] = b'e' as c_char;
  buf[1] = if num < 0 { b'-' } else { b'+' } as c_char;

  let mut v = if num < 0 { -num } else { num };
  let mut pos = 2;

  if v >= 100 {
    buf[pos] = (b'0' + (v / 100) as u8) as c_char;
    pos += 1;
    v %= 100;
  }

  // 与 printunsignedrev 共用的两位数字查找表，取 `v`（< 100）对应 2 字节
  let digits = &K_DIGIT_TABLE[(v as usize) * 2..(v as usize) * 2 + 2];
  buf[pos] = digits[0] as c_char;
  buf[pos + 1] = digits[1] as c_char;

  pos + 2
}
