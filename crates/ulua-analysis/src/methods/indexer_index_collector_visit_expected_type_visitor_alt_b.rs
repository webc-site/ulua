use crate::{
  records::{indexer_index_collector::IndexerIndexCollector, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

impl IndexerIndexCollector {
  pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
    true
  }
}
