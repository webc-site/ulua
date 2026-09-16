use alloc::string::String;

use crate::records::vfs_navigator::VfsNavigator;

impl VfsNavigator {
  pub fn get_absolute_file_path(&self) -> String {
    self.absolute_real_path.clone()
  }
}
