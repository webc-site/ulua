//! `AstName AstNameTable::get_or_add(const char* name)` — Ast/src/Lexer.cpp:259.

use core::ffi::{CStr, c_char};

use crate::records::{ast_name::AstName, ast_name_table::AstNameTable};

impl AstNameTable {
  /// Looks up or adds a name from a null-terminated C string.
  ///
  /// # Safety
  /// `name` must point to a valid null-terminated C string.
  pub unsafe fn get_or_add_c_str(&mut self, name: *const c_char) -> AstName {
    let len = unsafe { CStr::from_ptr(name).to_bytes().len() };
    unsafe { self.get_or_add_with_type(name, len).0 }
  }
}
