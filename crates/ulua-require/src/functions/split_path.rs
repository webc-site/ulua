pub fn split_path(path: &str) -> (&str, &str) {
  path.split_once('/').unwrap_or((path, ""))
}
