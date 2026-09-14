//! `AstName AstNameTable::get_or_add(const char* name, size_t len)` — Ast/src/Lexer.cpp:254.

use core::ffi::c_char;

use crate::records::{ast_name::AstName, ast_name_table::AstNameTable};

impl AstNameTable {
  /// Looks up or adds a name with the given length into the table.
  ///
  /// # Safety
  /// `name` must point to at least `len` initialized bytes.
  pub unsafe fn get_or_add(&mut self, name: *const c_char, len: usize) -> AstName {
    unsafe { self.get_or_add_with_type(name, len).0 }
  }

  /// Looks up or adds a name from a byte slice into the table.
  #[inline]
  pub fn get_or_add_slice(&mut self, name: &[u8]) -> AstName {
    unsafe { self.get_or_add(name.as_ptr() as *const c_char, name.len()) }
  }

  /// Looks up or adds a name from a string slice into the table.
  #[inline]
  pub fn get_or_add_str(&mut self, name: &str) -> AstName {
    self.get_or_add_slice(name.as_bytes())
  }
}
