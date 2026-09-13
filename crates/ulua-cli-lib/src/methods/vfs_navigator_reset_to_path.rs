use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    get_current_working_directory::get_current_working_directory, get_module_path::get_module_path,
    is_absolute_path::is_absolute_path, normalize_path::normalize_path,
  },
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn reset_to_path(&mut self, path: &str) -> NavigationStatus {
    let normalized_path = normalize_path(path);

    if is_absolute_path(&normalized_path) {
      self.module_path = get_module_path(&normalized_path);
      self.absolute_module_path = self.module_path.clone();

      let first_slash = normalized_path.find('/');
      LUAU_ASSERT!(first_slash.is_some());
      self.absolute_path_prefix = normalized_path
        .get(..first_slash.unwrap())
        .unwrap_or("")
        .to_string();
    } else {
      let cwd = get_current_working_directory();
      if cwd.is_none() {
        return NavigationStatus::NotFound;
      }

      self.module_path = get_module_path(&normalized_path);
      let joined_path = normalize_path(&format!("{}/{}", cwd.unwrap(), normalized_path));
      self.absolute_module_path = get_module_path(&joined_path);

      let first_slash = joined_path.find('/');
      LUAU_ASSERT!(first_slash.is_some());
      self.absolute_path_prefix = joined_path
        .get(..first_slash.unwrap())
        .unwrap_or("")
        .to_string();
    }

    self.update_real_paths()
  }
}
