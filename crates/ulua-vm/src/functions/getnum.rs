use core::{ffi::c_char, slice::from_raw_parts};

use crate::{macros::lua_l_error::luaL_error, records::header::Header};

/// 上游 Luau MAXSSIZE：((INT_MAX >> 1) + 1)，即 1073741824
const MAX_SSIZE: i32 = 1073741824;

/// 纯 safe 的 Rust 切片数字解析函数。
///
/// 从 `bytes` 开头解析十进制整数，支持指定默认值 `df`。
/// 返回 `Ok((parsed_value, bytes_consumed))`。若数值超出上限则返回 `Err("size specifier is too large")`。
#[inline]
pub fn parse_getnum_bytes(bytes: &[u8], df: i32) -> Result<(i32, usize), &'static str> {
  let Some(&first) = bytes.first() else {
    return Ok((df, 0));
  };

  if !first.is_ascii_digit() {
    return Ok((df, 0));
  }

  let mut a: i32 = 0;
  let mut idx = 0;

  loop {
    let b = bytes[idx];
    let digit_val = (b - b'0') as i32;
    a = a * 10 + digit_val;
    idx += 1;

    if idx == bytes.len() || !bytes[idx].is_ascii_digit() || a > (i32::MAX - 9) / 10 {
      break;
    }
  }

  if a > MAX_SSIZE || (idx < bytes.len() && bytes[idx].is_ascii_digit()) {
    return Err("size specifier is too large");
  }

  Ok((a, idx))
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn getnum(h: *mut Header, fmt: *mut *const c_char, df: i32) -> i32 {
  unsafe {
    let fmt_ptr = *fmt;
    if fmt_ptr.is_null() {
      return df;
    }

    // 扫描连续数字字节，上限 32 字节足以判断正常数字与溢出
    let mut len = 0;
    while len < 32 && (*fmt_ptr.add(len) as u8).is_ascii_digit() {
      len += 1;
    }

    let slice = from_raw_parts(fmt_ptr as *const u8, len);
    match parse_getnum_bytes(slice, df) {
      Ok((val, consumed)) => {
        *fmt = fmt_ptr.add(consumed);
        val
      }
      Err(err) => {
        luaL_error!((*h).l, "{}", err);
        unreachable!()
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_getnum_bytes_default() {
    assert_eq!(parse_getnum_bytes(b"", 4), Ok((4, 0)));
    assert_eq!(parse_getnum_bytes(b"xyz", 4), Ok((4, 0)));
  }

  #[test]
  fn test_parse_getnum_bytes_digits() {
    assert_eq!(parse_getnum_bytes(b"4", -1), Ok((4, 1)));
    assert_eq!(parse_getnum_bytes(b"16", -1), Ok((16, 2)));
    assert_eq!(parse_getnum_bytes(b"16xyz", -1), Ok((16, 2)));
    assert_eq!(parse_getnum_bytes(b"0", -1), Ok((0, 1)));
  }

  #[test]
  fn test_parse_getnum_bytes_overflow() {
    assert!(parse_getnum_bytes(b"99999999999999", -1).is_err());
    assert!(parse_getnum_bytes(b"1073741825", -1).is_err());
    assert_eq!(parse_getnum_bytes(b"1073741824", -1), Ok((1073741824, 10)));
  }
}
