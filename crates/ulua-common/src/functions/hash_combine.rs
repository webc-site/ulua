/// cpp `hashCombine` 的黄金比分割常数 `0x9e3779b9 = 2^32 / φ`
/// （`HashUtil.h:39-44`，注释指向 softwareengineering.stackexchange.com/a/402543）。
const GOLDEN_RATIO_32: usize = 0x9e37_79b9;

/// 混合单个哈希值到种子，cpp `hashCombine(size_t& seed, size_t hash)`。
///
/// C++ 用引用出参累积，Rust 侧改为返回新种子，链式/折叠调用即可。
#[inline]
pub fn hash_combine(seed: usize, hash: usize) -> usize {
  // 黄金分割常数，散列分布更好
  seed
    ^ hash
      .wrapping_add(GOLDEN_RATIO_32)
      .wrapping_add(seed << 6)
      .wrapping_add(seed >> 2)
}
