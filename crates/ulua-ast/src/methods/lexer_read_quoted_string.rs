//! `Lexeme Lexer::read_quoted_string()` — Ast/src/Lexer.cpp:583.

use ulua_common::LUAU_ASSERT;

use crate::records::{
  lexeme::{Lexeme, Type},
  lexer::Lexer,
  location::Location,
};

impl Lexer {
  pub(crate) fn read_quoted_string(&mut self) -> Lexeme {
    let start = self.position();

    let delimiter = self.peekch();
    LUAU_ASSERT!(delimiter == '\'' || delimiter == '"');
    self.consume();

    let start_offset = self.offset;

    while self.peekch() != delimiter {
      match self.peekch() {
        '\0' | '\r' | '\n' => {
          return Lexeme::new(Location::new(start, self.position()), Type::BROKEN_STRING);
        }
        '\\' => self.read_backslash_in_string(),
        _ => self.consume(),
      }
    }

    self.consume();

    Lexeme::with_data(
      Location::new(start, self.position()),
      Type::QUOTED_STRING,
      unsafe { self.buffer.add(start_offset as usize) },
      (self.offset - start_offset - 1) as usize,
    )
  }
}
