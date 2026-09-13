use alloc::string::String;

pub fn join_paths_string_view_string_view(lhs: &str, rhs: &str) -> String {
  let mut result = lhs.to_string();
  if !result.is_empty()
    && result.as_bytes().last() != Some(&b'/')
    && result.as_bytes().last() != Some(&b'\\')
  {
    result.push('/');
  }
  result.push_str(rhs);
  result
}
