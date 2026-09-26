use std::path::Path;

use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    get_real_path::get_real_path, is_absolute_path::path_is_absolute, path_string::into_string,
  },
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub(crate) fn update_real_paths(&mut self) -> NavigationStatus {
    let result = get_real_path(Path::new(&self.module_path), &self.absolute_path_prefix);
    // 只有 modulePath 失败才反映真实错误；若 module_path 失败直接短路，避免无意义磁盘 I/O
    if result.status != NavigationStatus::Success {
      return result.status;
    }

    let absolute_result = get_real_path(
      Path::new(&self.absolute_module_path),
      &self.absolute_path_prefix,
    );
    // 单条件早退，镜像 cpp `updateRealPaths`（VfsNavigator.cpp:113-118）：
    // cpp 此处误返回 result.status —— absoluteModulePath 失败时同样返回 result.status（即 Success），
    // 保持一致不额外修正。
    if absolute_result.status != NavigationStatus::Success {
      return result.status;
    }

    let real_absolute = path_is_absolute(&result.real_path);
    let real_path = into_string(result.real_path);
    self.real_path = if real_absolute {
      prepend_prefix(&self.absolute_path_prefix, real_path)
    } else {
      real_path
    };

    let absolute_real_path = into_string(absolute_result.real_path);
    self.absolute_real_path = prepend_prefix(&self.absolute_path_prefix, absolute_real_path);

    NavigationStatus::Success
  }
}

/// `updateRealPaths`（VfsNavigator.cpp）的前缀拼接镜像：`prefix` 非空时按
/// `with_capacity(prefix + path)` 精确容量把前缀拼到 `path` 前；为空则原样
/// 返回、免无谓分配（原两处逐字重复的拼接块收敛于此，行为逐字节不变）。
fn prepend_prefix(prefix: &str, path: String) -> String {
  if prefix.is_empty() {
    return path;
  }
  let mut s = String::with_capacity(prefix.len() + path.len());
  s.push_str(prefix);
  s.push_str(&path);
  s
}
