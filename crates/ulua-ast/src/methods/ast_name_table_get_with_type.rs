//! `std::pair<AstName, Lexeme::Type> AstNameTable::get_with_type(const char* name, size_t length) const`
//! — Ast/src/Lexer.cpp:245.

use core::ffi::c_char;

use crate::records::{ast_name::AstName, ast_name_table::AstNameTable, entry::Entry, lexeme::Type};

impl AstNameTable {
  pub fn get_with_type(&self, name: *const c_char, length: usize) -> (AstName, Type) {
    let key = Entry {
      value: AstName { value: name },
      length: length as u32,
      r#type: Type::EOF,
    };

    if let Some(entry) = self.data.find(&key) {
      (entry.value, entry.r#type)
    } else {
      (AstName::new(), Type::NAME)
    }
  }
}
