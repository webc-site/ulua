use core::ops::{BitAnd, BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BcBlockFlag {
  Dead = 1 << 0,
}

impl BcBlockFlag {
  #[inline]
  pub const fn mask(self) -> u8 {
    self as u8
  }

  #[inline]
  pub const fn is_set(self, flags: u8) -> bool {
    (flags & (self as u8)) != 0
  }

  #[inline]
  pub fn set(self, flags: &mut u8) {
    *flags |= self as u8;
  }
}

impl BitAnd<BcBlockFlag> for u8 {
  type Output = u8;
  #[inline]
  fn bitand(self, rhs: BcBlockFlag) -> u8 {
    self & (rhs as u8)
  }
}

impl BitOr<BcBlockFlag> for u8 {
  type Output = u8;
  #[inline]
  fn bitor(self, rhs: BcBlockFlag) -> u8 {
    self | (rhs as u8)
  }
}

impl BitOrAssign<BcBlockFlag> for u8 {
  #[inline]
  fn bitor_assign(&mut self, rhs: BcBlockFlag) {
    *self |= rhs as u8;
  }
}
