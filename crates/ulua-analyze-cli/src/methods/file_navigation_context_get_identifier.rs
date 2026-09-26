use core::option::Option;

use crate::records::file_navigation_context::FileNavigationContext;

impl FileNavigationContext {
  pub fn get_identifier(&self) -> Option<&str> {
    Some(self.vfs.absolute_real_path.as_str())
  }
}
