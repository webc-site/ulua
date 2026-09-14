use core::ffi::CStr;

use crate::records::ast_name::AstName;

impl AstName {
  #[inline]
  pub fn operator_ne_c_char(&self, rhs: &CStr) -> bool {
    !self.operator_eq_c_char(rhs)
  }
}
