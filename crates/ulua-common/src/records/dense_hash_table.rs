//! Faithful port of `Luau::detail::DenseHashTable` — the open-addressing core
//! shared by `DenseHashMap`/`DenseHashSet`. Reference:
//! `luau/Common/include/Luau/DenseHash.h` (open addressing, linear probing,
//! empty-key sentinel, erase via backward-shift "algorithm R"). Oracle: the
//! upstream cases in `luau/tests/DenseHash.test.cpp` + a
//! `std::collections::HashMap` differential fuzz (2000 trials × 200 ops).
//!
//! Design (option A — generic functor params): the C++ `Hash`/`Eq` template
//! parameters become the `DenseHasher`/`DenseEq` traits, and the `Set`/`Map`
//! item layout becomes the `ItemInterface` trait. The table stores items in a
//! `Vec<I>` and returns slot indices (`usize`) rather than raw pointers, which
//! keeps the port sound without `unsafe`. `core`/`alloc` only, so the crate
//! stays `wasm32-unknown-unknown` compatible.

use alloc::{
  collections::{BTreeMap, BTreeSet},
  string::String,
  vec::Vec,
};
use core::{
  fmt::{Debug, Formatter, Result as FmtResult},
  marker::PhantomData,
  mem,
  ptr::{null, null_mut},
};

// ---- functor traits (C++ `Hash` / `Eq` template params) ----

/// C++ `rehash` 从空表增长时的初始桶数（DenseHash.h）。
const K_REHASH_INITIAL_BUCKETS: usize = 16;

/// Hash functor, mirroring the C++ `Hash` template parameter.
pub trait DenseHasher<K> {
  fn hash(&self, key: &K) -> usize;
}

/// cpp `doHash` 的黄金比分割常数 `2^64 / φ`（DenseHash.h）。
const FIBONACCI_CONSTANT: u64 = 11400714819323198485;

/// Equality functor, mirroring the C++ `Eq` template parameter.
pub trait DenseEq<K> {
  fn eq(&self, a: &K, b: &K) -> bool;
}

/// Default equality functor used when none is supplied (`std::equal_to<T>`).
#[derive(Clone, Copy)]
pub struct DenseEqDefault<K>(PhantomData<K>);

// Manual impl: the derive would demand `K: Default` (PhantomData needs nothing).
impl<K> Default for DenseEqDefault<K> {
  fn default() -> Self {
    DenseEqDefault(PhantomData)
  }
}

impl<K: PartialEq> DenseEq<K> for DenseEqDefault<K> {
  fn eq(&self, a: &K, b: &K) -> bool {
    a == b
  }
}

// ---- empty-slot value initialization ----

/// Empty-slot value for a `DenseHashMap`, mirroring C++ value-initialization: a
/// freshly grown bucket holds a value-initialized `Value()` — null for pointers,
/// zero for scalars, `Default::default()` for ordinary types. Rust's `Default`
/// is *not* implemented for raw pointers, and the orphan rule forbids adding it,
/// so a blanket `impl<T: Default>` cannot coexist with the pointer impls below
/// (coherence). Hence an explicit trait: pointer and scalar value types are
/// covered here, and any struct value type a map stores supplies its own impl
/// (e.g. `Location` in `luau-ast`).
pub trait DenseDefault {
  fn dense_default() -> Self;
}

impl<T> DenseDefault for *mut T {
  fn dense_default() -> Self {
    null_mut()
  }
}

impl<T> DenseDefault for *const T {
  fn dense_default() -> Self {
    null()
  }
}

macro_rules! dense_default_via_default {
    ($($t:ty),* $(,)?) => {
        $(
            impl DenseDefault for $t {
                fn dense_default() -> Self {
                    <$t as Default>::default()
                }
            }
        )*
    };
}

dense_default_via_default!(
  bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, char,
);

impl DenseDefault for String {
  fn dense_default() -> Self {
    String::new()
  }
}

impl<T> DenseDefault for Vec<T> {
  fn dense_default() -> Self {
    Vec::new()
  }
}

// DenseHashMap 的值类型本身也可以是 map/set
impl<K: Ord, V> DenseDefault for BTreeMap<K, V> {
  fn dense_default() -> Self {
    BTreeMap::new()
  }
}

impl<T: Ord> DenseDefault for BTreeSet<T> {
  fn dense_default() -> Self {
    BTreeSet::new()
  }
}

// A nullable shared pointer (C++ `std::shared_ptr<T>`) defaults to null; the
// Rust mirror `Option<Arc<T>>` defaults to `None`. Used for DenseHashMap values
// whose C++ type is a smart pointer (e.g. `DenseHashMap<K, ScopePtr>`).
impl<T> DenseDefault for Option<T> {
  fn dense_default() -> Self {
    None
  }
}

impl<A: DenseDefault, B: DenseDefault> DenseDefault for (A, B) {
  fn dense_default() -> Self {
    (A::dense_default(), B::dense_default())
  }
}

// ---- item interface (Set: `I = K` ; Map: `I = (K, V)`) ----

/// Bridges the `Set`/`Map` item layout so the table is agnostic to whether a
/// slot stores a bare key or a `(key, value)` pair.
pub trait ItemInterface<K, I> {
  fn get_key(item: &I) -> &K;
  fn set_key(item: &mut I, key: K);
  fn make_empty(empty_key: &K) -> I;
}

/// Set layout: the item *is* the key.
pub struct ItemInterfaceSet<K>(PhantomData<K>);

impl<K: Clone> ItemInterface<K, K> for ItemInterfaceSet<K> {
  fn get_key(item: &K) -> &K {
    item
  }
  fn set_key(item: &mut K, key: K) {
    *item = key;
  }
  fn make_empty(empty_key: &K) -> K {
    empty_key.clone()
  }
}

/// Map layout: the item is a `(key, value)` pair; the value defaults on insert.
pub struct ItemInterfaceMap<K, V>(PhantomData<(K, V)>);

impl<K: Clone, V: DenseDefault> ItemInterface<K, (K, V)> for ItemInterfaceMap<K, V> {
  fn get_key(item: &(K, V)) -> &K {
    &item.0
  }
  fn set_key(item: &mut (K, V), key: K) {
    item.0 = key;
  }
  fn make_empty(empty_key: &K) -> (K, V) {
    (empty_key.clone(), V::dense_default())
  }
}

// ---- the table ----

/// `capacity == data.len()`, always a power of two or 0. A slot is empty iff its
/// key compares equal to `empty_key`.
pub struct DenseHashTable<K, I, Iface, H, E> {
  pub(crate) data: Vec<I>,
  pub(crate) capacity: usize,
  pub(crate) count: usize,
  pub(crate) empty_key: K,
  pub(crate) hasher: H,
  pub(crate) eq: E,
  pub(crate) _iface: PhantomData<Iface>,
}

// The C++ container is copyable; we provide `Clone`/`Debug` by hand (rather than
// `derive`) so the bounds stay precise — `derive` would spuriously require
// `Iface: Clone`/`Debug` on the zero-sized `PhantomData<Iface>` marker, and
// `Debug` would force `H`/`E` (the hash/eq functors) to be `Debug`.
impl<K: Clone, I: Clone, Iface, H: Clone, E: Clone> Clone for DenseHashTable<K, I, Iface, H, E> {
  fn clone(&self) -> Self {
    Self {
      data: self.data.clone(),
      capacity: self.capacity,
      count: self.count,
      empty_key: self.empty_key.clone(),
      hasher: self.hasher.clone(),
      eq: self.eq.clone(),
      _iface: PhantomData,
    }
  }
}

impl<K: Debug, I: Debug, Iface, H, E> Debug for DenseHashTable<K, I, Iface, H, E> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("DenseHashTable")
      .field("data", &self.data)
      .field("capacity", &self.capacity)
      .field("count", &self.count)
      .field("empty_key", &self.empty_key)
      .finish_non_exhaustive()
  }
}

impl<K, I, Iface, H, E> DenseHashTable<K, I, Iface, H, E>
where
  K: Clone,
  Iface: ItemInterface<K, I>,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  /// `DenseHashTable(const Key& empty_key, size_t buckets)`. `buckets` must be
  /// a power of two or 0. Reference: `DenseHash.h` ctor.
  pub fn new(empty_key: K, buckets: usize) -> Self {
    let eq = E::default();
    // equality must at least recognise the sentinel as equal to itself
    debug_assert!(eq.eq(&empty_key, &empty_key));
    debug_assert!(buckets & buckets.wrapping_sub(1) == 0);

    let data = if buckets > 0 {
      (0..buckets)
        .map(|_| Iface::make_empty(&empty_key))
        .collect()
    } else {
      Vec::new()
    };

    DenseHashTable {
      data,
      capacity: buckets,
      count: 0,
      empty_key,
      hasher: H::default(),
      eq,
      _iface: PhantomData,
    }
  }

  /// cpp `doHash`（DenseHash.h）：fibonacci hashing —— 哈希值乘黄金比常数后
  /// 取高 `log2(capacity)` 位。信息经乘法上移，高位散射质量不依赖调用方的
  /// 哈希函数（顺序整数键也能均匀散开）。`capacity` 是 2 的幂；仅在探测路径
  /// 调用（即容量非 0）。
  fn do_hash(&self, key: &K) -> usize {
    debug_assert!(self.capacity > 0);
    let shift = 64 - self.capacity.trailing_zeros();
    ((self.hasher.hash(key) as u64).wrapping_mul(FIBONACCI_CONSTANT) >> shift) as usize
  }

  /// cpp `getBucket`（DenseHash.h:619-641）：key 已存在时返回其槽位，否则
  /// 返回第一个空槽。调用方须保证容量非 0 且表未满（探测必然终止）。
  fn bucket_of(&self, key: &K) -> usize {
    // cpp `DenseHash.h:622` `LUAU_ASSERT(count < capacity)`（"Guarantees that
    // this function will terminate"）：表未满时线性探测必然撞上空槽，故下面的
    // 无界 loop + get_unchecked 成立；容量 0 或表满都会让探测无限循环，
    // 调用方（insert_unsafe / rehash）先经 rehash_if_full 保证此不变式。
    debug_assert!(self.capacity > 0 && self.count < self.capacity);
    let hashmod = self.capacity - 1;
    let mut bucket = self.do_hash(key);
    loop {
      // SAFETY：bucket 经 do_hash 落在 [0, capacity)，且 capacity == data.len()。
      let slot_key = Iface::get_key(unsafe { self.data.get_unchecked(bucket) });
      if self.eq.eq(slot_key, &self.empty_key) || self.eq.eq(slot_key, key) {
        return bucket;
      }
      bucket = (bucket + 1) & hashmod;
    }
  }

  /// Inserts `key` (or finds it if present) without checking load factor, and
  /// returns the slot index. The caller must `rehash_if_full` first.
  pub(crate) fn insert_unsafe(&mut self, key: K) -> usize {
    debug_assert!(!self.eq.eq(&key, &self.empty_key));
    let bucket = self.bucket_of(&key);
    // SAFETY：bucket < capacity == data.len()。
    let slot = unsafe { self.data.get_unchecked_mut(bucket) };
    if self.eq.eq(Iface::get_key(slot), &self.empty_key) {
      Iface::set_key(slot, key);
      self.count += 1;
    }
    bucket
  }

  /// Returns the slot index of `key` if present.
  pub(crate) fn find(&self, key: &K) -> Option<usize> {
    if self.count == 0 {
      return None;
    }
    if self.eq.eq(key, &self.empty_key) {
      return None;
    }
    let hashmod = self.capacity - 1;
    let mut bucket = self.do_hash(key);
    for _probe in 0..=hashmod {
      // SAFETY：bucket = doHash(key) < capacity == data.len()。
      let probe_key = Iface::get_key(unsafe { self.data.get_unchecked(bucket) });
      if self.eq.eq(probe_key, key) {
        return Some(bucket);
      }
      if self.eq.eq(probe_key, &self.empty_key) {
        return None;
      }
      bucket = (bucket + 1) & hashmod;
    }
    unreachable!(
      "dense hash table is full: capacity={}, count={}, occupied={}",
      self.capacity,
      self.count,
      self.occupied()
    );
  }

  /// `erase` — 删除 `key` 所在槽位。为维持开放定址不变式（每个元素都可从其
  /// 初始桶沿探测序列到达），删除后把探测链上"会被空洞截断搜索"的后续元素
  /// 逐个前移填洞：TAOCP Vol.3 6.4 算法 R 的 backward-shift 删除。空洞 `i`
  /// 落在元素 `j` 的初始桶 `r` 与 `j` 之间（沿探测方向）当且仅当
  /// `(i-r) & mask < (j-r) & mask`（线性探测下位移序即步序），此时必须把
  /// `j` 前移，空洞随之转移到 `j` 的旧槽。Reference: `DenseHash.h:414-422,
  /// 643-684`。
  pub(crate) fn erase(&mut self, key: &K) {
    if self.count == 0 {
      return;
    }
    let Some(bucket) = self.find(key) else {
      return;
    };

    let hashmod = self.capacity - 1;
    let mut i = bucket;
    let mut j = bucket;
    loop {
      j = (j + 1) & hashmod;
      // 空槽即探测链终点：空洞留在原地，收尾时清空。
      if self.eq.eq(Iface::get_key(&self.data[j]), &self.empty_key) {
        break;
      }
      let r = self.do_hash(Iface::get_key(&self.data[j]));
      let left = i.wrapping_sub(r) & hashmod;
      let right = j.wrapping_sub(r) & hashmod;
      if left < right {
        // j 前移填洞；空洞转移到 j 旧槽（置回 sentinel，等价 cpp 的
        // usedTable.set(j, false) + move 搬运）。
        let item = mem::replace(&mut self.data[j], Iface::make_empty(&self.empty_key));
        self.data[i] = item;
        i = j;
      }
    }
    self.data[i] = Iface::make_empty(&self.empty_key);
    self.count -= 1;
  }

  /// Grows to [`K_REHASH_INITIAL_BUCKETS`] (from empty) or `2×` capacity,
  /// re-inserting live items.
  pub(crate) fn rehash(&mut self) {
    let newsize = if self.capacity == 0 {
      K_REHASH_INITIAL_BUCKETS
    } else {
      self.capacity * 2
    };
    let mut newtable = Self::new(self.empty_key.clone(), newsize);
    // cpp `grow` 用 `getBucket` 探测目标槽后整项 move（DenseHash.h:454-464），
    // 不经 `insert_unsafe` 的 setKey——避免 key 的一次 clone 分配。
    for item in self.data.iter_mut() {
      if !self.eq.eq(Iface::get_key(item), &self.empty_key) {
        let bucket = newtable.bucket_of(Iface::get_key(item));
        newtable.count += 1;
        newtable.data[bucket] = mem::replace(item, Iface::make_empty(&self.empty_key));
      }
    }
    debug_assert_eq!(self.count, newtable.count);
    mem::swap(&mut self.data, &mut newtable.data);
    mem::swap(&mut self.capacity, &mut newtable.capacity);
  }

  /// Rehashes before an insert that would push past the 3/4 load factor — but
  /// only when `key` is genuinely new. The `find` guard is load-bearing:
  /// overwriting an existing key when full must NOT rehash (upstream relies on
  /// iterators surviving an overwrite-merge).
  pub(crate) fn rehash_if_full(&mut self, key: &K) {
    if self.count >= self.capacity * 3 / 4 && self.find(key).is_none() {
      self.rehash();
    }
  }

  /// `clear(thresholdToDestroy = 32)`（cpp `DenseHash.h:360-377`）：容量超过
  /// 阈值时连同存储一起释放（退回空表，cpp 的 `destroy()` 路径），否则仅把
  /// 槽位重置回空标记、保留已分配容量。
  pub(crate) fn clear(&mut self) {
    const THRESHOLD_TO_DESTROY: usize = 32;
    if self.count == 0 {
      return;
    }
    if self.capacity > THRESHOLD_TO_DESTROY {
      self.data = Vec::new();
      self.capacity = 0;
    } else {
      for slot in self.data.iter_mut() {
        *slot = Iface::make_empty(&self.empty_key);
      }
    }
    self.count = 0;
  }

  pub(crate) fn size(&self) -> usize {
    self.count
  }

  /// Index of the first occupied slot (== `capacity` when empty). Iterator seed.
  pub(crate) fn first_occupied(&self) -> usize {
    self
      .data
      .iter()
      .position(|item| !self.eq.eq(Iface::get_key(item), &self.empty_key))
      .unwrap_or(self.capacity)
  }

  /// Index of the next occupied slot strictly after `index`.
  pub(crate) fn next_occupied(&self, mut index: usize) -> usize {
    loop {
      index += 1;
      if index >= self.capacity
        || !self
          .eq
          .eq(Iface::get_key(&self.data[index]), &self.empty_key)
      {
        break;
      }
    }
    index
  }

  /// 占用（非 sentinel）槽位数，仅用于表满不可达路径的诊断信息。
  fn occupied(&self) -> usize {
    self
      .data
      .iter()
      .filter(|item| !self.eq.eq(Iface::get_key(item), &self.empty_key))
      .count()
  }
}
