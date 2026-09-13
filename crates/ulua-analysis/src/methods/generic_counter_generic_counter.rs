use alloc::string::String;
use core::ptr::null;

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
      seen_counts: DenseHashMap::new(null()),
      cached_types,
      generics: DenseHashMap::new(null()),
      generic_packs: DenseHashMap::new(null()),
      polarity: Polarity::default(),
      steps: 0,
      hit_limits: false,
    }
  }
}
