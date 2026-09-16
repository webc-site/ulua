use core::mem::transmute;

use crate::enums::kind::Kind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Patch {
  /// Bit-field: kind (2 bits), label (30 bits).
  /// In Rust, we represent this as a single u32 and provide accessors in the impl.
  pub(crate) kind_and_label: u32,
  pub(crate) location: u32,
}

impl Patch {
  #[inline]
  pub(crate) fn kind(&self) -> Kind {
    unsafe { transmute((self.kind_and_label & 0x3) as i32) }
  }

  #[inline]
  pub(crate) fn label(&self) -> u32 {
    self.kind_and_label >> 2
  }

  #[inline]
  pub(crate) fn set_kind(&mut self, kind: Kind) {
    self.kind_and_label = (self.kind_and_label & !0x3) | ((kind as u32) & 0x3);
  }

  #[inline]
  pub(crate) fn set_label(&mut self, label: u32) {
    self.kind_and_label = (self.kind_and_label & 0x3) | (label << 2);
  }
}
