use alloc::string::String;

use crate::{
  records::{type_arena::TypeArena, type_ids::TypeIds, type_once_visitor::TypeOnceVisitor},
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct IndexCollector {
  pub base: TypeOnceVisitor,
  pub arena: *mut TypeArena,
  pub indexes: TypeIds,
}

impl IndexCollector {
  pub fn new(arena: *mut TypeArena) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("IndexCollector"), true),
      arena,
      indexes: TypeIds::new(),
    }
  }
}

impl IndexCollector {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    false
  }
}
