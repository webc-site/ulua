use alloc::string::String;

/// cpp `joinPaths(std::string& out, lhs, rhs)`：拼接两段路径，仅在 lhs 尾部与
/// rhs 首部都不是分隔符时才补 `/`。
///
/// cpp 把首参当输出用，Rust 版直接返回拼接结果。
pub fn join_paths_basic_string_ch_ch_ch(lhs: &str, rhs: &str) -> String {
  let mut result = lhs.to_string();
  if !result.is_empty()
    && !result.ends_with(['/', '\\'])
    && !rhs.is_empty()
    && !rhs.starts_with(['/', '\\'])
  {
    result.push('/');
  }
  result.push_str(rhs);
  result
}
