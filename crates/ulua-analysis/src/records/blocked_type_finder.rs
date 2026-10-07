use alloc::string::String;
use core::option::Option;

use crate::{records::type_once_visitor::TypeOnceVisitor, type_aliases::type_id::TypeId};

#[derive(Debug, Clone)]
pub struct BlockedTypeFinder {
  pub base: TypeOnceVisitor,
  pub blocked: Option<TypeId>,
}

impl BlockedTypeFinder {
  pub fn new() -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("ContainsGenerics_DEPRECATED"), true),
      blocked: None,
    }
  }

  pub fn visit(&mut self, _ty: TypeId) -> bool {
    self.blocked.is_none()
  }
}

impl Default for BlockedTypeFinder {
  fn default() -> Self {
    Self::new()
  }
}
