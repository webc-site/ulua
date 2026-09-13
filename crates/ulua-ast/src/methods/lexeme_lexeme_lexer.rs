//! `Lexeme::Lexeme(const Location& location, Type type)` — Ast/src/Lexer.cpp:14.

use core::ptr::null;

use crate::records::{
  lexeme::{Lexeme, LexemeData, Type},
  location::Location,
};

impl Lexeme {
  /// A token carrying no payload (`length = 0`, `data = nullptr`).
  pub fn new(location: Location, r#type: Type) -> Lexeme {
    Lexeme {
      r#type,
      location,
      length: 0,
      data: LexemeData { data: null() },
    }
  }
}
