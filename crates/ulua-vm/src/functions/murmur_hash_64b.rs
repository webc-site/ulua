//! MurmurHash64B finalizer（cpp ltable.cpp 中 `hashint`/`hashnum` 共用的收尾混合）：
//! 两函数的 8 步 xor/mul 链逐字相同，收敛为单点保证同步演进。

/// cpp `ltable.cpp` `hashint`/`hashnum` 共同的 MurmurHash64B finalizer：
/// 以 64 位键拆出的低/高两个 `u32`（`h1`/`h2`，本机字节序拆位）做定序混合，
/// 返回截断到 32 位的 `h2`（cpp 注释：normally hash is equal to
/// `(uint64_t(h1) << 32) | h2`，但 VM 只需要低 32 位半部）。
///
/// 混合步序与常量不可改动（决定表的节点分布，与 cpp 逐位一致）。
#[inline]
pub(crate) const fn murmur_hash_64b(mut h1: u32, mut h2: u32) -> u32 {
  // finalizer from MurmurHash64B
  const M: u32 = 0x5bd1e995;

  h1 ^= h2 >> 18;
  h1 = h1.wrapping_mul(M);
  h2 ^= h1 >> 22;
  h2 = h2.wrapping_mul(M);
  h1 ^= h2 >> 17;
  h1 = h1.wrapping_mul(M);
  h2 ^= h1 >> 19;
  h2 = h2.wrapping_mul(M);
  h2
}
