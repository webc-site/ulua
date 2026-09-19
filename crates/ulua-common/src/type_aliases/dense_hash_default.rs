//! Port of `Luau::detail::DenseHashDefault` — the default hash functor a
//! `DenseHashMap`/`DenseHashSet` uses when none is supplied.
//!
//! Reference: `luau/Common/include/Luau/HashUtil.h:27`
//! `using DenseHashDefault = std::conditional_t<is_pointer_v<T>, DenseHashPointer, std::hash<T>>;`
//!
//! Deviation (documented, behaviorally faithful): Rust has no stable
//! specialization, so a single `K: Hash` blanket impl serves every key type —
//! raw pointers included — via the inline FNV-1a hasher below. cpp 的指针特化
//! `DenseHashPointer`（HashUtil.h:14-26，乘法混洗 + 右移异或）因此不移植：
//! 全仓零使用者，需要时按上游那 12 行补回即可。两者只差常量混洗强度：
//! `DenseHash` 的迭代序从来不是契约，任何自洽哈希都正确，且槽位下标由
//! `DenseHashTable::do_hash` 的 fibonacci 散射取高位，不依赖哈希函数的低位质量。

use core::{
  hash::{Hash, Hasher},
  marker::PhantomData,
};

use crate::records::dense_hash_table::DenseHasher;

/// FNV-1a 64 位偏移基数。
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
/// FNV-1a 64 位质数。
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Deterministic, dependency-free FNV-1a 64-bit hasher (portable to
/// `wasm32-unknown-unknown`; `core` exposes no concrete `Hasher`).
struct FnvHasher(u64);

impl Hasher for FnvHasher {
  fn finish(&self) -> u64 {
    self.0
  }

  fn write(&mut self, bytes: &[u8]) {
    let mut hash = self.0;
    for &byte in bytes {
      hash ^= u64::from(byte);
      hash = hash.wrapping_mul(FNV_PRIME);
    }
    self.0 = hash;
  }
}

/// Default key hasher. Generic over the key type so it mirrors the C++ alias
/// `DenseHashDefault<T>`.
#[derive(Clone, Copy)]
pub struct DenseHashDefault<K>(PhantomData<K>);

// Manual impl: the derive would demand `K: Default`, but PhantomData needs
// nothing (keys like raw pointers and `String` must work too).
impl<K> Default for DenseHashDefault<K> {
  fn default() -> Self {
    DenseHashDefault(PhantomData)
  }
}

impl<K: Hash> DenseHasher<K> for DenseHashDefault<K> {
  fn hash(&self, key: &K) -> usize {
    let mut hasher = FnvHasher(FNV_OFFSET_BASIS);
    key.hash(&mut hasher);
    hasher.finish() as usize
  }
}
