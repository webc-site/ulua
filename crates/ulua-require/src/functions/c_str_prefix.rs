use alloc::ffi::CString;

/// 对应 cpp `std::string::c_str()`：按首个 NUL 截断。
/// 返回切片保证不含 NUL，`c_cstring` 构造必然成功。
pub(crate) fn c_str_prefix(s: &str) -> &str {
  // split 至少产出一个元素，恒安全
  s.split('\0').next().unwrap()
}

/// 对应 cpp 把 `std::string` 传给 C API 的 `c_str()` 语义：先按首个 NUL 截断
/// 再转 `CString`；截断后必无 NUL，构造不会失败。
pub(crate) fn c_cstring(s: &str) -> CString {
  // SAFETY 恒成立：c_str_prefix 保证切片不含 NUL
  CString::new(c_str_prefix(s)).unwrap()
}
