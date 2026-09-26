//! 警告位掩码表。四个微型位操作方法（体量过小）按 vec_deque 先例并回本文件，
//! 不再拆 one-item-per-file。

use crate::enums::code::Code;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LintOptions {
  pub warning_mask: u64,
}

impl LintOptions {
  /// 默认启用全部警告。
  pub fn set_defaults(&mut self) {
    self.warning_mask = !0u64;
  }

  #[inline]
  pub fn enable_warning(&mut self, code: Code) {
    self.warning_mask |= code.mask_bit();
  }

  #[inline]
  pub fn disable_warning(&mut self, code: Code) {
    self.warning_mask &= !code.mask_bit();
  }

  #[inline]
  pub fn is_enabled(&self, code: Code) -> bool {
    self.warning_mask & code.mask_bit() != 0
  }
}
