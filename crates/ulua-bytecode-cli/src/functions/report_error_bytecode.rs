use core::ffi::c_char;

use ulua_ast::records::parse_error::ParseError;

use crate::functions::report::report;

/// 报告解析错误
///
/// # Safety
///
/// `name` 必须为有效且以 NUL 结尾的 C 字符串指针。
pub unsafe fn report_error_c_char_luau_parse_error(name: *const c_char, error: &ParseError) {
  unsafe {
    report(
      name,
      error.get_location(),
      c"SyntaxError".as_ptr(),
      error.what().as_ptr() as *const c_char,
    );
  }
}
