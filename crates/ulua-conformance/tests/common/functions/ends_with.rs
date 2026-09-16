pub fn ends_with(str: &str, suffix: &str) -> bool {
  if suffix.len() > str.len() {
    return false;
  }

  let start = str.len() - suffix.len();
  &str[start..] == suffix
}
