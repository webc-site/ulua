use crate::records::{generic_type_finder::GenericTypeFinder, type_once_visitor::TypeOnceVisitor};

impl GenericTypeFinder {
  pub fn new() -> Self {
    GenericTypeFinder {
      base: TypeOnceVisitor::new("GenericTypeFinder".to_string(), true),
      found: false,
    }
  }
}

impl Default for GenericTypeFinder {
  fn default() -> Self {
    Self::new()
  }
}
