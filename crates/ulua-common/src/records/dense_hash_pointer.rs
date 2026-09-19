use core::ffi::c_void;

/// cpp 常量 `0xbf58476d1ce4e5b9`（HashUtil.h:20）。
const MULTIPLIER: u64 = 0xbf58_476d_1ce4_e5b9;

/// cpp `Luau::DenseHashPointer`（Common/include/Luau/HashUtil.h:14-26）：乘法
/// 混洗后右移异或，为 arena 分配的指针（高位高度雷同）提供更好的散射。
#[derive(Debug, Clone, Copy, Default)]
pub struct DenseHashPointer;

impl DenseHashPointer {
  #[inline]
  pub fn hash(&self, key: *const c_void) -> usize {
    let u = (key as usize as u64).wrapping_mul(MULTIPLIER);
    (u ^ (u >> 31)) as usize
  }

  /// cpp `operator()` 的调用形态别名（HashUtil.h:24）。
  #[inline]
  pub fn call(&self, key: *const c_void) -> usize {
    self.hash(key)
  }
}
