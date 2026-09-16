use crate::{
  records::{indexer_index_collector::IndexerIndexCollector, intersection_type::IntersectionType},
  type_aliases::type_id::TypeId,
};

impl IndexerIndexCollector {
  pub fn visit_type_id_intersection_type(&mut self, _ty: TypeId, _it: &IntersectionType) -> bool {
    true
  }
}
