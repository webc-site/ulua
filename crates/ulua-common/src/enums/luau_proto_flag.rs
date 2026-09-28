//! Source: `Common/include/Luau/Bytecode.h`

use core::ops::{BitAnd, BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LuauProtoFlag {
  /// used to tag main Proto for modules with --!native
  LpfNativeModule = 1 << 0,
  /// used to tag individual protos as not profitable to compile natively
  LpfNativeCold = 1 << 1,
  /// used to tag main Proto for modules that have at least one function with native attribute
  LpfNativeFunction = 1 << 2,
  /// function can be inlined
  LpfInlinable = 1 << 3,
  /// top-level function uses export statements and returns the export table
  LpfUsesExport = 1 << 4,
}

impl LuauProtoFlag {
  pub const LPF_NATIVE_MODULE: Self = Self::LpfNativeModule;
  pub const LPF_NATIVE_COLD: Self = Self::LpfNativeCold;
  pub const LPF_NATIVE_FUNCTION: Self = Self::LpfNativeFunction;
  pub const LPF_INLINABLE: Self = Self::LpfInlinable;
  pub const LPF_USES_EXPORT: Self = Self::LpfUsesExport;

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

impl BitAnd<LuauProtoFlag> for u8 {
  type Output = u8;
  #[inline]
  fn bitand(self, rhs: LuauProtoFlag) -> u8 {
    self & (rhs as u8)
  }
}

impl BitOr<LuauProtoFlag> for u8 {
  type Output = u8;
  #[inline]
  fn bitor(self, rhs: LuauProtoFlag) -> u8 {
    self | (rhs as u8)
  }
}

impl BitOrAssign<LuauProtoFlag> for u8 {
  #[inline]
  fn bitor_assign(&mut self, rhs: LuauProtoFlag) {
    *self |= rhs as u8;
  }
}
