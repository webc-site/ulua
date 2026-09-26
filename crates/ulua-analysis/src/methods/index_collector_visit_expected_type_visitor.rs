use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    index_collector::IndexCollector,
    indexer_index_collector::IndexerIndexCollector,
    intersection_type::IntersectionType,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

impl GenericTypeVisitorTrait for IndexCollector {
  type Seen = DenseHashSet<*mut ()>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  /// ExpectedTypeVisitor.cpp:120 — `bool visit(TypeId ty)`.
  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    IndexCollector::visit_type_id(self, ty)
  }

  /// ExpectedTypeVisitor.cpp:125 — `bool visit(TypeId, const UnionType&)`.
  fn visit_type_id_union_type(&mut self, ty: TypeId, utv: &UnionType) -> bool {
    IndexCollector::visit_type_id_union_type(self, ty, utv)
  }

  /// ExpectedTypeVisitor.cpp:130 — `bool visit(TypeId, const IntersectionType&)`.
  fn visit_type_id_intersection_type(&mut self, ty: TypeId, itv: &IntersectionType) -> bool {
    IndexCollector::visit_type_id_intersection_type(self, ty, itv)
  }

  /// ExpectedTypeVisitor.cpp:135 — `bool visit(TypeId, const TableType&)`.
  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    IndexCollector::visit_type_id_table_type(self, ty, ttv)
  }
}

impl IndexCollector {
  pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
    true
  }

  pub fn visit_type_id_intersection_type(&mut self, _ty: TypeId, _it: &IntersectionType) -> bool {
    true
  }

  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, ttv: &TableType) -> bool {
    for name in ttv.props.keys() {
      let singleton = self
        .arena
        .get_mut()
        .add_type(SingletonType::new(SingletonVariant::V1(
          StringSingleton::new(name.clone()),
        )));
      self.indexes.insert_type_id(singleton);
    }

    if let Some(indexer) = &ttv.indexer {
      let mut iic = IndexerIndexCollector::new(&mut self.indexes as *mut _);
      iic.traverse_type_id(indexer.index_type);
    }

    false
  }
}
