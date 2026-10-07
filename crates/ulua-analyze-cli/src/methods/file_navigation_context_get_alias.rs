use alloc::vec::Vec;
use core::option::Option;

use crate::records::file_navigation_context::FileNavigationContext;

impl FileNavigationContext {
  pub fn get_alias(&self, _alias: &[u8]) -> Option<Vec<u8>> {
    None
  }
}
