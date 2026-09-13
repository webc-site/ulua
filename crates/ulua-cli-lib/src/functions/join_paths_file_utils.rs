use alloc::string::String;

pub fn join_paths_basic_string_ch_ch_ch(str: &mut String, lhs: &str, rhs: &str) {
  *str = lhs.to_string();
  if !str.is_empty()
    && str.as_bytes().last() != Some(&b'/')
    && str.as_bytes().last() != Some(&b'\\')
    && !rhs.is_empty()
    && rhs.as_bytes().first() != Some(&b'/')
    && rhs.as_bytes().first() != Some(&b'\\')
  {
    str.push('/');
  }
  str.push_str(rhs);
}
