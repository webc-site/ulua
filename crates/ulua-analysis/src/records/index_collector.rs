use alloc::string::String;

use crate::{
  records::{
    arena_handle::Handle, type_arena::TypeArena, type_ids::TypeIds,
    type_once_visitor::TypeOnceVisitor,
  },
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct IndexCollector {
  pub base: TypeOnceVisitor,
  pub arena: Handle<TypeArena>,
  pub indexes: TypeIds,
}

impl IndexCollector {
  pub fn new(arena: Handle<TypeArena>) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("IndexCollector"), true),
      arena,
      indexes: TypeIds::new(),
    }
  }

  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    false
  }
}
