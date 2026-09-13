use alloc::string::String;

pub fn get_parent_path(path: &str) -> Option<String> {
  if path.is_empty() || path == "." || path == "/" {
    return None;
  }

  #[cfg(windows)]
  {
    if path.len() == 2 && path.as_bytes().get(1) == Some(&b':') {
      return None;
    }
  }

  let last_slash_pos = path.rfind(['\\', '/']);

  match last_slash_pos {
    Some(0) => Some("/".to_string()),
    Some(slash_index) => Some(path[..slash_index].to_string()),
    None => {
      #[cfg(windows)]
      {
        if path.len() == 2 && path.as_bytes().get(1) == Some(&b':') {
          None
        } else {
          Some("".to_string())
        }
      }
      #[cfg(not(windows))]
      {
        Some("".to_string())
      }
    }
  }
}
