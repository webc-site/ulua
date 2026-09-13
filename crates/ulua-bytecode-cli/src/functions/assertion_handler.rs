use core::ffi::{CStr, c_char};

/// 断言失败回调处理器
///
/// # Safety
///
/// 传入的原始指针必须为有效且以 NUL 结尾的 C 字符串指针。
pub unsafe fn assertion_handler(
  expr: *const c_char,
  file: *const c_char,
  line: i32,
  _function: *const c_char,
) -> i32 {
  let file_str = unsafe { CStr::from_ptr(file) }.to_string_lossy();
  let expr_str = unsafe { CStr::from_ptr(expr) }.to_string_lossy();

  println!("{}({}): ASSERTION FAILED: {}", file_str, line, expr_str);
  1
}
