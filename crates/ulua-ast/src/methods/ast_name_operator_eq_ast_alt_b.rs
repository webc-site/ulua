use core::ffi::{CStr, c_char};

use crate::records::ast_name::AstName;

impl AstName {
  #[inline]
  pub fn operator_eq_c_char(&self, rhs: &CStr) -> bool {
    if self.value.is_null() {
      return false;
    }
    self.as_bytes() == rhs.to_bytes()
  }

  /// # Safety
  /// `rhs` must be a valid null-terminated C string pointer.
  #[inline]
  /// # Safety
  /// `rhs` 必须指向合法的 C 字符串或为 null。
  pub unsafe fn operator_eq_raw(&self, rhs: *const c_char) -> bool {
    if self.value.is_null() || rhs.is_null() {
      return false;
    }
    unsafe { CStr::from_ptr(self.value) == CStr::from_ptr(rhs) }
  }
}
