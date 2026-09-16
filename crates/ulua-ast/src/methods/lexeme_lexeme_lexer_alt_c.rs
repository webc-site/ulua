//! `Lexeme::Lexeme(const Location&, Type, const char* data, size_t size)` — Ast/src/Lexer.cpp:30.

use core::ffi::c_char;

use crate::records::{
  lexeme::{Lexeme, LexemeData, Type},
  location::Location,
};

impl Lexeme {
  /// A token with a `data`/`length` payload (string, number, comment, ...).
  pub fn with_data(location: Location, r#type: Type, data: *const c_char, size: usize) -> Lexeme {
    ulua_common::LUAU_ASSERT!(
      r#type == Type::RAW_STRING
        || r#type == Type::QUOTED_STRING
        || r#type == Type::INTERP_STRING_BEGIN
        || r#type == Type::INTERP_STRING_MID
        || r#type == Type::INTERP_STRING_END
        || r#type == Type::INTERP_STRING_SIMPLE
        || r#type == Type::BROKEN_INTERP_DOUBLE_BRACE
        || r#type == Type::NUMBER
        || r#type == Type::COMMENT
        || r#type == Type::BLOCK_COMMENT
    );

    Lexeme {
      r#type,
      location,
      length: size as u32,
      data: LexemeData { data },
    }
  }
}
