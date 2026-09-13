//! `Lexeme Lexer::read_comment_body()` — Ast/src/Lexer.cpp:475.

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::is_newline::is_newline,
  records::{
    lexeme::{Lexeme, Type},
    lexer::Lexer,
    location::Location,
  },
};

impl Lexer {
  pub(crate) fn read_comment_body(&mut self) -> Lexeme {
    let start = self.position();

    LUAU_ASSERT!(self.peekch_ahead(0) == '-' && self.peekch_ahead(1) == '-');
    self.consume();
    self.consume();

    let start_offset = self.offset;

    if self.peekch() == '[' {
      let sep = self.skip_long_separator();

      if sep >= 0 {
        return self.read_long_string(&start, sep, Type::BLOCK_COMMENT, Type::BROKEN_COMMENT);
      }
    }

    // fall back to single-line comment
    while self.peekch() != '\0' && self.peekch() != '\r' && !is_newline(self.peekch()) {
      self.consume();
    }

    Lexeme::with_data(
      Location::new(start, self.position()),
      Type::COMMENT,
      unsafe { self.buffer.add(start_offset as usize) },
      (self.offset - start_offset) as usize,
    )
  }
}
