use core::{ffi::c_char, ptr::null, slice::from_raw_parts};

const MAX_UNICODE: u32 = 0x10FFFF;
const LIMITS: [u32; 4] = [0xFF, 0x7F, 0x7FF, 0xFFFF];

/// 纯 safe 的 Rust 切片 UTF-8 解码函数。
///
/// 解析切片开头的单个 UTF-8 字符序列，返回 `Some((codepoint, bytes_consumed))`。
/// 若遇到无效字节序列、超长编码、代理区码点、超出 Unicode 范围或切片长度不足，则返回 `None`。
#[inline]
pub fn utf8_decode_bytes(bytes: &[u8]) -> Option<(u32, usize)> {
  let &first = bytes.first()?;
  let c = first as u32;

  if c < 0x80 {
    return Some((c, 1));
  }

  let mut count: usize = 0;
  let mut c_val = c;
  let mut res: u32 = 0;

  while (c_val & 0x40) != 0 {
    count += 1;
    let &cc = bytes.get(count)?;
    let cc = cc as u32;
    if (cc & 0xC0) != 0x80 {
      return None;
    }
    res = (res << 6) | (cc & 0x3F);
    c_val <<= 1;
  }

  res |= (c_val & 0x7F) << (count * 5);

  if count > 3 || res > MAX_UNICODE || res <= LIMITS[count] {
    return None;
  }

  if (res.wrapping_sub(0xD800)) < 0x800 {
    return None;
  }

  Some((res, count + 1))
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn utf_8_decode(o: *const c_char, val: *mut i32) -> *const c_char {
  if o.is_null() {
    return null();
  }

  let s = o as *const u8;
  unsafe {
    let first = *s;
    if first < 0x80 {
      if !val.is_null() {
        *val = first as i32;
      }
      return s.add(1) as *const c_char;
    }

    // 根据头部续字节标志位计算潜在的最大序列长度（最多 4 个续字节）
    let mut len = 1;
    let mut c_val = first as u32;
    while (c_val & 0x40) != 0 && len <= 4 {
      len += 1;
      c_val <<= 1;
    }

    let slice = from_raw_parts(s, len);
    match utf8_decode_bytes(slice) {
      Some((code, consumed)) => {
        if !val.is_null() {
          *val = code as i32;
        }
        s.add(consumed) as *const c_char
      }
      None => null(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_utf8_decode_ascii() {
    assert_eq!(utf8_decode_bytes(b"hello"), Some((b'h' as u32, 1)));
    assert_eq!(utf8_decode_bytes(b"\0"), Some((0, 1)));
    assert_eq!(utf8_decode_bytes(b""), None);
  }

  #[test]
  fn test_utf8_decode_multibyte() {
    // 2-byte: U+00A2 '¢' -> [0xC2, 0xA2]
    assert_eq!(utf8_decode_bytes(&[0xC2, 0xA2]), Some((0xA2, 2)));
    // 3-byte: U+20AC '€' -> [0xE2, 0x82, 0xAC]
    assert_eq!(utf8_decode_bytes(&[0xE2, 0x82, 0xAC]), Some((0x20AC, 3)));
    // 4-byte: U+1F600 '😀' -> [0xF0, 0x9F, 0x98, 0x80]
    assert_eq!(
      utf8_decode_bytes(&[0xF0, 0x9F, 0x98, 0x80]),
      Some((0x1F600, 4))
    );
  }

  #[test]
  fn test_utf8_decode_invalid() {
    // Truncated sequence
    assert_eq!(utf8_decode_bytes(&[0xC2]), None);
    assert_eq!(utf8_decode_bytes(&[0xE2, 0x82]), None);
    // Invalid continuation byte
    assert_eq!(utf8_decode_bytes(&[0xC2, 0x00]), None);
    // Overlong encoding: U+0020 encoded as 2 bytes [0xC0, 0xA0]
    assert_eq!(utf8_decode_bytes(&[0xC0, 0xA0]), None);
    // Surrogate: U+D800 -> [0xED, 0xA0, 0x80]
    assert_eq!(utf8_decode_bytes(&[0xED, 0xA0, 0x80]), None);
    // Beyond MAX_UNICODE: > 0x10FFFF
    assert_eq!(utf8_decode_bytes(&[0xF4, 0x90, 0x80, 0x80]), None);
  }
}
