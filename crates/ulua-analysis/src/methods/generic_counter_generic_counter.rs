use alloc::string::String;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  enums::polarity::Polarity,
  records::{generic_counter::GenericCounter, type_visitor::TypeVisitor},
  type_aliases::type_id::TypeId,
};
impl GenericCounter {
  pub fn new(cached_types: *mut DenseHashSet<TypeId>) -> Self {
    Self {
      base: TypeVisitor::new(String::from("GenericCounter"), true),
      seen_counts: DenseHashMap::default(),
      cached_types,
      generics: DenseHashMap::default(),
      generic_packs: DenseHashMap::default(),
      polarity: Polarity::default(),
      steps: 0,
      hit_limits: false,
    }
  }
}
