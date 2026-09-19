use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE};

use crate::{
  enums::config_status::ConfigStatus,
  functions::{
    config_names::{K_CONFIG_NAME, K_LUAU_CONFIG_NAME},
    read_file::read_file,
  },
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  /// 读配置文件，镜像 cpp `VfsNavigator::getConfig`（VfsNavigator.cpp:259-269）：
  /// `PresentJson` 直读 `kConfigName`，无 legacy 回退分支。
  pub fn get_config(&self) -> Option<String> {
    let status = self.get_config_status();
    LUAU_ASSERT!(status == ConfigStatus::PresentJson || status == ConfigStatus::PresentLuau);

    if status == ConfigStatus::PresentJson {
      read_file(&self.get_config_path(K_CONFIG_NAME))
    } else if status == ConfigStatus::PresentLuau {
      read_file(&self.get_config_path(K_LUAU_CONFIG_NAME))
    } else {
      LUAU_UNREACHABLE!();
    }
  }
}
