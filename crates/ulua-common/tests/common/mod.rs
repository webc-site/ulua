//! `tests/dense_hash.rs` / `tests/small_vector.rs` 共用的微型 xorshift 随机源：
//! 无依赖、给定种子即确定可复现，此前在两个测试文件逐字双份，此处单点收口。
//! 仅由这两个测试文件以 `mod common;` 引入，遵守「导出面按消费收敛、不设
//! `#[allow(dead_code)]`」纪律——两个消费方都会用到全部导出项。

/// 微型 xorshift：让模糊测试在无依赖前提下确定可复现。
pub struct Rng(pub u64);

impl Rng {
  /// 产出下一个 64 位伪随机数。
  pub fn next(&mut self) -> u64 {
    let mut x = self.0;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    self.0 = x;
    x
  }

  /// 产出 `[0, n)` 内的伪随机数。
  pub fn below(&mut self, n: u64) -> u64 {
    self.next() % n
  }
}
