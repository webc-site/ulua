//! @interface-stub
use crate::{
  records::{
    generic_type_visitor::GenericTypeVisitorTrait, index_collector::IndexCollector,
    indexer_index_collector::IndexerIndexCollector, singleton_type::SingletonType,
    string_singleton::StringSingleton, table_type::TableType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

impl IndexCollector {
  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, ttv: &TableType) -> bool {
    unsafe {
      for name in ttv.props.keys() {
        let singleton = (*self.arena).add_type(SingletonType::new(SingletonVariant::V1(
          StringSingleton::new(name.clone()),
        )));
        self.indexes.insert_type_id(singleton);
      }

      if let Some(indexer) = &ttv.indexer {
        let mut iic = IndexerIndexCollector::new(&mut self.indexes as *mut _);
        iic.traverse_type_id(indexer.index_type);
      }
    }

    false
  }
}
