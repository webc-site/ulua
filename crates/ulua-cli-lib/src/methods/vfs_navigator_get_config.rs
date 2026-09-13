use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_unreachable::LUAU_UNREACHABLE};

use crate::{
  enums::config_status::ConfigStatus,
  functions::{is_file::is_file, read_file::read_file},
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn get_config(&self) -> Option<String> {
    let status = self.get_config_status();
    LUAU_ASSERT!(status == ConfigStatus::PresentJson || status == ConfigStatus::PresentLuau);

    if status == ConfigStatus::PresentJson {
      // Luau::kConfigName (".luaurc") or legacy/alias (".uluarc")
      if is_file(&self.get_config_path(".luaurc")) {
        read_file(&self.get_config_path(".luaurc"))
      } else {
        read_file(&self.get_config_path(".uluarc"))
      }
    } else if status == ConfigStatus::PresentLuau {
      // Luau::kLuauConfigName
      read_file(&self.get_config_path(".config.luau"))
    } else {
      LUAU_UNREACHABLE!();
    }
  }
}
