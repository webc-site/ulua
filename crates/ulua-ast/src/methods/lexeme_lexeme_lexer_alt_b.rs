//! `Lexeme::Lexeme(const Location& location, char character)` — Ast/src/Lexer.cpp:22.

use core::ptr::null;

use crate::records::{
  lexeme::{Lexeme, LexemeData, Type},
  location::Location,
};

impl Lexeme {
  /// A single-character token: `type = static_cast<Type>(static_cast<unsigned
  /// char>(character))`, so the token type is the raw byte value (`< 256`).
  pub fn from_char(location: Location, character: char) -> Lexeme {
    Lexeme {
      r#type: Type((character as u8) as i32),
      location,
      length: 0,
      data: LexemeData { data: null() },
    }
  }
}
