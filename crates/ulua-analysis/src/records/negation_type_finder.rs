use alloc::string::String;

use crate::{records::type_once_visitor::TypeOnceVisitor, type_aliases::type_id::TypeId};

#[derive(Debug, Clone)]
pub struct NegationTypeFinder {
  pub base: TypeOnceVisitor,
  pub found: bool,
}

impl NegationTypeFinder {
  pub fn new() -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("NegationTypeFinder"), false),
      found: false,
    }
  }

  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found
  }
}

impl Default for NegationTypeFinder {
  fn default() -> Self {
    Self::new()
  }
}
