use core::ffi::{CStr, c_char};

use ulua_ast::records::location::Location;

/// 报告错误信息
///
/// # Safety
///
/// 传入的 `name`、`r#type` 与 `message` 必须为有效且以 NUL 结尾的 C 字符串指针。
pub unsafe fn report(
  name: *const c_char,
  location: &Location,
  r#type: *const c_char,
  message: *const c_char,
) {
  let name_str = unsafe { CStr::from_ptr(name) }.to_string_lossy();
  let type_str = unsafe { CStr::from_ptr(r#type) }.to_string_lossy();
  let message_str = unsafe { CStr::from_ptr(message) }.to_string_lossy();

  eprintln!(
    "{}({},{}): {}: {}",
    name_str,
    location.begin.line + 1,
    location.begin.column + 1,
    type_str,
    message_str
  );
}
