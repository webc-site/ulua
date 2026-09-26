/// 计算以 2 的幂为模的哈希桶索引：`s & (size - 1)`。
///
/// 要求 `size` 必须为 2 的幂且大于 0。
///
/// # Panics
/// 在 debug 模式下，若 `size <= 0` 或不是 2 的幂则断言失败。
#[inline(always)]
pub const fn lmod(s: i32, size: i32) -> i32 {
  debug_assert!(
    size > 0 && (size & (size - 1)) == 0,
    "size must be a positive power of two"
  );
  s & (size - 1)
}

/// 计算以 2 的幂为模的哈希桶索引（向后兼容宏）。
#[macro_export]
macro_rules! lmod {
  ($s:expr, $size:expr) => {
    $crate::macros::lmod::lmod(($s) as i32, ($size) as i32)
  };
}

pub use crate::lmod;
