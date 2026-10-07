use std::path::Path;

use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    absolute_prefix::absolute_prefix, get_current_working_directory::get_current_working_directory,
    get_module_path::get_module_path, is_absolute_path::is_absolute_path,
    normalize_path::normalize_path, path_string::into_string,
  },
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn reset_to_path(&mut self, path: &str) -> NavigationStatus {
    let normalized_path = normalize_path(path);

    let absolute_module_path = if is_absolute_path(&normalized_path) {
      normalized_path.clone()
    } else {
      // 相对路径拼接 cwd, cwd 缺失 → NotFound
      let Some(cwd) = get_current_working_directory() else {
        return NavigationStatus::NotFound;
      };
      normalize_path(&format!("{cwd}/{normalized_path}"))
    };

    self.module_path = into_string(get_module_path(Path::new(&normalized_path)));
    self.absolute_module_path = into_string(get_module_path(Path::new(&absolute_module_path)));
    self.absolute_path_prefix = absolute_prefix(&absolute_module_path);

    self.update_real_paths()
  }
}
