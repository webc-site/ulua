use core::{
  ffi::c_char,
  ptr::{eq, null_mut},
};

use ulua_common::strtoull_shim::rust_strtoull;

use crate::macros::{cast_num::cast_num, luai_str_2_num::luai_str2num};

/// cpp `luaO_str2d`（lobject.cpp:86）：把 C 字符串解析为整数/浮点数。
///
/// cpp 用 `lua_Number* result` 出参 + `bool` 返回值，Rust 版折叠为 `Option<f64>`。
///
/// # Safety
///
/// `s` 必须指向以 NUL 结尾的有效缓冲区。
pub(crate) unsafe fn lua_o_str_2_d(s: *const c_char) -> Option<f64> {
  let mut endptr: *mut c_char = null_mut();
  let mut result = unsafe { luai_str2num!(s, &mut endptr) };
  if eq(endptr, s) {
    return None; // conversion failed
  }
  unsafe {
    if *endptr == b'x' as c_char || *endptr == b'X' as c_char {
      // maybe an hexadecimal constant?
      result = cast_num!(rust_strtoull(s, &mut endptr, 16));
    }
    if *endptr == b'\0' as c_char {
      return Some(result); // most common case
    }
    while isspace(*endptr as u8) {
      endptr = endptr.add(1);
    }
    if *endptr != b'\0' as c_char {
      return None; // invalid trailing characters?
    }
  }
  Some(result)
}

// Helper for isspace: check if a u8 value corresponds to an ASCII whitespace character
#[inline]
fn isspace(c: u8) -> bool {
  matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

#[cfg(test)]
mod tests {
  use core::ffi::CStr;

  use super::*;

  /// 对照 cpp luaO_str2d（lobject.cpp:86）的接受/拒绝边界
  fn str2d(s: &CStr) -> Option<f64> {
    unsafe { lua_o_str_2_d(s.as_ptr()) }
  }

  #[test]
  fn accepts_decimal_with_trailing_space() {
    assert_eq!(str2d(c"42"), Some(42.0));
    assert_eq!(str2d(c" 3.5 "), Some(3.5));
    assert_eq!(str2d(c"-0.0"), Some(0.0)); // 值比较下 -0.0 == 0.0
    assert!(str2d(c"-0.0").unwrap().is_sign_negative());
    assert_eq!(str2d(c"1e3"), Some(1000.0));
  }

  #[test]
  fn accepts_hex_when_strtod_stops_at_x() {
    // strtod 只吃 "0"，hex 路径由 strtoull 接管（与无 hex-float 的 C 行为一致）
    assert_eq!(str2d(c"0x1f"), Some(31.0));
    assert_eq!(str2d(c"0X10"), Some(16.0));
    assert_eq!(str2d(c"0"), Some(0.0));
    assert_eq!(str2d(c"0x"), None); // strtoull 停在 x，trailing 非法
  }

  #[test]
  fn rejects_non_numeric_and_trailing_junk() {
    assert_eq!(str2d(c""), None);
    assert_eq!(str2d(c"abc"), None);
    assert_eq!(str2d(c"12abc"), None); // 尾随非空白 → 拒绝
    assert_eq!(str2d(c".e5"), None); // 无数字的伪浮点
    assert_eq!(str2d(c"  "), None);
  }

  #[test]
  fn accepts_c_inf_nan_spellings() {
    assert_eq!(str2d(c"inf"), Some(f64::INFINITY));
    assert_eq!(str2d(c"-infinity"), Some(f64::NEG_INFINITY));
    assert!(str2d(c"nan").unwrap().is_nan());
  }
}
