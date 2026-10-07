//! cpp `Luau::hashRange`（Common/src/StringUtils.cpp:220）：FNV-1a 32 位。
//!
//! 只保留切片形态（零拷贝、无 unsafe）：cpp 的 `(const char*, size_t)` 参数
//! 形态在 Rust 侧即 `&[u8]`，由调用方自行取切片，无需裸指针边界。
//!
//! review.md §5 的 `crc32fast` 在此**不引入**（有意例外，故写明理由）：本函数是
//! `DenseHashTable`/`AstNameTable` 的**桶位来源**，§0 要求桶分布与 cpp oracle 一致
//! ——`hash_range_bytes` 的每一位都必须是 FNV-1a 的输出，换成 CRC32 折叠值会让所有
//! 既有表的探测序列、`nextPowerOf2` 增长点的行为全部漂移（conformance 快照与
//! `DenseHash` 相关测试按此取值）。另外全仓没有一处真正的校验和/完整性用途
//! （`lua_pcall` 侧的 `PCRC` 是「本次调用落在 C 函数上」的调用形态标志，见 cpp
//! `VM/src/ldo.h:52`，与 CRC 多项式无关），所以 `crc32fast` 在本仓没有适用位点。

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
