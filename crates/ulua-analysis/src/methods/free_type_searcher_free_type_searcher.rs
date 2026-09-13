use core::ptr::null;

use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet,
  insertion_ordered_map::InsertionOrderedMap,
};

use crate::{
  enums::polarity::Polarity,
  records::{free_type_searcher::FreeTypeSearcher, scope::Scope, type_visitor::TypeVisitor},
  type_aliases::type_id::TypeId,
};
impl FreeTypeSearcher {
  pub fn new(scope: *mut Scope, cached_types: *mut DenseHashSet<TypeId>) -> Self {
    Self {
      base: TypeVisitor::new("FreeTypeSearcher".to_string(), true),
      scope,
      cached_types,
      is_within_function: false,
      polarity: Polarity::Positive,
      seen_positive: DenseHashSet::new(null()),
      seen_negative: DenseHashSet::new(null()),
      negative_types: DenseHashMap::new(null()),
      positive_types: DenseHashMap::new(null()),
      types: InsertionOrderedMap::new(),
      type_packs: InsertionOrderedMap::new(),
      unsealed_tables: DenseHashSet::new(null()),
    }
  }
}
