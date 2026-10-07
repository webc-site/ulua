use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum Polarity {
  #[default]
  None = 0b000,
  Positive = 0b001,
  Negative = 0b010,
  Mixed = 0b011,
  Unknown = 0b100,
}

// C++ `Polarity.h`：None 为 `|` 恒等元，Mixed 吸收正负，Unknown 单独占位 0b100
impl BitOr for Polarity {
  type Output = Self;

  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    match (self, rhs) {
      (Self::None, r) | (r, Self::None) => r,
      (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
      (Self::Mixed, _) | (_, Self::Mixed) => Self::Mixed,
      (Self::Positive, Self::Positive) => Self::Positive,
      (Self::Negative, Self::Negative) => Self::Negative,
      _ => Self::Mixed,
    }
  }
}

impl BitOrAssign for Polarity {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitAnd for Polarity {
  type Output = Self;

  #[inline]
  fn bitand(self, rhs: Self) -> Self {
    match (self, rhs) {
      (Self::None, _) | (_, Self::None) => Self::None,
      (Self::Unknown, Self::Unknown) => Self::Unknown,
      (Self::Unknown, _) | (_, Self::Unknown) => Self::None,
      (Self::Mixed, r) | (r, Self::Mixed) => r,
      (Self::Positive, Self::Positive) => Self::Positive,
      (Self::Negative, Self::Negative) => Self::Negative,
      _ => Self::None,
    }
  }
}

impl BitAndAssign for Polarity {
  #[inline]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}
