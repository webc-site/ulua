use core::ffi::{CStr, c_char};

use crate::functions::debugger_present::debugger_present;
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn test_assertion_handler(
  expr: *const c_char,
  file: *const c_char,
  line: i32,
  function: *const c_char,
) -> i32 {
  if debugger_present() {
    return 1;
  }

  let expr_str = if expr.is_null() {
    ""
  } else {
    unsafe { CStr::from_ptr(expr).to_str().unwrap_or("") }
  };
  let file_str = if file.is_null() {
    ""
  } else {
    unsafe { CStr::from_ptr(file).to_str().unwrap_or("") }
  };
  let function_str = if function.is_null() {
    ""
  } else {
    unsafe { CStr::from_ptr(function).to_str().unwrap_or("") }
  };

  panic!(
    "Assertion failed: {} at {}:{} in {}",
    expr_str, file_str, line, function_str
  );
}
