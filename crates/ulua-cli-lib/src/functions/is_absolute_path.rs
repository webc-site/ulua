pub fn is_absolute_path(path: &str) -> bool {
  #[cfg(windows)]
  {
    let bytes = path.as_bytes();
    (bytes.len() >= 3
      && bytes[0].is_ascii_alphabetic()
      && bytes[1] == b':'
      && (bytes[2] == b'/' || bytes[2] == b'\\'))
      || (bytes.len() >= 1 && (bytes[0] == b'/' || bytes[0] == b'\\'))
  }
  #[cfg(not(windows))]
  {
    let bytes = path.as_bytes();
    !bytes.is_empty() && bytes[0] == b'/'
  }
}
