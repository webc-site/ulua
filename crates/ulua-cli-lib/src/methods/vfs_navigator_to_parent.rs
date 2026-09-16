use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::navigation_status::NavigationStatus, functions::normalize_path::normalize_path,
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn to_parent(&mut self) -> NavigationStatus {
    if self.absolute_module_path == "/" {
      return NavigationStatus::NotFound;
    }

    let num_slashes = self
      .absolute_module_path
      .bytes()
      .filter(|c| *c == b'/')
      .count();
    LUAU_ASSERT!(num_slashes > 0);

    if num_slashes == 1 {
      return NavigationStatus::NotFound;
    }

    self.module_path = normalize_path(&(self.module_path.clone() + "/.."));
    self.absolute_module_path = normalize_path(&(self.absolute_module_path.clone() + "/.."));

    let status = self.update_real_paths();
    if status == NavigationStatus::Ambiguous {
      NavigationStatus::Success
    } else {
      status
    }
  }
}
