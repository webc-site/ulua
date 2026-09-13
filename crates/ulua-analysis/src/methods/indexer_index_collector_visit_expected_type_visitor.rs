//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ExpectedTypeVisitor.cpp:92:indexer_index_collector_visit`
//! Source: `Analysis/src/ExpectedTypeVisitor.cpp:82-107` (hand-ported)
//!
//! C++ `struct IndexerIndexCollector : TypeOnceVisitor` (anonymous namespace).
//! The virtual `visit(...)` overrides live as the `GenericTypeVisitorTrait`
//! impl (the `FindCyclicTypes`/`TypeFunctionFinder` precedents) so `traverse`
//! dispatches into them; the bodies delegate to the inherent methods declared
//! on the record (`indexer_index_collector.rs`).

use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    indexer_index_collector::IndexerIndexCollector,
    intersection_type::IntersectionType,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl GenericTypeVisitorTrait for IndexerIndexCollector {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  /// ExpectedTypeVisitor.cpp:92 — `bool visit(TypeId ty)`.
  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    IndexerIndexCollector::visit_type_id(self, ty)
  }

  /// ExpectedTypeVisitor.cpp:98 — `bool visit(TypeId, const UnionType&)`.
  fn visit_type_id_union_type(&mut self, ty: TypeId, utv: &UnionType) -> bool {
    IndexerIndexCollector::visit_union_type(self, ty, utv)
  }

  /// ExpectedTypeVisitor.cpp:103 — `bool visit(TypeId, const IntersectionType&)`.
  fn visit_type_id_intersection_type(&mut self, ty: TypeId, itv: &IntersectionType) -> bool {
    IndexerIndexCollector::visit_intersection_type(self, ty, itv)
  }
}
