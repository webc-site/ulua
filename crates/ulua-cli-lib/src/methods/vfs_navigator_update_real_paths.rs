use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{get_real_path::get_real_path, is_absolute_path::is_absolute_path},
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub(crate) fn update_real_paths(&mut self) -> NavigationStatus {
    let result = get_real_path(&self.module_path);
    let absolute_result = get_real_path(&self.absolute_module_path);

    // 单条件早退，镜像 cpp `updateRealPaths`（VfsNavigator.cpp:113-118）：
    // cpp 此处误返回 result.status —— 只有 modulePath 失败才反映真实状态，
    // absoluteModulePath 失败时同样返回 result.status（即 Success），保持一致不额外修正。
    if result.status != NavigationStatus::Success
      || absolute_result.status != NavigationStatus::Success
    {
      return result.status;
    }

    self.real_path = if is_absolute_path(&result.real_path) {
      format!("{}{}", self.absolute_path_prefix, result.real_path)
    } else {
      result.real_path
    };

    self.absolute_real_path = format!("{}{}", self.absolute_path_prefix, absolute_result.real_path);

    NavigationStatus::Success
  }
}
