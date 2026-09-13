//! `AstName AstNameTable::get(const char* name) const` — Ast/src/Lexer.cpp:264.

use core::ffi::{CStr, c_char};

use crate::records::{ast_name::AstName, ast_name_table::AstNameTable};

impl AstNameTable {
  /// Looks up a name in the table.
  ///
  /// # Safety
  /// `name` must point to a valid null-terminated C string.
  pub unsafe fn get(&self, name: *const c_char) -> AstName {
    let len = unsafe { CStr::from_ptr(name).to_bytes().len() };
    self.get_with_type(name, len).0
  }

  /// Looks up a name by byte slice.
  #[inline]
  pub fn get_slice(&self, name: &[u8]) -> AstName {
    self
      .get_with_type(name.as_ptr() as *const c_char, name.len())
      .0
  }

  /// Looks up a name by string slice.
  #[inline]
  pub fn get_str(&self, name: &str) -> AstName {
    self.get_slice(name.as_bytes())
  }
}
