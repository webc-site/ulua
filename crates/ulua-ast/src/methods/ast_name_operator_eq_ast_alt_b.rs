use core::ffi::{CStr, c_char};

use crate::records::ast_name::AstName;

impl AstName {
  /// 与裸 `const char*` 比较，对应 cpp `AstName::operator==(const char* rhs)`：
  /// `value && rhs && strcmp(value, rhs) == 0`，任一侧为 null 即 false。
  /// 字面量场景请用 `AstName == &str`（`PartialEq<&str>` 同语义）。
  ///
  /// # Safety
  /// `rhs` 必须指向合法的 C 字符串或为 null。
  #[inline]
  pub unsafe fn operator_eq_raw(&self, rhs: *const c_char) -> bool {
    if self.value.is_null() || rhs.is_null() {
      return false;
    }
    unsafe { CStr::from_ptr(self.value) == CStr::from_ptr(rhs) }
  }
}
