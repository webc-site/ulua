use core::ops::BitOr;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FunctionStatsFlags {
  FunctionStatsEnable = 1 << 0,
  FunctionStatsBytecodeSummary = 1 << 1,
}

impl BitOr for FunctionStatsFlags {
  type Output = i32;

  fn bitor(self, rhs: Self) -> Self::Output {
    (self as i32) | (rhs as i32)
  }
}

impl BitOr<FunctionStatsFlags> for i32 {
  type Output = i32;

  fn bitor(self, rhs: FunctionStatsFlags) -> Self::Output {
    self | (rhs as i32)
  }
}
