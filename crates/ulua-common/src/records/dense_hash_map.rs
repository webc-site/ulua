//! Faithful port of `Luau::DenseHashMap` — a fast `unordered_map` alternative
//! that supports erase (TAOCP algorithm R backward-shift) and uses `find()`
//! instead of returning an iterator. Reference:
//! `luau/Common/include/Luau/DenseHash.h:823-940`.
//!
//! 构造仍收一个 `empty_key`：上游新版已改用来给空槽填占位值（占用判定由位图
//! 负责），因此等于 `empty_key` 的键也可以正常存取；保留该参数是为了对齐下游
//! 数百处 `DenseHashMap<K, V>(emptyKey)` 构造点，详见
//! [`crate::records::dense_hash_table`] 的模块文档。

use core::{
  fmt::{Debug, Formatter, Result},
  iter::FusedIterator,
};

use crate::{
  records::{
    const_iterator::ConstIterator,
    dense_hash_table::{
      DenseDefault, DenseEq, DenseEqDefault, DenseHashTable, DenseHasher, ItemInterfaceMap,
    },
    iterator::MutIterator,
  },
  type_aliases::dense_hash_default::{DenseHashDefault, dense_hash_of},
};

type MapImpl<K, V, H, E> = DenseHashTable<K, (K, V), ItemInterfaceMap<K, V>, H, E>;

pub struct DenseHashMap<K, V, H = DenseHashDefault<K>, E = DenseEqDefault<K>> {
  pub(crate) impl_: MapImpl<K, V, H, E>,
}

pub struct Iter<'a, K, V> {
  pub(crate) impl_: ConstIterator<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
  type Item = (&'a K, &'a V);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.impl_.next().map(|item| (&item.0, &item.1))
  }
}

impl<K, V> FusedIterator for Iter<'_, K, V> {}

pub struct IterMut<'a, K, V> {
  pub(crate) impl_: MutIterator<'a, (K, V)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
  type Item = (&'a K, &'a mut V);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.impl_.next().map(|item| (&item.0, &mut item.1))
  }
}

impl<K, V> FusedIterator for IterMut<'_, K, V> {}

impl<K: Clone, V: Clone, H: Clone, E: Clone> Clone for DenseHashMap<K, V, H, E> {
  fn clone(&self) -> Self {
    DenseHashMap {
      impl_: self.impl_.clone(),
    }
  }
}

impl<K: Debug, V: Debug, H, E> Debug for DenseHashMap<K, V, H, E> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("DenseHashMap")
      .field("impl_", &self.impl_)
      .finish()
  }
}

impl<K, V, H, E> DenseHashMap<K, V, H, E>
where
  K: Clone,
  V: DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  /// `DenseHashMap(empty_key, buckets = 0)`. Reference: `DenseHash.h:836-839`.
  pub fn new(empty_key: K) -> Self {
    DenseHashMap {
      impl_: DenseHashTable::new(empty_key, 0),
    }
  }
  /// 对应 cpp `DenseHashMap(size_t buckets)` 构造变体的桶预留语义
  /// （`DenseHash.h:836-839`，消费点 `Substitution.cpp:161-163`）：对已构造的
  /// 空 map 将桶数扩到 `buckets`；`buckets` 须为 0 或 2 的幂，非空表为 no-op。
  pub fn reserve_buckets(&mut self, buckets: usize) {
    self.impl_.reserve_buckets(buckets);
  }

  /// `operator[]` — inserts a default value when absent and returns a mutable
  /// reference to the slot's value. Reference: `DenseHash.h:847-851`.
  pub fn get_or_insert(&mut self, key: K) -> &mut V {
    self.impl_.rehash_if_full(&key);
    let idx = self.impl_.insert_unsafe(key);
    &mut self.impl_.data[idx].1
  }

  /// Inserts or replaces a key-value pair, returning a mutable reference to the slot's value.
  pub fn insert(&mut self, key: K, value: V) -> &mut V {
    let slot = self.get_or_insert(key);
    *slot = value;
    slot
  }

  /// `const Value* find(const Key&) const`. Reference: `DenseHash.h:854-859`.
  pub fn find(&self, key: &K) -> Option<&V> {
    self.impl_.find(key).map(|idx| &self.impl_.data[idx].1)
  }

  /// std-style alias for generated Rust that spelled C++ `find` as `get`.
  pub fn get(&self, key: &K) -> Option<&V> {
    self.find(key)
  }

  /// `Value* find(const Key&)`. Reference: `DenseHash.h:862-867`.
  pub fn find_mut(&mut self, key: &K) -> Option<&mut V> {
    self.impl_.find(key).map(|idx| &mut self.impl_.data[idx].1)
  }

  /// std-style alias for generated Rust that spelled C++ `find` as `get_mut`.
  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    self.find_mut(key)
  }

  /// `contains`. Reference: `DenseHash.h:869-872`.
  /// std-style alias for `contains` — translations use HashMap idioms.
  pub fn contains_key(&self, key: &K) -> bool {
    self.contains(key)
  }

  pub fn contains(&self, key: &K) -> bool {
    self.impl_.find(key).is_some()
  }

  /// `try_insert` — returns `(value_ref, fresh)`, where `fresh` is true only
  /// when the key was newly inserted; an existing slot keeps its value.
  /// Reference: `DenseHash.h:879-909`.
  pub fn try_insert(&mut self, key: K, value: V) -> (&mut V, bool) {
    self.impl_.rehash_if_full(&key);

    let before = self.impl_.size();
    let idx = self.impl_.insert_unsafe(key);

    // Value is fresh if container count has increased
    let fresh = self.impl_.size() > before;

    if fresh {
      self.impl_.data[idx].1 = value;
    }

    (&mut self.impl_.data[idx].1, fresh)
  }

  /// `erase`。Reference: `DenseHash.h:874-877`。
  pub fn erase(&mut self, key: &K) {
    self.impl_.erase(key);
  }

  /// `size`. Reference: `DenseHash.h:911-914`.
  pub fn size(&self) -> usize {
    self.impl_.size()
  }

  /// std-style alias for `size()`.
  #[inline]
  pub fn len(&self) -> usize {
    self.size()
  }

  /// `empty`. Reference: `DenseHash.h:916-919`.
  pub fn empty(&self) -> bool {
    self.impl_.size() == 0
  }

  /// std-style alias for translated code that calls C++ `empty()` as `is_empty()`.
  pub fn is_empty(&self) -> bool {
    self.empty()
  }

  /// `clear`. Reference: `DenseHash.h:841-844`.
  pub fn clear(&mut self) {
    self.impl_.clear();
  }

  /// `begin()/end()` const iteration, yielding `(&Key, &Value)`.
  #[inline]
  pub fn iter(&self) -> Iter<'_, K, V> {
    Iter {
      impl_: self.impl_.iter(),
    }
  }

  /// `begin()/end()` mutable iteration, yielding `(&Key, &mut Value)`.
  #[inline]
  pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
    IterMut {
      impl_: self.impl_.iter_mut(),
    }
  }
}

/// `String` 键 + 默认 functor 的 `&str` 借用视图查询口（r7-rc-4 形状口）。
///
/// 动机：`find/get/contains/erase` 只收 `&K`（即 `&String`），下游（analysis
/// 24+ 站点）为查询被迫 `String::from(&str)` 现场分配堆键。本组口零克隆、零
/// 临时分配：预计算 `&str` 哈希后直接进与 owned 口同一张探测核
/// （[`DenseHashTable::find_by_view`]/[`erase_by_view`]）。
///
/// 为何专化到 `K = String` + 默认 `DenseHashDefault`/`DenseEqDefault`，而非
/// `where K: Borrow<str>` 泛型口：借用口成立的**全部**前提是哈希与等值两条路径
/// 对 `String`/`str` 逐位一致，且该性质只在默认 functor 下可证——
/// - hash：std 的 `impl Hash for String` 是对 `impl Hash for str` 的纯转发，
///   两者经 [`dense_hash_of`] 喂入同一字节流（含空串、内含 NUL、多字节 UTF-8、
///   超长键；`tests/dense_hash.rs` 双口等价用例钉死，wasm32 的 `usize` 截断也同路径）；
/// - eq：`DenseEqDefault<String>` 走 `String == String`，与 `stored.as_str() == key`
///   同为字节相等，逐位一致。
///
/// 定制 functor（任意非默认 `H`/`E`）可能持有仅对 `&String` 成立的语义，泛型口
/// 会静默破坏其命中判定，故本票刻意不给——这也是比 `Borrow<str>` 泛型更保守的
/// 选型理由：默认参数是全部 String 键消费点的现状，泛型口则是把未验证的组合
/// 放进类型系统。既有 `&String` 口一行不改，全部消费者零破坏。
///
/// [`erase_by_view`]: DenseHashTable::erase_by_view
impl<V: DenseDefault> DenseHashMap<String, V, DenseHashDefault<String>, DenseEqDefault<String>> {
  /// `find` 的 `&str` 借用口。Reference: `DenseHash.h:854-859`。
  pub fn find_str(&self, key: &str) -> Option<&V> {
    self
      .impl_
      .find_by_view(dense_hash_of(key), |stored| stored.as_str() == key)
      .map(|idx| &self.impl_.data[idx].1)
  }

  /// std-style alias for generated Rust that spelled C++ `find` as `get`.
  pub fn get_str(&self, key: &str) -> Option<&V> {
    self.find_str(key)
  }

  /// `find_mut` 的 `&str` 借用口。Reference: `DenseHash.h:862-867`。
  pub fn find_mut_str(&mut self, key: &str) -> Option<&mut V> {
    let idx = self
      .impl_
      .find_by_view(dense_hash_of(key), |stored| stored.as_str() == key)?;
    Some(&mut self.impl_.data[idx].1)
  }

  /// std-style alias for `find_mut_str`.
  pub fn get_mut_str(&mut self, key: &str) -> Option<&mut V> {
    self.find_mut_str(key)
  }

  /// `contains` 的 `&str` 借用口。Reference: `DenseHash.h:869-872`。
  pub fn contains_str(&self, key: &str) -> bool {
    self
      .impl_
      .find_by_view(dense_hash_of(key), |stored| stored.as_str() == key)
      .is_some()
  }

  /// std-style alias for `contains_str`。
  pub fn contains_key_str(&self, key: &str) -> bool {
    self.contains_str(key)
  }

  /// `erase` 的 `&str` 借用口。Reference: `DenseHash.h:874-877`。
  pub fn erase_str(&mut self, key: &str) {
    self
      .impl_
      .erase_by_view(dense_hash_of(key), |stored| stored.as_str() == key);
  }
}

impl<'a, K, V, H, E> IntoIterator for &'a DenseHashMap<K, V, H, E>
where
  K: Clone,
  V: DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  type Item = (&'a K, &'a V);
  type IntoIter = Iter<'a, K, V>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, K, V, H, E> IntoIterator for &'a mut DenseHashMap<K, V, H, E>
where
  K: Clone,
  V: DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  type Item = (&'a K, &'a mut V);
  type IntoIter = IterMut<'a, K, V>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<K, V, H, E> Extend<(K, V)> for DenseHashMap<K, V, H, E>
where
  K: Clone,
  V: DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
    for (k, v) in iter {
      self.insert(k, v);
    }
  }
}

impl<K, V, H, E> FromIterator<(K, V)> for DenseHashMap<K, V, H, E>
where
  K: Clone + DenseDefault,
  V: DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
    let mut map = Self::default();
    map.extend(iter);
    map
  }
}

/// 指针键场景的安全门面：`DenseHashMap::default()` 取代旧式「显式传入空指针
/// 占位键」的构造调用，对应 cpp `DenseHashMap<K*, V>(nullptr)` 惯用法的 Rust 镜像。
///
/// 契约（见 [`crate::records::dense_hash_table`] 模块文档）：占用与否由位图判定，
/// `empty_key` 只是 `Vec<Item>` 空槽的占位值，**不参与命中比较**——因此等于占位值
/// （空指针键场景即 null）的键同样可以正常 get/insert/erase，"哨兵可存取"。
/// 键类型经 [`DenseDefault`] 提供占位值：`*mut T`/`*const T` 取 null，与旧式
/// 显式传空指针的构造逐位等价；非指针键沿用各自 `DenseDefault` 占位。
/// 与 [`DenseHashSet`](crate::records::dense_hash_set::DenseHashSet) 的同名
/// `Default` 门面保持对称。
impl<K, V, H, E> Default for DenseHashMap<K, V, H, E>
where
  K: Clone + DenseDefault,
  V: DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  fn default() -> Self {
    Self::new(K::dense_default())
  }
}
