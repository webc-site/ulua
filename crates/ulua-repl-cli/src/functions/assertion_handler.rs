use core::ffi::{CStr, c_char, c_int};

/// C++ static function: assertionHandler
/// Prints assertion failure message to stdout and returns 1.
pub unsafe fn assertion_handler(
  expr: *const c_char,
  file: *const c_char,
  line: c_int,
  _function: *const c_char,
) -> i32 {
  unsafe {
    let expr_str = if expr.is_null() {
      ""
    } else {
      CStr::from_ptr(expr).to_str().unwrap_or("")
    };
    let file_str = if file.is_null() {
      ""
    } else {
      CStr::from_ptr(file).to_str().unwrap_or("")
    };
    println!("{}({}): ASSERTION FAILED: {}", file_str, line, expr_str);
    1
  }
}
