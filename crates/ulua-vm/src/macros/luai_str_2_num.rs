use core::ffi::{CStr, c_char};

use ulua_common::strtod_shim::parse_c_double;

#[macro_export]
macro_rules! luai_str2num {
  ($s:expr, $p:expr) => {
    $crate::macros::luai_str_2_num::rust_strtod($s, $p)
  };
}

/// 纯 Rust strtod 替代：用 fast-float2 驱动的 parse_c_double 取代 libc strtod。
///
/// 全平台统一路径，消除 native/wasm 分歧。保持 C strtod 的 endptr 语义。
///
/// # Safety
/// `s` 必须指向以 NUL 结尾的缓冲区；`endptr` 非空时必须可写。
pub unsafe fn rust_strtod(s: *const c_char, endptr: *mut *mut c_char) -> f64 {
  unsafe {
    if s.is_null() {
      if !endptr.is_null() {
        *endptr = s.cast_mut();
      }
      return 0.0;
    }
    let bytes = CStr::from_ptr(s).to_bytes();
    let (val, consumed) = parse_c_double(bytes);
    if !endptr.is_null() {
      *endptr = (s as *mut c_char).add(consumed);
    }
    val
  }
}

pub use luai_str2num;
