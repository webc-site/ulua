//! `set` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::hash::Hash;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::set::Set;

// Source: `Analysis/include/Luau/Set.h:104-199` (hand-ported)

impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `begin()/end()` over live entries only. `erase` leaves tombstones in
  /// the underlying map, so iteration must skip entries whose presence bit is
  /// false.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self
      .mapping
      .iter()
      .filter_map(|(element, present)| if *present { Some(element) } else { None })
  }
}

// Source: `Analysis/include/Luau/Set.h:77-81` (hand-ported)

impl<T: Clone + Hash + PartialEq> Set<T> {
  pub fn clear(&mut self) {
    self.mapping.clear();
    self.entry_count = 0;
  }
}

// Source: `Analysis/include/Luau/Set.h:99-102` (hand-ported)

impl<T: Clone + Hash + PartialEq> Set<T> {
  pub fn contains(&self, element: &T) -> bool {
    self.count(element) != 0
  }
}

// Source: `Analysis/include/Luau/Set.h:88-91` (hand-ported)

impl<T: Clone + Hash + PartialEq> Set<T> {
  pub fn empty(&self) -> bool {
    self.entry_count == 0
  }
}

// Source: `Analysis/include/Luau/Set.h:29-32` (hand-ported)

impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `explicit Set(const T& empty_key)`.
  pub fn new(empty_key: T) -> Self {
    Self {
      mapping: DenseHashMap::new(empty_key),
      entry_count: 0,
    }
  }
}

// Source: `Analysis/include/Luau/Set.h:83-86` (hand-ported)

impl<T: Clone + Hash + PartialEq> Set<T> {
  pub fn size(&self) -> usize {
    self.entry_count
  }
}
