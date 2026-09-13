use alloc::vec::Vec;

pub fn split_string_by_slashes(s: &str) -> Vec<&str> {
  s.split('/').collect()
}
