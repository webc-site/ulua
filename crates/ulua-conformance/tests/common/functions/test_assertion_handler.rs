use core::ffi::{CStr, c_char, c_int};

use crate::common::functions::debugger_present::debugger_present;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn test_assertion_handler(
  expr: *const c_char,
  file: *const c_char,
  line: c_int,
  _function: *const c_char,
) -> c_int {
  if debugger_present() {
    return 1;
  }

  let expr_str = if expr.is_null() {
    ""
  } else {
    unsafe { CStr::from_ptr(expr) }.to_str().unwrap_or("")
  };
  let file_str = if file.is_null() {
    ""
  } else {
    unsafe { CStr::from_ptr(file) }.to_str().unwrap_or("")
  };

  eprintln!("{file_str}:{line}: Failure: Assertion failed: {expr_str}");
  1
}
