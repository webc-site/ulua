use crate::{
  enums::config_status::ConfigStatus, functions::is_file::is_file,
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn get_config_status(&self) -> ConfigStatus {
    let luaurc_exists =
      is_file(&self.get_config_path(".luaurc")) || is_file(&self.get_config_path(".uluarc"));
    let luau_config_exists = is_file(&self.get_config_path(".config.luau"));

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
