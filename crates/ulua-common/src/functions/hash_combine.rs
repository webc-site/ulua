/// 混合单个哈希值到种子，cpp `hashCombine(size_t& seed, size_t hash)`。
///
/// C++ 用引用出参累积，Rust 侧改为返回新种子，链式/折叠调用即可。
#[inline]
pub fn hash_combine(seed: usize, hash: usize) -> usize {
  // 黄金分割常数，散列分布更好
  // 参见 https://softwareengineering.stackexchange.com/a/402543
  seed
    ^ hash
      .wrapping_add(0x9e3779b9)
      .wrapping_add(seed << 6)
      .wrapping_add(seed >> 2)
}
