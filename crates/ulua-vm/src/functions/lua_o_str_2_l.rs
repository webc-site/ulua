use core::{
  ffi::c_char,
  ptr::{eq, null_mut},
};

use ulua_common::strtoull_shim::rust_strtoull;

use crate::macros::luai_str_2_long::strtoll;

/// cpp `luaO_str2l`：按 `base` 把 C 字符串解析为 64 位整数。
///
/// cpp 用 `lua_Integer64* result` 出参 + `bool` 返回值，Rust 版折叠为 `Option<i64>`。
///
/// # Safety
///
/// `s` 必须指向以 NUL 结尾的有效缓冲区。
pub(crate) unsafe fn lua_o_str_2_l(s: *const c_char, base: i32) -> Option<i64> {
  let mut endptr: *mut c_char = null_mut();
  let mut result: i64;

  unsafe {
    if base == 10 {
      result = strtoll(s, &mut endptr, base) as i64;
      if eq(endptr, s) {
        return None; // conversion failed
      }
      if *endptr == b'x' as c_char || *endptr == b'X' as c_char {
        // maybe an hexadecimal constant?
        result = rust_strtoull(s, &mut endptr, 16) as i64;
      }
    } else {
      // unsigned parse in other bases
      result = rust_strtoull(s, &mut endptr, base as u32) as i64;
      if eq(endptr, s) {
        return None;
      }
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

// Helper for isspace: check if a u32 value corresponds to an ASCII whitespace character
#[inline]
fn isspace(c: u8) -> bool {
  matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}
