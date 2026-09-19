use crate::{
  enums::config_status::ConfigStatus,
  functions::{
    config_names::{K_CONFIG_NAME, K_LEGACY_CONFIG_NAME, K_LUAU_CONFIG_NAME},
    is_file::is_file,
  },
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn get_config_status(&self) -> ConfigStatus {
    let luaurc_exists = is_file(&self.get_config_path(K_CONFIG_NAME))
      || is_file(&self.get_config_path(K_LEGACY_CONFIG_NAME));
    let luau_config_exists = is_file(&self.get_config_path(K_LUAU_CONFIG_NAME));

    if luaurc_exists && luau_config_exists {
      ConfigStatus::Ambiguous
    } else if luau_config_exists {
      ConfigStatus::PresentLuau
    } else if luaurc_exists {
      ConfigStatus::PresentJson
    } else {
      ConfigStatus::Absent
    }
  }
}
