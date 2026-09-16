use core::ptr::null;

use crate::records::ast_name::AstName;

impl AstName {
  pub const fn new() -> Self {
    Self { value: null() }
  }
}

impl Default for AstName {
  fn default() -> Self {
    Self::new()
  }
}
