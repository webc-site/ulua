//! cpp `traverseDirectory` + 静态 `traverseDirectoryRec`（CLI/src/ FileUtils.cpp）
use std::{fs::read_dir, path::Path};

/// 递归收集 `path` 下全部文件，回调收到完整路径；目录不可读返回 false
pub fn traverse_directory(path: &str, mut callback: impl FnMut(&str)) -> bool {
  traverse_directory_rec(path, &mut callback)
}

fn traverse_directory_rec(path: &str, callback: &mut impl FnMut(&str)) -> bool {
  let path_obj = Path::new(path);
  let Ok(entries) = read_dir(path_obj) else {
    return false;
  };

  for entry in entries.flatten() {
    // read_dir 已排除 "."/".."，保留防御检查镜像 cpp 行为
    let file_name = entry.file_name();
    let name_str = file_name.to_string_lossy();
    if name_str == "." || name_str == ".." {
      continue;
    }

    let Ok(file_type) = entry.file_type() else {
      continue;
    };
    let full_path = path_obj.join(&file_name);
    let path_str = full_path.to_string_lossy();

    if file_type.is_dir() && !traverse_directory_rec(path_str.as_ref(), callback) {
      return false;
    } else if file_type.is_file() {
      callback(path_str.as_ref());
    }
  }
  true
}
