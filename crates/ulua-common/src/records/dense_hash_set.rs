//! Faithful port of `Luau::DenseHashSet` — a fast `unordered_set` alternative
//! that supports erase (TAOCP algorithm R backward-shift). Reference:
//! `luau/Common/include/Luau/DenseHash.h:727-823`.
//!
//! 构造仍收一个 `empty_key`：上游新版已改用来给空槽填占位值（占用判定由位图
//! 负责），因此等于 `empty_key` 的键也可以正常插入/查回；保留该参数是为了对齐
//! 下游数百处 `DenseHashSet<T>{emptyT}` 构造点，详见
//! [`crate::records::dense_hash_table`] 的模块文档。

use core::fmt::{Debug, Formatter, Result};

use crate::{
  records::{
    const_iterator::ConstIterator,
    dense_hash_table::{
      DenseDefault, DenseEq, DenseEqDefault, DenseHashTable, DenseHasher, ItemInterfaceSet,
    },
  },
  type_aliases::dense_hash_default::{DenseHashDefault, dense_hash_of},
};

type SetImpl<K, H, E> = DenseHashTable<K, K, ItemInterfaceSet<K>, H, E>;

pub struct DenseHashSet<K, H = DenseHashDefault<K>, E = DenseEqDefault<K>> {
  pub(crate) impl_: SetImpl<K, H, E>,
}

impl<K: Clone, H: Clone, E: Clone> Clone for DenseHashSet<K, H, E> {
  fn clone(&self) -> Self {
    DenseHashSet {
      impl_: self.impl_.clone(),
    }
  }
}

impl<K: Debug, H, E> Debug for DenseHashSet<K, H, E> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("DenseHashSet")
      .field("impl_", &self.impl_)
      .finish()
  }
}

impl<K, H, E> DenseHashSet<K, H, E>
where
  K: Clone,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  /// `DenseHashSet(empty_key, buckets = 0)`. Reference: `DenseHash.h:737-740`.
  pub fn new(empty_key: K) -> Self {
    DenseHashSet {
      impl_: DenseHashTable::new(empty_key, 0),
    }
  }

  /// `insert` — inserts `key` if absent and returns a reference to the stored
  /// key. Reference: `DenseHash.h:747-751`.
  pub fn insert(&mut self, key: K) -> &K {
    self.impl_.rehash_if_full(&key);
    let idx = self.impl_.insert_unsafe(key);
    &self.impl_.data[idx]
  }

  /// `try_insert` — inserts `key` if absent; returns true only when the key
  /// was newly inserted. Reference: `DenseHash.h:753-758`.
  pub fn try_insert(&mut self, key: K) -> bool {
    self.impl_.rehash_if_full(&key);

    let before = self.impl_.size();
    self.impl_.insert_unsafe(key);

    self.impl_.size() > before
  }

  /// `const Key* find(const Key&) const`. Reference: `DenseHash.h:760-763`.
  pub fn find(&self, key: &K) -> Option<&K> {
    self.impl_.find(key).map(|idx| &self.impl_.data[idx])
  }

  /// std-style alias for generated Rust that spelled C++ `find` as `get`.
  pub fn get(&self, key: &K) -> Option<&K> {
    self.find(key)
  }

  /// Mutable analogue of `insert`: inserts `key` if absent and returns a
  /// mutable reference to the stored key. C++ `insert` returns `const Key&`
  /// and callers reach for `const_cast<Key&>` when they need to fix up a
  /// stored entry in place (e.g. `AstNameTable::getOrAddWithType` rewriting a
  /// non-owned name pointer to an allocator-owned copy of the same bytes —
  /// the hash/eq are unchanged so the slot stays valid). This exposes that
  /// mutation soundly. Not a separate C++ method; the mutable access is the
  /// faithful Rust spelling of the `const_cast` idiom.
  pub fn insert_mut(&mut self, key: K) -> &mut K {
    self.impl_.rehash_if_full(&key);
    let idx = self.impl_.insert_unsafe(key);
    &mut self.impl_.data[idx]
  }

  /// Mutable analogue of `find`.
  pub fn find_mut(&mut self, key: &K) -> Option<&mut K> {
    self.impl_.find(key).map(|idx| &mut self.impl_.data[idx])
  }

  /// `contains`. Reference: `DenseHash.h:765-768`.
  pub fn contains(&self, key: &K) -> bool {
    self.impl_.find(key).is_some()
  }

  /// `erase`。Reference: `DenseHash.h:770-773`。
  pub fn erase(&mut self, key: &K) {
    self.impl_.erase(key);
  }

  /// `size`. Reference: `DenseHash.h:775-778`.
  pub fn size(&self) -> usize {
    self.impl_.size()
  }

  /// std-style alias for `size()`.
  #[inline]
  pub fn len(&self) -> usize {
    self.size()
  }

  /// `empty`. Reference: `DenseHash.h:780-783`.
  pub fn empty(&self) -> bool {
    self.impl_.size() == 0
  }

  /// std-style alias for `empty()`.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.empty()
  }

  /// `clear`. Reference: `DenseHash.h:742-745`.
  pub fn clear(&mut self) {
    self.impl_.clear();
  }

  /// `begin()/end()` iteration, yielding `&Key`.
  pub fn iter(&self) -> ConstIterator<'_, K> {
    self.impl_.iter()
  }
}

/// `String` 元素 + 默认 functor 的 `&str` 借用视图查询口（r7-rc-4 形状口），
/// 与 [`DenseHashMap`](crate::records::dense_hash_map::DenseHashMap) 的同名口
/// 同一论证：hash 经 [`dense_hash_of`] 对 `String`/`str` 逐位一致，eq 为字节
/// 相等；定制 functor 容器刻意不给（详见 dense_hash_map.rs 专化 impl 文档）。
/// 既有 `&String` 口一行不改。
///
/// [`dense_hash_of`]: crate::type_aliases::dense_hash_default::dense_hash_of
impl DenseHashSet<String, DenseHashDefault<String>, DenseEqDefault<String>> {
  /// `find` 的 `&str` 借用口。Reference: `DenseHash.h:760-763`。
  pub fn find_str(&self, key: &str) -> Option<&String> {
    self
      .impl_
      .find_by_view(dense_hash_of(key), |stored| stored.as_str() == key)
      .map(|idx| &self.impl_.data[idx])
  }

  /// std-style alias for generated Rust that spelled C++ `find` as `get`.
  pub fn get_str(&self, key: &str) -> Option<&String> {
    self.find_str(key)
  }

  /// `find_mut` 的 `&str` 借用口。
  pub fn find_mut_str(&mut self, key: &str) -> Option<&mut String> {
    let idx = self
      .impl_
      .find_by_view(dense_hash_of(key), |stored| stored.as_str() == key)?;
    Some(&mut self.impl_.data[idx])
  }

  /// `contains` 的 `&str` 借用口。Reference: `DenseHash.h:765-768`。
  pub fn contains_str(&self, key: &str) -> bool {
    self
      .impl_
      .find_by_view(dense_hash_of(key), |stored| stored.as_str() == key)
      .is_some()
  }

  /// `erase` 的 `&str` 借用口。Reference: `DenseHash.h:770-773`。
  pub fn erase_str(&mut self, key: &str) {
    self
      .impl_
      .erase_by_view(dense_hash_of(key), |stored| stored.as_str() == key);
  }
}

impl<'a, K, H, E> IntoIterator for &'a DenseHashSet<K, H, E>
where
  K: Clone,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  type Item = &'a K;
  type IntoIter = ConstIterator<'a, K>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<K, H, E> Extend<K> for DenseHashSet<K, H, E>
where
  K: Clone,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  fn extend<I: IntoIterator<Item = K>>(&mut self, iter: I) {
    for item in iter {
      self.insert(item);
    }
  }
}

impl<'a, K, H, E> Extend<&'a K> for DenseHashSet<K, H, E>
where
  K: Clone,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  fn extend<I: IntoIterator<Item = &'a K>>(&mut self, iter: I) {
    for item in iter {
      self.insert(item.clone());
    }
  }
}

impl<K, H, E> FromIterator<K> for DenseHashSet<K, H, E>
where
  K: Clone + DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
    let mut set = Self::default();
    set.extend(iter);
    set
  }
}

/// `operator==` / `operator!=` — set equality. Reference: `DenseHash.h:805-822`.
impl<K, H, E> PartialEq for DenseHashSet<K, H, E>
where
  K: Clone,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  fn eq(&self, other: &Self) -> bool {
    if self.size() != other.size() {
      return false;
    }
    for k in self.iter() {
      if !other.contains(k) {
        return false;
      }
    }
    true
  }
}

/// 指针键场景的安全门面：`DenseHashSet::default()` 取代旧式「显式传入空指针
/// 占位键」的构造调用，对应 cpp `DenseHashSet<K*>({nullptr})` 惯用法的 Rust
/// 镜像（C++ 成员把空键内联写成
/// `DenseHashSet<T> x{emptyT}`，外层结构体 `#[derive(Default)]` 也需要本实现）。
///
/// 契约（见 [`crate::records::dense_hash_table`] 模块文档）：占用与否由位图判定，
/// `empty_key` 只是空槽占位值，**不参与命中比较**——因此等于占位值（空指针键场景
/// 即 null）的键同样可以正常 insert/find/erase，"哨兵可存取"。占位值来自键类型
/// 的 [`DenseDefault`]：`*mut T`/`*const T` 取 null，与 [`DenseHashMap`] 的同名
/// `Default` 门面保持对称。
///
/// [`DenseHashMap`]: crate::records::dense_hash_map::DenseHashMap
impl<K, H, E> Default for DenseHashSet<K, H, E>
where
  K: Clone + DenseDefault,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  fn default() -> Self {
    Self::new(K::dense_default())
  }
}
