use alloc::vec::Vec;

/// 按 '/' 或 '\' 切分；与 cpp `splitPath` 一致，保留空段（如前导分隔符产生 ""）
pub fn split_path(path: &str) -> Vec<&str> {
  path.split(['\\', '/']).collect()
}

#[cfg(test)]
mod tests {
  use alloc::vec;

  #[test]
  fn splits_on_both_separators() {
    assert_eq!(super::split_path("a/b\\c"), vec!["a", "b", "c"]);
    assert_eq!(super::split_path("/abs"), vec!["", "abs"]);
    assert_eq!(super::split_path("trailing/"), vec!["trailing", ""]);
    assert_eq!(super::split_path(""), vec![""]);
  }
}
