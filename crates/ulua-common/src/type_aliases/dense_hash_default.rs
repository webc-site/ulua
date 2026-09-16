//! Port of `Luau::detail::DenseHashDefault` — the default hash functor a
//! `DenseHashMap`/`DenseHashSet` uses when none is supplied.
//!
//! Reference: `luau/Common/include/Luau/HashUtil.h:27`
//! `using DenseHashDefault = std::conditional_t<is_pointer_v<T>, DenseHashPointer, std::hash<T>>;`
//!
//! Deviation (documented, behaviorally faithful): Rust has no stable
//! specialization, so a single `K: Hash` blanket impl serves every key type,
//! including raw pointers, via the inline FNV-1a hasher below. C++'s pointer
//! specialization (`DenseHashPointer`，乘法混洗版，见
//! `records::dense_hash_pointer`) and `std::hash`
//! differ only in the constant scrambling — any consistent hash is correct
//! here because `DenseHash` iteration order is not a contract. The exact
//! `DenseHashPointer` remains available for explicit use.

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
