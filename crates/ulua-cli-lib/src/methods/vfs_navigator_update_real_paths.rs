use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{get_real_path::get_real_path, is_absolute_path::is_absolute_path},
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub(crate) fn update_real_paths(&mut self) -> NavigationStatus {
    let result = get_real_path(self.module_path.clone());
    let absolute_result = get_real_path(self.absolute_module_path.clone());

    if result.status != NavigationStatus::Success
      || absolute_result.status != NavigationStatus::Success
    {
      return if result.status != NavigationStatus::Success {
        result.status
      } else {
        absolute_result.status
      };
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
