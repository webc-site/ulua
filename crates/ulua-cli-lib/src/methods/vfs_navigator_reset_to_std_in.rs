use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::navigation_status::NavigationStatus,
  functions::{
    get_current_working_directory::get_current_working_directory, get_module_path::get_module_path,
    normalize_path::normalize_path,
  },
  records::vfs_navigator::VfsNavigator,
};

pub fn vfs_navigator_reset_to_std_in(navigator: &mut VfsNavigator) -> NavigationStatus {
  let cwd = get_current_working_directory();
  if cwd.is_none() {
    return NavigationStatus::NotFound;
  }

  navigator.real_path = "./stdin".to_string();
  navigator.absolute_real_path = normalize_path(&(cwd.unwrap() + "/stdin"));
  navigator.module_path = "./stdin".to_string();
  navigator.absolute_module_path = get_module_path(&navigator.absolute_real_path);

  let first_slash = navigator.absolute_real_path.find('/');
  LUAU_ASSERT!(first_slash.is_some());

  navigator.absolute_path_prefix = navigator
    .absolute_real_path
    .get(..first_slash.unwrap())
    .unwrap_or("")
    .to_string();

  NavigationStatus::Success
}
