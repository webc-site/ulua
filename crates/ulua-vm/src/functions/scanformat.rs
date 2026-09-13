use core::{ffi::c_char, ptr, ptr::copy_nonoverlapping, slice::from_raw_parts};

use crate::{macros::lua_l_error::luaL_error, type_aliases::lua_state::lua_State};

/// 纯 safe 的 Rust 切片格式说明符扫描函数。
///
/// 从 `bytes`（紧跟 `%` 之后的字节切片）开始解析：
/// - 标志位（`FLAGS = b"-+ #0"`）：每个标志最多出现一次，总长不得超过 FLAGS.len()；
/// - 字段宽度（最多 2 位 ASCII 数字）；
/// - 精度（可选的 `.` 紧随最多 2 位 ASCII 数字）；
///
/// 若遇到多余数字或重复标志，返回 `Err(&'static str)`；
/// 成功则返回 `Ok(spec_len)`，其中 `bytes[..spec_len]` 为格式选项，`bytes[spec_len]` 为转换指示符（如 'd', 's' 等）。
#[inline]
pub fn scan_format_spec(bytes: &[u8]) -> Result<usize, &'static str> {
  const FLAGS: &[u8] = b"-+ #0";

  let mut p = 0;
  while p < bytes.len() && bytes[p] != 0 && FLAGS.contains(&bytes[p]) {
    p += 1;
  }

  // C++: (size_t)(p - strfrmt) >= sizeof(FLAGS), 其中 sizeof(FLAGS) 为 6 (包含 '\0')，即最多 5 个不同 flag
  if p > FLAGS.len() {
    return Err("invalid format (repeated flags)");
  }

  if p < bytes.len() && bytes[p].is_ascii_digit() {
    p += 1;
  }
  if p < bytes.len() && bytes[p].is_ascii_digit() {
    p += 1;
  }

  if p < bytes.len() && bytes[p] == b'.' {
    p += 1;
    if p < bytes.len() && bytes[p].is_ascii_digit() {
      p += 1;
    }
    if p < bytes.len() && bytes[p].is_ascii_digit() {
      p += 1;
    }
  }

  if p < bytes.len() && bytes[p].is_ascii_digit() {
    return Err("invalid format (width or precision too long)");
  }

  Ok(p)
}

#[unsafe(export_name = "ulua_scanformat")]
pub(crate) unsafe fn scanformat(
  l: *mut lua_State,
  strfrmt: *const c_char,
  mut form: *mut c_char,
  size: *mut usize,
) -> *const c_char {
  unsafe {
    // 扫描到 NUL 或最多 16 字节（足以覆盖 flags(5) + width(2) + '.'(1) + prec(2) + indicator(1) + 溢出数字(1)）
    let mut len = 0;
    while len < 16 && *strfrmt.add(len) != 0 {
      len += 1;
    }

    let slice = from_raw_parts(strfrmt as *const u8, len);
    match scan_format_spec(slice) {
      Ok(p_offset) => {
        let p = strfrmt.add(p_offset);
        ptr::write(form, b'%' as c_char);
        form = form.offset(1);
        *size = p_offset + 1;
        copy_nonoverlapping(strfrmt, form, *size);
        form = form.add(*size);
        ptr::write(form, 0);
        p
      }
      Err(err) => {
        luaL_error!(l, "{}", err);
        unreachable!()
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_scan_format_spec_valid() {
    assert_eq!(scan_format_spec(b"d"), Ok(0));
    assert_eq!(scan_format_spec(b"5d"), Ok(1));
    assert_eq!(scan_format_spec(b"12d"), Ok(2));
    assert_eq!(scan_format_spec(b".5d"), Ok(2));
    assert_eq!(scan_format_spec(b"12.34d"), Ok(5));
    assert_eq!(scan_format_spec(b"-+ #012.34d"), Ok(10));
  }

  #[test]
  fn test_scan_format_spec_invalid() {
    // Repeated flags: > 5 flags
    assert_eq!(
      scan_format_spec(b"-+-+-+d"),
      Err("invalid format (repeated flags)")
    );
    // Width > 2 digits
    assert_eq!(
      scan_format_spec(b"123d"),
      Err("invalid format (width or precision too long)")
    );
    // Precision > 2 digits
    assert_eq!(
      scan_format_spec(b".123d"),
      Err("invalid format (width or precision too long)")
    );
  }
}
