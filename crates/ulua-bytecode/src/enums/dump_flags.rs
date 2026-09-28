use core::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub(crate) enum DumpFlags {
  Code = 1 << 0,
  Lines = 1 << 1,
  Source = 1 << 2,
  Locals = 1 << 3,
  Remarks = 1 << 4,
  Types = 1 << 5,
  Constants = 1 << 6,
}

impl DumpFlags {
  #[inline]
  pub const fn is_set(self, flags: u32) -> bool {
    (flags & (self as u32)) != 0
  }
}

impl BitOr for DumpFlags {
  type Output = u32;

  #[inline]
  fn bitor(self, rhs: Self) -> Self::Output {
    (self as u32) | (rhs as u32)
  }
}

impl BitAnd<DumpFlags> for u32 {
  type Output = u32;

  #[inline]
  fn bitand(self, rhs: DumpFlags) -> Self::Output {
    self & (rhs as u32)
  }
}
