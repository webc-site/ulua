//! Faithful port of `Luau::detail::DenseHashTable` — the open-addressing core
//! shared by `DenseHashMap`/`DenseHashSet`. Reference:
//! `cpp/Common/include/Luau/DenseHash.h`（当前上游实现：BitSet 占用位图 +
//! fibonacci hashing + 线性探测 + TAOCP Vol.3 6.4-R backward-shift erase）。
//!
//! 设计（option A — generic functor params）：C++ 的 `Hash`/`Eq` 模板参数落成
//! `DenseHasher`/`DenseEq` trait，`Set`/`Map` 的 item 布局落成 `ItemInterface`；
//! 表把 item 存在 `Vec<I>` 里并返回槽号（`usize`）而非裸指针，因此业务层没有
//! 指针别名问题，`unsafe` 只剩探测热路径上两处带契约的 `get_unchecked`
//! （桶号由 `& (capacity-1)` 回绕，界内可静态论证）。
//! 只用 `core`/`alloc`，保持 `wasm32-unknown-unknown` 可编。
//!
//! 与上游的两处结构性差异（Rust 语义所需，行为等价）：
//! - **DELIBERATE DEVIATION**（出处 `DenseHash.h:265-283` 构造 + `:717-722`
//!   `ItemInterfaceMap2::setKey`）：上游用 `::operator new` + placement new/destroy
//!   管理**未初始化**槽位；Rust 的 `Vec<I>` 要求每个空槽都持有合法值，所以保留
//!   `empty_key` 作为空槽**占位值**的来源（`ItemInterface::make_empty`）。它不再参与
//!   "槽位是否已占用"的判定——那是位图的职责——所以**等于 `empty_key` 的键同样可以
//!   存取**（旧代哨兵实现做不到）。相应地构造签名是 `new(empty_key, buckets)`，比上游
//!   的 `DenseHashTable(buckets)` 多一个占位值参数；下游按上游签名对齐时注意这点差异。
//! - **DELIBERATE DEVIATION**（出处 `DenseHash.h:424-445`）：上游 `insert_unsafe`/
//!   `find` 返回 `Item*`，这里返回槽号，由 map/set 包装层换算成引用，避免把别名规则
//!   外泄给调用方（§2 裸指针收口）。
//! - cpp `clear(size_t thresholdToDestroy = 32)` 的增长阈值实参在 Rust 侧落成常量
//!   `K_THRESHOLD_TO_DESTROY`：全仓 cpp 调用点（含 `tests/`）无一处传入非默认值，
//!   故省略参数不改变任何可观察行为。

use alloc::{
  collections::{BTreeMap, BTreeSet},
  string::String,
  vec::Vec,
};
use core::{
  fmt::{Debug, Formatter, Result as FmtResult},
  iter::{Enumerate, FusedIterator},
  marker::PhantomData,
  mem,
  ptr::{null, null_mut},
  slice,
};

use crate::records::{const_iterator::ConstIterator, iterator::MutIterator, variant::Variant3};

// ---- cpp 对齐的增长/容量常数（DenseHash.h 的 `K_*` 与位图字参数） ----

/// cpp `grow` 从空表增长时的初始桶数（DenseHash.h:449 `capacity == 0 ? 16`）。
const K_GROW_INITIAL_BUCKETS: usize = 16;
/// cpp `grow` 的增长倍率（DenseHash.h:449 `capacity * 2`）：翻倍以摊销查找成本。
const K_GROW_FACTOR: usize = 2;
/// cpp `rehash_if_full` 的负载因子分子/分母（DenseHash.h:476 `capacity * 3 / 4`）：
/// 占用数达到容量的 3/4 即增长。
const LOAD_FACTOR_NUM: usize = 3;
const LOAD_FACTOR_DEN: usize = 4;
/// cpp `clear(size_t thresholdToDestroy = 32)` 的默认阈值（DenseHash.h:360,841）：
/// 容量超过它时连同存储一起释放，否则保留已分配容量。
const K_THRESHOLD_TO_DESTROY: usize = 32;

/// 3/4 负载因子下的增长阈值（cpp `capacity * 3 / 4`，整数除法向零取整）。
const fn grow_threshold(capacity: usize) -> usize {
  capacity * LOAD_FACTOR_NUM / LOAD_FACTOR_DEN
}

/// cpp `BitSet::numElements`（DenseHash.h:72 `sizeof(BitsetT) * 8`）：一个位图字
/// 覆盖的桶数。
const BITS_PER_WORD: usize = u64::BITS as usize;
/// cpp `BitSet::numElementsLog2`（DenseHash.h:76，上游手写死 6）：字桶数的 log2，
/// 这里由 `BITS_PER_WORD` 编译期反推，换 backing 字类型时自动跟随。
const WORD_LOG2: usize = BITS_PER_WORD.trailing_zeros() as usize;
/// cpp `static_assert((numElements & (numElements - 1)) == 0)`（DenseHash.h:74）：
/// 字桶数必须是 2 的幂，否则 `>> WORD_LOG2` / `& (BITS_PER_WORD - 1)` 的分桶算式失效。
const _: () = assert!(BITS_PER_WORD.is_power_of_two(), "位图字容量必须是 2 的幂");

// ---- functor traits (C++ `Hash` / `Eq` template params) ----

/// Hash functor, mirroring the C++ `Hash` template parameter.
pub trait DenseHasher<K> {
  fn hash(&self, key: &K) -> usize;
}

/// cpp `doHash` 的黄金比分割常数 `2^64 / φ`（DenseHash.h:405 `11400714819323198485ull`）：
/// 乘法把信息上移，随后按 `hashShift` 取高位即得桶号。
const FIBONACCI_CONSTANT: u64 = 11400714819323198485;
/// cpp `hashShift` 的位宽基（DenseHash.h:281 `64 - ctz(buckets)`、初值 64）：
/// 即 `u64::BITS`，编译期常量。
const HASH_SHIFT_BASE: u32 = u64::BITS;

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
    // 保留空指针：键/值类型面本身就是 `*mut T`（cpp 指针键 DenseHash 的移植），
    // 空槽占位必须与 cpp 值初始化的 nullptr 逐位一致；改 Option 会把类型外泄给
    // 全部指针键调用点，且位图判定下该占位不参与比较，无 null 哨兵判空逻辑。
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

// cpp `Variant()` 默认构造即首成员值初始化：`Variant3` 的占位哨兵只由 T0 分量
// 决定（`V0(T0::dense_default())`），与 variant.rs 的 `Default` 生成形态同构。
// 指针分量实例（如 analysis `BlockedConstraintId = Variant3<TypeId, ..>`，
// TypeId = `*const Type`）即得 `V0(null)`，与旧调用点内联传参
// `new(V0(null::<Type>()))` 逐位等价。
// 契约：哨兵非占用哨兵——槽位占用由位图判定（见本模块文档），等于哨兵的键
// 照常可插入/命中/擦除，与真实键共存。
impl<T0: DenseDefault, T1, T2> DenseDefault for Variant3<T0, T1, T2> {
  fn dense_default() -> Self {
    Self::V0(T0::dense_default())
  }
}

// ---- item interface (Set: `I = K` ; Map: `I = (K, V)`) ----

/// Bridges the `Set`/`Map` item layout so the table is agnostic to whether a
/// slot stores a bare key or a `(key, value)` pair.
pub trait ItemInterface<K, I> {
  /// cpp `ItemInterface::getKey`。
  fn get_key(item: &I) -> &K;
  /// cpp `ItemInterface::setKey`：键写入槽位，值保持槽位原有的占位值
  /// （上游 map 版在此默认构造 value）。
  fn set_key(item: &mut I, key: K);
  /// 空槽占位值。仅用于给 `Vec<I>` 的空槽一个合法值，不参与占用判定。
  fn make_empty(empty_key: &K) -> I;
}

/// Set layout: the item *is* the key. 仅经私有条别名 `SetImpl` 装配
/// `DenseHashSet`，降 `pub(crate)`（b28 零消费点收口）。
pub(crate) struct ItemInterfaceSet<K>(PhantomData<K>);

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

// ---- used-table bitset (cpp `DenseHashTable::BitSet`, DenseHash.h:62-259) ----

/// 占用位图：第 `i` 个桶是否已被元素占据。每个 `u64` 覆盖 64 个桶。
/// 取代旧代实现的"哨兵键比较"，于是
/// - 查找/擦除不再需要与 `empty_key` 比较（键可以与 `empty_key` 相同）；
/// - 迭代与清空按字跳过空槽，不再是 O(capacity) 次键比较。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct BitSet {
  words: Vec<u64>,
}

impl BitSet {
  /// cpp `BitSet(size_t capacity)`：`capacity` 为 2 的幂或 0，字数组按 64 桶向上
  /// 取整（上游 `capacity < 64 ? 1 : capacity >> 6`，与 `div_ceil` 对 2 的幂等价）；
  /// 空表为 0 个字——所有探测路径都受 `count < capacity` 不变式保护，不会索引它。
  fn with_capacity(capacity: usize) -> Self {
    debug_assert!(capacity == 0 || capacity & (capacity - 1) == 0);
    BitSet {
      words: vec![0; capacity.div_ceil(BITS_PER_WORD)],
    }
  }

  /// cpp `contains(bucket)`。`bucket < capacity` 由表不变式保证。
  #[inline]
  fn contains(&self, bucket: usize) -> bool {
    self.words[bucket >> WORD_LOG2] >> (bucket & (BITS_PER_WORD - 1)) & 1 == 1
  }

  /// cpp `set(bucket, v)`。
  #[inline]
  fn set(&mut self, bucket: usize, value: bool) {
    let (word, mask) = split(bucket);
    if value {
      self.words[word] |= mask;
    } else {
      self.words[word] &= !mask;
    }
  }

  /// cpp `clear()`：只清位，不释放字数组。
  fn clear(&mut self) {
    self.words.iter_mut().for_each(|word| *word = 0);
  }

  /// cpp `begin()/end()`：产出所有置位桶号（升序）。
  fn bits(&self) -> Bits<'_> {
    Bits {
      words: self.words.iter().enumerate(),
      word: 0,
      base: 0,
    }
  }
}

/// `bucket` 所在字下标与该位的掩码（cpp 的 `whichBitvec` / `1 << offset`）。
#[inline]
fn split(bucket: usize) -> (usize, u64) {
  (bucket >> WORD_LOG2, 1u64 << (bucket & (BITS_PER_WORD - 1)))
}

/// cpp `BitSet::iterator`：流式产出置位桶号。每轮用 `trailing_zeros` 定位当前字
/// 内最低位，并用 `word &= word - 1` 抹掉已产出的位，从而按字（而非按桶）推进；
/// 字序列由切片迭代器供给，全空字由循环自然跳过，不留下标与越界检查。
pub(crate) struct Bits<'a> {
  /// 尚未取出的字（带字下标，用于换算首桶号）。
  words: Enumerate<slice::Iter<'a, u64>>,
  /// 正在产出的字，为 0 表示需要换字。
  word: u64,
  /// `word` 首桶的桶号。
  base: usize,
}

impl<'a> Iterator for Bits<'a> {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    loop {
      if self.word != 0 {
        // `word != 0`，`trailing_zeros` 必落在 [0, 64)。
        let bit = self.word.trailing_zeros() as usize;
        self.word &= self.word - 1;
        return Some(self.base + bit);
      }
      // 换字：整字为空的块由循环自然跳过（cpp: `while (word == 0) { wordIdx++; … }`）。
      let (word_idx, word) = self.words.next()?;
      self.word = *word;
      self.base = word_idx * BITS_PER_WORD;
    }
  }
}

/// 字内取最低置位、字间升序推进 → 桶号严格升序；耗尽后恒返回 `None`。
impl FusedIterator for Bits<'_> {}

// ---- the table ----

/// 开放定址核心表。不变式：`capacity == data.len() == used 覆盖的桶数`，
/// 且是 2 的幂或 0；`count` 为置位数，始终 `< capacity`（3/4 负载因子保证）。
pub struct DenseHashTable<K, I, Iface, H, E> {
  pub(crate) data: Vec<I>,
  /// cpp `usedTable`。
  pub(crate) used: BitSet,
  pub(crate) count: usize,
  /// 空槽占位值的键部分（见模块文档：不是哨兵，不参与占用判定）。
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
      used: self.used.clone(),
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
      .field("count", &self.count)
      .field("capacity", &self.data.len())
      .field("used", &self.used)
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
  /// `DenseHashTable(const Key& empty_key, size_t buckets)`。`buckets` 必须是
  /// 2 的幂或 0。Reference: `DenseHash.h:265-283`。
  pub fn new(empty_key: K, buckets: usize) -> Self {
    debug_assert!(buckets & buckets.wrapping_sub(1) == 0);

    DenseHashTable {
      // cpp 只分配未初始化存储；Rust 侧每个空槽放一份占位值。
      data: (0..buckets)
        .map(|_| Iface::make_empty(&empty_key))
        .collect(),
      used: BitSet::with_capacity(buckets),
      count: 0,
      empty_key,
      hasher: H::default(),
      eq: E::default(),
      _iface: PhantomData,
    }
  }

  /// 对应 cpp `DenseHashTable(size_t buckets)` 构造器的桶预留语义
  /// （`DenseHash.h:265-283`）：仅当表为空且现有桶数不足时扩到 `buckets`
  /// （须为 2 的幂或 0）；非空表为 no-op，避免丢项。
  pub(crate) fn reserve_buckets(&mut self, buckets: usize) {
    debug_assert!(buckets & buckets.wrapping_sub(1) == 0);
    if self.count == 0 && self.data.len() < buckets {
      self.data = (0..buckets)
        .map(|_| Iface::make_empty(&self.empty_key))
        .collect();
      self.used = BitSet::with_capacity(buckets);
    }
  }

  /// cpp `doHash`（DenseHash.h:392-406）：fibonacci hashing —— 哈希值乘黄金比常数
  /// 后取高 `log2(capacity)` 位。信息经乘法上移，高位散射质量不依赖调用方的哈希
  /// 函数（顺序整数键也能均匀散开）。容量是 2 的幂；仅在探测路径调用（容量非 0）。
  fn do_hash(&self, key: &K) -> usize {
    debug_assert!(!self.data.is_empty());
    let shift = HASH_SHIFT_BASE - self.data.len().trailing_zeros();
    ((self.hasher.hash(key) as u64).wrapping_mul(FIBONACCI_CONSTANT) >> shift) as usize
  }

  /// cpp `getBucket`（DenseHash.h:619-641）：返回 `(槽位, 是否命中)`——未命中时
  /// 给出探测链上的第一个空槽。`count < capacity` 保证线性探测必然遇到空槽。
  fn bucket_of(&self, key: &K) -> (usize, bool) {
    debug_assert!(self.count < self.data.len());
    let hashmod = self.data.len() - 1;
    let mut bucket = self.do_hash(key);
    loop {
      if !self.used.contains(bucket) {
        return (bucket, false);
      }
      // Safety: bucket 由 do_hash 取高 log2(len) 位后按 `& hashmod` 递增回绕，
      // 恒落在 [0, capacity) == [0, data.len())，故 get_unchecked 索引界内。
      let slot = unsafe { self.data.get_unchecked(bucket) };
      if self.eq.eq(Iface::get_key(slot), key) {
        return (bucket, true);
      }
      bucket = (bucket + 1) & hashmod;
    }
  }

  /// Inserts `key` (or finds it if present) without checking load factor, and
  /// returns the slot index. The caller must `rehash_if_full` first.
  pub(crate) fn insert_unsafe(&mut self, key: K) -> usize {
    let (bucket, found) = self.bucket_of(&key);
    if !found {
      self.used.set(bucket, true);
      Iface::set_key(&mut self.data[bucket], key);
      self.count += 1;
    }
    bucket
  }

  /// Returns the slot index of `key` if present.
  pub(crate) fn find(&self, key: &K) -> Option<usize> {
    if self.count == 0 {
      return None;
    }
    let (bucket, found) = self.bucket_of(key);
    found.then_some(bucket)
  }

  /// cpp `erase`（DenseHash.h:414-422）。
  pub(crate) fn erase(&mut self, key: &K) {
    if self.count == 0 {
      return;
    }
    let (bucket, found) = self.bucket_of(key);
    if found {
      self.do_erase(bucket);
    }
  }

  /// cpp `doErase`（DenseHash.h:643-684，TAOCP Vol.3 6.4 算法 R 的
  /// backward-shift）：沿探测方向把"空洞会截断其搜索"的后续元素逐个前移。
  /// 判据：元素在 `j`、其初始桶为 `r`、空洞在 `i`，当且仅当
  /// `(i-r) & hashmod < (j-r) & hashmod` 时前移 `j`（线性探测下位移序即步序），
  /// 空洞随之转移到 `j`。
  fn do_erase(&mut self, bucket: usize) {
    let hashmod = self.data.len() - 1;
    let mut i = bucket;
    let mut j = bucket;
    loop {
      j = (j + 1) & hashmod;
      // 空槽即探测链终点：空洞留在原地，收尾时清空。
      if !self.used.contains(j) {
        break;
      }
      // Safety: j 由 `& hashmod` 回绕，恒 < capacity == data.len()，索引界内。
      let key = Iface::get_key(unsafe { self.data.get_unchecked(j) });
      let r = self.do_hash(key);
      if i.wrapping_sub(r) & hashmod < j.wrapping_sub(r) & hashmod {
        // cpp: `data[i].~Item(); new (&data[i]) Item(std::move(data[j]))`。
        // Rust 用 swap 表达"搬运 + 留下待销毁项"：被删项随空洞一起后移，循环
        // 结束后一次性复位为空槽值（即销毁），避免每轮构造一份占位值。
        self.data.swap(i, j);
        self.used.set(i, true);
        self.used.set(j, false);
        i = j;
      }
    }

    self.used.set(i, false);
    self.data[i] = Iface::make_empty(&self.empty_key);
    self.count -= 1;
  }

  /// cpp `grow`（DenseHash.h:447-472）：从空表增长到
  /// [`K_GROW_INITIAL_BUCKETS`]，否则容量翻倍，整项搬运占用槽。
  pub(crate) fn grow(&mut self) {
    let newsize = if self.data.is_empty() {
      K_GROW_INITIAL_BUCKETS
    } else {
      self.data.len() * K_GROW_FACTOR
    };
    let mut newtable = Self::new(self.empty_key.clone(), newsize);

    // cpp 用 `for (size_t bucket : usedTable)` 按位图跳过空槽；Rust 侧要搬运
    // 所有项才能释放 `data`，故整表一次 `into_iter`，空槽的占位值直接丢弃。
    let used = mem::take(&mut self.used);
    let data = mem::take(&mut self.data);
    for (bucket, item) in data.into_iter().enumerate() {
      if !used.contains(bucket) {
        continue;
      }
      // cpp: 手动 getBucket + set bit + move 构造，不经 `insert_unsafe` 的
      // setKey —— 免得为已存在的键多做一次占位/克隆。
      let key = Iface::get_key(&item);
      let (dest, found) = newtable.bucket_of(key);
      debug_assert!(!found);
      newtable.used.set(dest, true);
      newtable.data[dest] = item;
      newtable.count += 1;
    }
    debug_assert_eq!(self.count, newtable.count);

    self.used = newtable.used;
    self.data = newtable.data;
  }

  /// Rehashes before an insert that would push past the 3/4 load factor — but
  /// only when `key` is genuinely new. The `find` guard is load-bearing:
  /// overwriting an existing key when full must NOT rehash (upstream relies on
  /// iterators surviving an overwrite-merge).
  pub(crate) fn rehash_if_full(&mut self, key: &K) {
    if self.count >= grow_threshold(self.data.len()) && self.find(key).is_none() {
      self.grow();
    }
  }

  /// cpp `clear(thresholdToDestroy = 32)`（DenseHash.h:360-377,841-844）：容量超过
  /// [`K_THRESHOLD_TO_DESTROY`] 时连同存储一起释放（cpp 的 `destroy()` 路径），
  /// 否则只销毁占用槽里的元素并清空位图，保留已分配容量。
  pub(crate) fn clear(&mut self) {
    if self.count == 0 {
      return;
    }
    if self.data.len() > K_THRESHOLD_TO_DESTROY {
      self.data = Vec::new();
      self.used = BitSet::default();
    } else {
      // cpp: `for (size_t bucket : usedTable) data[bucket].~Item();`
      // 解构出互不相交的字段借用：`bits()` 只读位图与 `empty_key`、写入
      // `data`，循环结束后再复位位图，免掉中间 Vec 桶号缓存。
      let Self {
        data,
        used,
        empty_key,
        ..
      } = self;
      for bucket in used.bits() {
        data[bucket] = Iface::make_empty(empty_key);
      }
      used.clear();
    }
    self.count = 0;
  }

  pub(crate) fn size(&self) -> usize {
    self.count
  }

  /// cpp `begin()/end()`（const 版）：按位图升序产出占用槽里的元素。
  pub(crate) fn iter(&self) -> ConstIterator<'_, I> {
    ConstIterator {
      items: &self.data,
      buckets: self.used.bits(),
    }
  }

  /// cpp `begin()/end()`（可变版）：同上，产出 `&mut I`。
  ///
  /// cpp 的 `iterator` 直接持有 `Item *data`，`operator*` 用 `*bucketIt` 索引；
  /// Rust 侧同一件事要拆成"可变切片迭代器 + 只读位图"两个借用（解构 `&mut self`
  /// 拿到互不相交的字段借用），见 [`MutIterator`]。
  pub(crate) fn iter_mut(&mut self) -> MutIterator<'_, I> {
    let Self { data, used, .. } = self;
    MutIterator {
      items: data.iter_mut(),
      buckets: used.bits(),
      consumed: 0,
    }
  }
}

// §8：`BitSet` 的字数组与 `bits()` 是私有物理布局，外部测试无从构造/读取，
// 故留在 src（`tests/dense_hash.rs` 的公开迭代 oracle 只覆盖到"无中间空字"的形状）。
#[cfg(test)]
mod tests {
  use alloc::vec::Vec;

  use super::{BITS_PER_WORD, BitSet};

  /// 跨字推进：全空字要跳过且不影响后续桶号，多位置位按字内升序输出。
  #[test]
  fn bits_spans_words_and_skips_empty_words() {
    let occupied = BitSet {
      words: alloc::vec![1 << (BITS_PER_WORD - 1), 0, 0b101, 0],
    };
    let buckets = occupied.bits().collect::<Vec<_>>();
    assert_eq!(
      buckets,
      [BITS_PER_WORD - 1, 2 * BITS_PER_WORD, 2 * BITS_PER_WORD + 2,]
    );
  }

  /// 空位图（含空字数组）不产出任何桶号，且可安全重复耗尽。
  #[test]
  fn bits_of_empty_set_yields_nothing() {
    let empty = BitSet { words: Vec::new() };
    let mut it = empty.bits();
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
  }
}
