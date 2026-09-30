use core::ops::{BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AstTableAccess {
  Read = 0b01,
  Write = 0b10,
  ReadWrite = 0b11,
}

impl BitOr for AstTableAccess {
  type Output = Self;

  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    match (self as u8) | (rhs as u8) {
      0b01 => Self::Read,
      0b10 => Self::Write,
      _ => Self::ReadWrite,
    }
  }
}

impl BitOrAssign for AstTableAccess {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}
