use alloc::string::String;

use crate::{
  functions::{get_module_path::K_MODULE_SUFFIXES, has_suffix::has_suffix},
  records::vfs_navigator::VfsNavigator,
};

impl VfsNavigator {
  pub fn get_config_path(&self, filename: &str) -> String {
    let mut directory = self.real_path.as_str();

    for &suffix in K_MODULE_SUFFIXES {
      if has_suffix(directory, suffix) {
        directory = &directory[..directory.len() - suffix.len()];
        return [directory, "/", filename].concat();
      }
    }

    [directory, "/", filename].concat()
  }
}
