pub fn skip_fast_flag(flag_name: &str) -> bool {
  if flag_name.starts_with("Test") {
    return true;
  }

  if flag_name.starts_with("Debug") {
    return true;
  }

  false
}
