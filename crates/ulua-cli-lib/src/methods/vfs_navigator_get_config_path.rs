use alloc::string::String;

use crate::{functions::has_suffix::has_suffix, records::vfs_navigator::VfsNavigator};

const K_INIT_SUFFIXES: &[&str] = &["/init.lua", "/init.luau"];
const K_SUFFIXES: &[&str] = &[".lua", ".luau"];

impl VfsNavigator {
  pub fn get_config_path(&self, filename: &str) -> String {
    let mut directory = self.real_path.as_str();

    for suffix in K_INIT_SUFFIXES {
      if has_suffix(directory, suffix) {
        directory = &directory[..directory.len() - suffix.len()];
        return [directory, "/", filename].concat();
      }
    }

    for suffix in K_SUFFIXES {
      if has_suffix(directory, suffix) {
        directory = &directory[..directory.len() - suffix.len()];
        return [directory, "/", filename].concat();
      }
    }

    [directory, "/", filename].concat()
  }
}
