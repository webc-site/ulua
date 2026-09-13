use alloc::string::String;

use crate::records::vfs_navigator::VfsNavigator;

impl VfsNavigator {
  pub fn get_file_path(&self) -> String {
    self.real_path.clone()
  }
}
