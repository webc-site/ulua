//! Source: `Analysis/include/Luau/Set.h:18-130` (hand-ported)
//!
//! C++ `Luau::Set<T>` — DenseHashMap<T, bool> with a tombstone-false `erase`
//! (DenseHashSet cannot erase; Set can).

use core::hash::Hash;

use ulua_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct Set<T> {
  pub(crate) mapping: DenseHashMap<T, bool>,
  pub(crate) entry_count: usize,
}

impl<T: Clone + Hash + PartialEq> PartialEq for Set<T> {
  fn eq(&self, other: &Self) -> bool {
    if self.size() != other.size() {
      return false;
    }
    self
      .mapping
      .iter()
      .all(|(elem, present)| !*present || other.contains(elem))
  }
}

impl<T: Clone + Hash + Eq> Eq for Set<T> {}
