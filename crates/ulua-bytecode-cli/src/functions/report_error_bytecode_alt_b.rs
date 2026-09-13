use core::ffi::c_char;

use ulua_compiler::records::compile_error::CompileError;

use crate::functions::report::report;

/// 报告编译错误
///
/// # Safety
///
/// `name` 必须为有效且以 NUL 结尾的 C 字符串指针。
pub unsafe fn report_error_c_char_luau_compile_error(name: *const c_char, error: &CompileError) {
  unsafe {
    report(
      name,
      error.get_location(),
      c"CompileError".as_ptr(),
      error.what(),
    );
  }
}
