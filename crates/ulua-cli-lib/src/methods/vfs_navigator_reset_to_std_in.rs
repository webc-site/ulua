use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    absolute_prefix::absolute_prefix, get_current_working_directory::get_current_working_directory,
    get_module_path::get_module_path, normalize_path::normalize_path,
  },
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn reset_to_std_in(&mut self) -> NavigationStatus {
    let Some(cwd) = get_current_working_directory() else {
      return NavigationStatus::NotFound;
    };

    self.real_path = "./stdin".to_string();
    self.absolute_real_path = normalize_path(&format!("{cwd}/stdin"));
    self.module_path = "./stdin".to_string();
    self.absolute_module_path = get_module_path(&self.absolute_real_path);
    self.absolute_path_prefix = absolute_prefix(&self.absolute_real_path);

    NavigationStatus::Success
  }
}
