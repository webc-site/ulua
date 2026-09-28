use alloc::string::String;

use crate::records::type_once_visitor::TypeOnceVisitor;

#[derive(Debug, Clone)]
pub struct InternalTypeFinder {
  pub base: TypeOnceVisitor,
}

impl InternalTypeFinder {
  pub fn new() -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("InternalTypeFinder"), true),
    }
  }
}

impl Default for InternalTypeFinder {
  fn default() -> Self {
    Self::new()
  }
}
