use alloc::string::String;
use core::option::Option;

use crate::records::file_navigation_context::FileNavigationContext;

impl FileNavigationContext {
  pub fn get_identifier(&self) -> Option<String> {
    Some(self.vfs.get_absolute_file_path())
  }
}
