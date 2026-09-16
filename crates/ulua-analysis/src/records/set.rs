//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Set.h:18:set`
//! Source: `Analysis/include/Luau/Set.h:18-130` (hand-ported)
//!
//! C++ `Luau::Set<T>` — DenseHashMap<T, bool> with a tombstone-false `erase`
//! (DenseHashSet cannot erase; Set can).

use ulua_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct Set<T> {
  pub(crate) mapping: DenseHashMap<T, bool>,
  pub(crate) entry_count: usize,
}
