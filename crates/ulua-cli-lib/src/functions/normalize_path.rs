use alloc::{string::String, vec::Vec};

use crate::functions::{is_absolute_path::is_absolute_path, split_path::split_path};

pub fn normalize_path(path: &str) -> String {
  let components: Vec<&str> = split_path(path);
  let mut normalized_components: Vec<&str> = Vec::new();

  let is_absolute = is_absolute_path(path);

  // 1. Normalize path components
  for component in components.iter().copied().skip(usize::from(is_absolute)) {
    if component == ".." {
      if normalized_components.is_empty() {
        if !is_absolute {
          normalized_components.push("..");
        }
      } else if normalized_components.last() == Some(&"..") {
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
  } else if normalized_components.first() != Some(&"..") {
    normalized_path.push_str("./");
  }

  // 3. 归一化后的段以 '/' 连接（切片版 `join` 即"元素间插分隔符"，无需手工
  //    判断首元素）
  normalized_path.push_str(&normalized_components.join("/"));

  // 尾部 ".." 追加 '/', 镜像 cpp
  if normalized_path.as_bytes().ends_with(b"..") {
    normalized_path.push('/');
  }

  normalized_path
}
