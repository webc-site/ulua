use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    absolute_prefix::absolute_prefix, get_current_working_directory::get_current_working_directory,
    get_module_path::get_module_path, normalize_path::normalize_path,
  },
  records::vfs_navigator::VfsNavigator,
};

pub fn vfs_navigator_reset_to_std_in(navigator: &mut VfsNavigator) -> NavigationStatus {
  let Some(cwd) = get_current_working_directory() else {
    return NavigationStatus::NotFound;
  };

  navigator.real_path = "./stdin".to_string();
  navigator.absolute_real_path = normalize_path(&format!("{cwd}/stdin"));
  navigator.module_path = "./stdin".to_string();
  navigator.absolute_module_path = get_module_path(&navigator.absolute_real_path);
  navigator.absolute_path_prefix = absolute_prefix(&navigator.absolute_real_path);

  NavigationStatus::Success
}
