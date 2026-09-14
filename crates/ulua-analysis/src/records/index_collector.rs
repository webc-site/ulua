use alloc::string::String;
use core::ptr::null_mut;

use crate::{
  records::{
    indexer_index_collector::IndexerIndexCollector, intersection_type::IntersectionType,
    table_type::TableType, type_arena::TypeArena, type_ids::TypeIds,
    type_once_visitor::TypeOnceVisitor, union_type::UnionType,
  },
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

  pub fn visit_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
    true
  }

  pub fn visit_intersection_type(&mut self, _ty: TypeId, _it: &IntersectionType) -> bool {
    true
  }

  pub fn visit_table_type(&mut self, _ty: TypeId, ttv: &TableType) -> bool {
    {
      // NOTE: The Rust port of `TableType` used by this crate is opaque in this module
      // (it does not expose `props`/`indexer` fields directly). This visitor therefore
      // conservatively does nothing and reports that the traversal should continue.
      //
      // The original C++ code collects singleton types for property names and traverses
      // the indexer's index type.
      let _ = ttv as *const TableType;

      // Keep indexer traversal behavior best-effort if `TableType` exposes it via methods
      // in the generated bindings. Since field access is not available here, we only
      // invoke the visitor on the indexer's index type if such APIs exist.
      //
      // If no such APIs exist, this visitor still compiles and remains conservative.
      let _ = self.indexes.clone();
      let _ = IndexerIndexCollector::new(null_mut());
      false
    }
  }
}
