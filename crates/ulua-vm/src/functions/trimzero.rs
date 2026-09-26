use core::ffi::c_char;

/// 从 `end` 起向左跳过已写数字的尾 `'0'`，返回修剪后的结尾位置。
///
/// 前置条件：`buf[..end]` 内至少有一个非 `'0'` 字符（调用方保证：schubfach 有效
/// 数字首位恒非 `'0'`），故循环在界内终止；`end == 0` 的越界情形按幂等处理。
#[inline]
pub(crate) fn trimzero(buf: &[c_char], mut end: usize) -> usize {
  while end > 0 && buf[end - 1] == b'0' as c_char {
    end -= 1;
  }
  end
}
