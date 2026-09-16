use alloc::string::String;

use crate::{
  records::{
    intersection_type::IntersectionType, type_ids::TypeIds, type_once_visitor::TypeOnceVisitor,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct IndexerIndexCollector {
  pub base: TypeOnceVisitor,
  pub indexes: *mut TypeIds,
}

impl IndexerIndexCollector {
  pub fn new(indexes: *mut TypeIds) -> Self {
    Self {
      base: TypeOnceVisitor::new(String::from("IndexerIndexCollector"), true),
      indexes,
    }
  }
}

impl IndexerIndexCollector {
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    unsafe {
      (*self.indexes).insert_type_id(ty);
    }
    false
  }

  pub fn visit_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
    true
  }

  pub fn visit_intersection_type(&mut self, _ty: TypeId, _it: &IntersectionType) -> bool {
    true
  }
}
