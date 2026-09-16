use crate::{
  enums::navigation_status::NavigationStatus, functions::normalize_path::normalize_path,
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn to_child(&mut self, name: &str) -> NavigationStatus {
    if name == ".config" {
      return NavigationStatus::NotFound;
    }

    self.module_path = normalize_path(&format!("{}/{}", self.module_path, name));
    self.absolute_module_path = normalize_path(&format!("{}/{}", self.absolute_module_path, name));

    self.update_real_paths()
  }
}
