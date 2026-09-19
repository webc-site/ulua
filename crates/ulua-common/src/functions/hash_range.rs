//! cpp `Luau::hashRange`（Common/src/StringUtils.cpp:220）：FNV-1a 32 位。
//!
//! 只保留切片形态（零拷贝、无 unsafe）：cpp 的 `(const char*, size_t)` 参数
//! 形态在 Rust 侧即 `&[u8]`，由调用方自行取切片，无需裸指针边界。

/// FNV-1a 32 位偏移基数。
const FNV_OFFSET_BASIS: u32 = 2166136261;
/// FNV-1a 32 位质数。
const FNV_PRIME: u32 = 16777619;

/// 对字节切片做 FNV-1a 32 位哈希，cpp `hashRange(data, size)` 的等价实现。
///
/// 短小热点（名字驻留、字符串表查询均调用），显式 `#[inline]` 以免跨 crate
/// 退化为实调用。
#[inline]
pub fn hash_range_bytes(b: &[u8]) -> usize {
  b.iter().fold(FNV_OFFSET_BASIS, |hash, &byte| {
    (hash ^ u32::from(byte)).wrapping_mul(FNV_PRIME)
  }) as usize
}
