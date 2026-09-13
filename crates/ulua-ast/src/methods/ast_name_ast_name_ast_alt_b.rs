use core::ffi::c_char;

use crate::records::ast_name::AstName;

impl AstName {
  pub const fn ast_name_c_char(value: *const c_char) -> Self {
    Self { value }
  }
}
