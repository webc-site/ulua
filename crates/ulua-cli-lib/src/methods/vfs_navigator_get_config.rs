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
    LUAU_ASSERT!(matches!(
      status,
      ConfigStatus::PresentJson | ConfigStatus::PresentLuau
    ));

    // 按状态枚举分派（cpp `getConfig` 的 if/else 链）；Absent/Ambiguous 已由上方
    // 断言排除，cpp 此处即 `LUAU_UNREACHABLE()`。
    match status {
      ConfigStatus::PresentJson => read_file(&self.get_config_path(K_CONFIG_NAME)),
      ConfigStatus::PresentLuau => read_file(&self.get_config_path(K_LUAU_CONFIG_NAME)),
      _ => LUAU_UNREACHABLE!(),
    }
  }
}
