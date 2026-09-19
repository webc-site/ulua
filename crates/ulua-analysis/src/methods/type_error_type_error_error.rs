use alloc::string::String;
use core::mem::zeroed;

use ulua_ast::records::location::Location;

use crate::records::type_error::TypeError;
impl TypeError {
  pub fn new() -> Self {
    Self {
      location: Location::default(),
      module_name: String::new(),
      data: unsafe { zeroed() },
    }
  }
}

impl Default for TypeError {
  fn default() -> Self {
    Self::new()
  }
}
