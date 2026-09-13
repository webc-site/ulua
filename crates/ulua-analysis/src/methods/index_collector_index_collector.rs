use alloc::string::ToString;

use crate::records::{
  index_collector::IndexCollector, type_arena::TypeArena, type_once_visitor::TypeOnceVisitor,
};

impl IndexCollector {
  pub fn index_collector(&mut self, arena: *mut TypeArena) {
    self.base = TypeOnceVisitor::new("IndexCollector".to_string(), true);
    self.arena = arena;
  }
}
