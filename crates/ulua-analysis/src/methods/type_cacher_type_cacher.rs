use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{type_cacher::TypeCacher, type_once_visitor::TypeOnceVisitor},
  type_aliases::type_id::TypeId,
};
impl TypeCacher {
  pub fn new(cached_types: *mut DenseHashSet<TypeId>) -> Self {
    Self {
      base: TypeOnceVisitor::new("TypeCacher".to_string(), true),
      cached_types,
      uncacheable: DenseHashSet::default(),
      uncacheable_packs: DenseHashSet::default(),
    }
  }
}
