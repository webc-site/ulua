use core::ffi::{CStr, c_char, c_int};
use std::io::stdout;
/// # Safety
/// 调用方须保证 C 字符串指针有效（C 运行时调用契约）。
#[unsafe(export_name = "ulua_assertion_handler")]
pub unsafe extern "C-unwind" fn assertion_handler(
  expr: *const c_char,
  file: *const c_char,
  line: c_int,
  _function: *const c_char,
) -> c_int {
  let file_str = unsafe { CStr::from_ptr(file) }.to_string_lossy();
  let expr_str = unsafe { CStr::from_ptr(expr) }.to_string_lossy();
  // Using libc printf equivalent via std::io::Write to stdout
  use std::io::Write;
  let _ = writeln!(
    stdout(),
    "{}({}): ASSERTION FAILED: {}",
    file_str,
    line,
    expr_str
  );
  let _ = stdout().flush();
  1
}
