use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE};

use crate::{
  enums::config_status::ConfigStatus,
  functions::{
    config_names::{K_CONFIG_NAME, K_LEGACY_CONFIG_NAME, K_LUAU_CONFIG_NAME},
    is_file::is_file,
    read_file::read_file,
  },
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn get_config(&self) -> Option<String> {
    let status = self.get_config_status();
    LUAU_ASSERT!(status == ConfigStatus::PresentJson || status == ConfigStatus::PresentLuau);

    if status == ConfigStatus::PresentJson {
      let json_path = self.get_config_path(K_CONFIG_NAME);
      if is_file(&json_path) {
        read_file(&json_path)
      } else {
        read_file(&self.get_config_path(K_LEGACY_CONFIG_NAME))
      }
    } else if status == ConfigStatus::PresentLuau {
      read_file(&self.get_config_path(K_LUAU_CONFIG_NAME))
    } else {
      LUAU_UNREACHABLE!();
    }
  }
}
