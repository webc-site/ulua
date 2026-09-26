//! cpp `traverseDirectory` + 静态 `traverseDirectoryRec`（CLI/src/ FileUtils.cpp）
use std::{fs::read_dir, path::Path};

use crate::functions::join_paths_file_utils::join_paths;

/// 递归收集 `path` 下全部文件，回调收到完整路径；仅顶层目录不可读时返回 false
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
    // cpp joinPaths 条件追加，收口到 FileUtils 的公有 join_paths（rhs_sep_matters
    // = true 对应此处的模板重载语义）。两实现唯一差异是公有版多一个「lhs 非空」
    // 判定——空 lhs 在本可达域不可达：`read_dir("")` 必失败并在进入首个拼接前
    // 早退，递归的 lhs 又都来自非空拼接结果。
    let full_path = join_paths(&path_obj.to_string_lossy(), &name_str, true);

    if file_type.is_dir() {
      // cpp `traverseDirectoryRec` 丢弃递归返回值（FileUtils.cpp:260,318）：
      // 子目录不可读只跳过该子树，不得中断同层/父级遍历
      let _ = traverse_directory_rec(full_path.as_ref(), callback);
    } else if file_type.is_file() {
      callback(full_path.as_ref());
    }
  }
  true
}
