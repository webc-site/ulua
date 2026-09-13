use alloc::{string::String, vec::Vec};

use crate::functions::{is_absolute_path::is_absolute_path, split_path::split_path};

pub fn normalize_path(path: &str) -> String {
  let components: Vec<&str> = split_path(path);
  let mut normalized_components: Vec<&str> = Vec::new();

  let is_absolute = is_absolute_path(path);

  // 1. Normalize path components
  for component in components.iter().skip(usize::from(is_absolute)).copied() {
    if component == ".." {
      if normalized_components.is_empty() {
        if !is_absolute {
          normalized_components.push("..");
        }
      } else if *normalized_components.last().unwrap() == ".." {
        normalized_components.push("..");
      } else {
        normalized_components.pop();
      }
    } else if !component.is_empty() && component != "." {
      normalized_components.push(component);
    }
  }

  let mut normalized_path = String::new();

  // 2. Add correct prefix to formatted path
  if is_absolute {
    // Mirror the C++ behavior: assumes path is absolute and components[0] exists.
    normalized_path.push_str(components[0]);
    normalized_path.push('/');
  } else if normalized_components.is_empty() || normalized_components[0] != ".." {
    normalized_path.push_str("./");
  }

  // 3. Join path components to form the normalized path
  for (idx, component) in normalized_components.iter().enumerate() {
    if idx != 0 {
      normalized_path.push('/');
    }
    normalized_path.push_str(component);
  }

  let bytes = normalized_path.as_bytes();
  if bytes.len() >= 2 && bytes[bytes.len() - 1] == b'.' && bytes[bytes.len() - 2] == b'.' {
    normalized_path.push('/');
  }

  normalized_path
}
