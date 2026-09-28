//! `Lexeme Lexer::read_long_string(const Position& start, int sep, Lexeme::Type ok, Lexeme::Type broken)`
//! — Ast/src/Lexer.cpp:525.

use ulua_common::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  records::{lexeme::Lexeme, lexer::Lexer, location::Location, position::Position},
};

impl Lexer {
  pub(crate) fn read_long_string(
    &mut self,
    start: &Position,
    sep: i32,
    ok: Type,
    broken: Type,
  ) -> Lexeme {
    // skip (second) [
    LUAU_ASSERT!(self.peekch() == '[');
    self.consume();

    let start_offset = self.offset;

    while self.peekch() != '\0' {
      if self.peekch() == ']' {
        if self.skip_long_separator() == sep {
          LUAU_ASSERT!(self.peekch() == ']');
          self.consume(); // skip (second) ]

          let end_offset = self.offset - sep as u32 - 2;
          LUAU_ASSERT!(end_offset >= start_offset);

          return Lexeme::with_data(
            Location::new(*start, self.position()),
            ok,
            // ⇔ cpp `Lexeme(..., &buffer[startOffset], endOffset - startOffset)`
            // （Lexer.cpp:545）：前置 LUAU_ASSERT! 校验 end_offset ≥ start_offset，
            // 载荷与源缓冲同寿命。
            &self.buffer[start_offset as usize..end_offset as usize],
          );
        }
      } else {
        self.consume_any();
      }
    }

    Lexeme::new(Location::new(*start, self.position()), broken)
  }
}
