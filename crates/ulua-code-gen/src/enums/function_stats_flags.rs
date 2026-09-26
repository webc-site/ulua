//! Source: `CodeGen/include/Luau/LoweringStats.h:37`

use core::ops::BitOr;

crate::flag_enum! {
  pub enum FunctionStatsFlags: u32 {
    FunctionStatsEnable = 1 << 0,
    FunctionStatsBytecodeSummary = 1 << 1,
  }
}

// cpp 侧存在 `flags | FunctionStats_*` 落在带符号 int 上的用法，保留该有符号变体。
impl BitOr<FunctionStatsFlags> for i32 {
  type Output = i32;

  #[inline]
  fn bitor(self, rhs: FunctionStatsFlags) -> i32 {
    self | (rhs as i32)
  }
}
