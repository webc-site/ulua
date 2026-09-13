//! `Lexeme Lexer::read_interpolated_string_begin()` — Ast/src/Lexer.cpp:616.

use ulua_common::LUAU_ASSERT;

use crate::records::{
  lexeme::{Lexeme, Type},
  lexer::Lexer,
};

impl Lexer {
  pub(crate) fn read_interpolated_string_begin(&mut self) -> Lexeme {
    LUAU_ASSERT!(self.peekch() == '`');

    let start = self.position();
    self.consume();

    self.read_interpolated_string_section(
      start,
      Type::INTERP_STRING_BEGIN,
      Type::INTERP_STRING_SIMPLE,
    )
  }
}
