use alloc::string::String;

pub fn join_paths_string_view_string_view(lhs: &str, rhs: &str) -> String {
  let mut result = lhs.to_string();
  // cpp 无条件在 lhs 尾部非分隔符时补 '/'（rhs 前导分隔符不参与判定）
  if !result.is_empty() && !result.ends_with(['/', '\\']) {
    result.push('/');
  }
  result.push_str(rhs);
  result
}
