use core::ffi::{c_char, c_int};

#[cfg(not(target_arch = "wasm32"))]
unsafe extern "C" {
  pub fn strtoll(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn strtoll(
  _s: *const core::ffi::c_char,
  _endptr: *mut *mut core::ffi::c_char,
  _base: core::ffi::c_int,
) -> i64 {
  0
}
