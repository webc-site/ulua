use std::ops::{BitAnd, BitAndAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum UnifyResult {
  Ok,
  OccursCheckFailed,
  TooComplex,
}

// C++ `Unifier2.h`：`Ok` 为 `&` 恒等元，首个错误短路胜出
impl BitAnd for UnifyResult {
  type Output = Self;

  #[inline]
  fn bitand(self, rhs: Self) -> Self {
    if self == Self::Ok { rhs } else { self }
  }
}

impl BitAndAssign for UnifyResult {
  #[inline]
  fn bitand_assign(&mut self, rhs: Self) {
    if *self == Self::Ok {
      *self = rhs;
    }
  }
}
