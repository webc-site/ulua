use core::ptr::null;

use crate::records::lazy_type::LazyType;
impl LazyType {
  pub fn new() -> Self {
    Self {
      unwrap: None,
      unwrapped: null(),
    }
  }
}

impl Default for LazyType {
  fn default() -> Self {
    Self::new()
  }
}
