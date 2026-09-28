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
//!
//! **DELIBERATE DEVIATION**（出处 `InsertionOrderedMap.h:14-145`，逐项理由）：
//! - 结构：cpp 是 `std::vector<pair>` + `std::unordered_map<K, size_t>` 双写；Rust
//!   委托单个 `indexmap::IndexMap`（同一结构同时给插入序迭代与 O(1) 键查找），
//!   因而没有 `erase` 里那段 O(n) 的索引回移循环，也不存在两份结构失配的可能
//!   （cpp `size()` 的 `LUAU_ASSERT(pairs.size() == indices.size())` 在 Rust 侧是
//!   类型不变式）。行为逐方法等价。
//! - 句柄形态：cpp 的 `find` 返回迭代器、`erase` 收迭代器、`get` 返回 `V*`/`nullptr`；
//!   Rust 不外泄裸指针与迭代器别名（§2/§3），`find` 返回插入序下标 `Option<usize>`
//!   （`end()` 即 `None`），`erase` 按键（键不存在 = cpp 擦除 `end()` 的 no-op），
//!   `get`/`get_mut` 返回 `Option<&V>`/`Option<&mut V>`。
//! - `operator[]`（缺省插入并取值引用）落成 [`InsertionOrderedMap::get_or_default`]。
//! - cpp 的 `static_assert(std::is_trivially_copyable_v<K>)` 无 Rust 对应物，不上约束：
//!   该断言只为规避双结构下的平凡拷贝假设，单结构 IndexMap 无此前提。

use core::hash::Hash;

use foldhash::fast::FixedState;
use indexmap::{IndexMap, map::Iter};

/// 底层索引表用 `foldhash::fast::FixedState`（固定种子，取代 indexmap 默认的 std
/// `RandomState`/SipHash）：哈希只决定桶位置，`IndexMap` 的迭代序恒为插入序，
/// 故语义与 C++ 原实现完全一致，仅省去 SipHash 开销。
#[derive(Debug, Clone)]
pub struct InsertionOrderedMap<K, V>(IndexMap<K, V, FixedState>)
where
  K: Eq + Hash;

impl<K, V> InsertionOrderedMap<K, V>
where
  K: Eq + Hash,
{
  pub fn new() -> Self {
    Self(IndexMap::with_hasher(FixedState::default()))
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
