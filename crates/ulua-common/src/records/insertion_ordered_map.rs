//! Node: `cxx:Record:Luau.Common:Common/include/Luau/InsertionOrderedMap.h:15:insertion_ordered_map`
//! Source: `Common/include/Luau/InsertionOrderedMap.h:14-145`
//!
//! A map preserving insertion order, delegating to `indexmap::IndexMap`.
//! Semantics match the C++ original method-by-method: `insert` is
//! first-write-wins (a duplicate key keeps its original position and value,
//! a no-op), `erase` uses `shift_remove` (the pair is removed and every later
//! position shifts down by one, exactly like the C++ loop), `find` returns the
//! position in insertion order (`get_index_of`), and C++ `operator[]`
//! (find-or-default-insert) is `get_or_default`. The hand-written
//! Vec+HashMap dual structure (and its O(n) erase re-index loop) is gone;
//! lookups and iteration are single-pass over one structure.
//!
//! Iterator items are `(&K, &V)` (IndexMap shape) — the C++ `begin()`/`end()`
//! iterate `pair<const K, V>&`, so the key is immutable in both.

use core::hash::Hash;

use indexmap::{
  IndexMap,
  map::{Iter, IterMut},
};

#[derive(Debug, Clone)]
pub struct InsertionOrderedMap<K, V>(IndexMap<K, V>)
where
  K: Eq + Hash;

impl<K, V> InsertionOrderedMap<K, V>
where
  K: Eq + Hash,
{
  pub fn new() -> Self {
    Self(IndexMap::new())
  }

  /// 首写胜出：键已存在时保持原位次与原值（对齐 C++ `insert`）。
  pub fn insert(&mut self, k: K, v: V) {
    self.0.entry(k).or_insert(v);
  }

  pub fn clear(&mut self) {
    self.0.clear();
  }

  pub fn size(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn contains(&self, k: &K) -> bool {
    self.0.contains_key(k)
  }

  pub fn get(&self, k: &K) -> Option<&V> {
    self.0.get(k)
  }

  pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
    self.0.get_mut(k)
  }

  /// C++ `operator[]`: returns the value for `k`, default-inserting it at
  /// the back if absent.
  pub fn get_or_default(&mut self, k: K) -> &mut V
  where
    V: Default,
  {
    self.0.entry(k).or_default()
  }

  /// C++ `find`: position of `k` in insertion order, or `None` (`end()`).
  pub fn find(&self, k: &K) -> Option<usize> {
    self.0.get_index_of(k)
  }

  /// C++ `erase(find(k))`: removes the pair and shifts every later index
  /// down by one. Absent keys are a no-op (erasing `end()`).
  pub fn erase(&mut self, k: &K) {
    self.0.shift_remove(k);
  }

  pub fn iter(&self) -> Iter<'_, K, V> {
    self.0.iter()
  }

  /// 迭代中值是可变的；键是不可变的。
  pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
    self.0.iter_mut()
  }
}

impl<'a, K, V> IntoIterator for &'a InsertionOrderedMap<K, V>
where
  K: Eq + Hash,
{
  type Item = (&'a K, &'a V);
  type IntoIter = Iter<'a, K, V>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<K, V> Default for InsertionOrderedMap<K, V>
where
  K: Eq + Hash,
{
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::InsertionOrderedMap;

  // Behavioral oracle: mirrors the C++ semantics in
  // Common/include/Luau/InsertionOrderedMap.h.
  #[test]
  fn insertion_order_preserved_and_duplicate_insert_is_noop() {
    let mut m: InsertionOrderedMap<i32, &str> = InsertionOrderedMap::new();
    m.insert(3, "c");
    m.insert(1, "a");
    m.insert(2, "b");
    m.insert(1, "OVERWRITE"); // C++: duplicate key is a no-op
    let keys: Vec<i32> = m.iter().map(|(k, _)| *k).collect();
    assert_eq!(keys, vec![3, 1, 2]);
    assert_eq!(m.get(&1), Some(&"a"));
    assert_eq!(m.size(), 3);
  }

  #[test]
  fn erase_reindexes_later_entries() {
    let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
    for k in [10, 20, 30, 40] {
      m.insert(k, k * 2);
    }
    m.erase(&20);
    assert_eq!(m.size(), 3);
    assert_eq!(m.find(&10), Some(0));
    assert_eq!(m.find(&30), Some(1));
    assert_eq!(m.find(&40), Some(2));
    assert_eq!(m.get(&40), Some(&80));
    m.erase(&999); // erasing end() is a no-op
    assert_eq!(m.size(), 3);
  }

  #[test]
  fn get_or_default_matches_cpp_index_operator() {
    let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
    *m.get_or_default(5) = 50;
    assert_eq!(m.get(&5), Some(&50));
    *m.get_or_default(5) += 1; // existing: no new entry
    assert_eq!(m.get(&5), Some(&51));
    assert_eq!(m.size(), 1);
  }
}
