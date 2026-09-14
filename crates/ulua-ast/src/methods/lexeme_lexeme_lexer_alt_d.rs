//! `Lexeme::Lexeme(const Location&, Type, const char* name)` — Ast/src/Lexer.cpp:42.

use core::ffi::c_char;

use crate::records::{
  lexeme::{Lexeme, LexemeData, Type},
  location::Location,
};

impl Lexeme {
  /// A name/attribute/reserved-word token: the `name` union arm, `length = 0`.
  pub fn with_name(location: Location, r#type: Type, name: *const c_char) -> Lexeme {
    ulua_common::LUAU_ASSERT!(
      r#type == Type::NAME
        || r#type == Type::ATTRIBUTE
        || (r#type >= Type::RESERVED_BEGIN && r#type < Type::RESERVED_END_TOKEN)
    );

    Lexeme {
      r#type,
      location,
      length: 0,
      data: LexemeData { name },
    }
  }
}
