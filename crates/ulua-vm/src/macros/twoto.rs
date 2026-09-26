/// 计算 2 的 `x` 次幂（等价于 `1i32 << x`）。
///
/// # Panics
/// 在 debug 模式下，若 `x >= 31` 则断言失败（避免超出 `i32` 正数表达范围）。
#[inline(always)]
pub const fn twoto(x: u8) -> i32 {
  debug_assert!(x < 31, "exponent must be less than 31 for positive i32");
  1i32 << x
}

/// 计算 2 的 `x` 次幂的宏包装（向后兼容）。
#[macro_export]
macro_rules! twoto {
  ($x:expr) => {
    $crate::macros::twoto::twoto(($x) as u8)
  };
}

pub use crate::twoto;
