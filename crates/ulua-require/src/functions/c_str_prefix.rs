/// 对应 cpp `std::string::c_str()`：按首个 NUL 截断。
/// 返回切片保证不含 NUL，`CString::new` 必然成功。
pub(crate) fn c_str_prefix(s: &str) -> &str {
  s.split('\0').next().unwrap()
}
