//! `Lexeme Lexer::read_utf_8_error()` — Ast/src/Lexer.cpp:1048.

use crate::records::{
  lexeme::{Lexeme, LexemeData, Type},
  lexer::Lexer,
  location::Location,
};

impl Lexer {
  pub(crate) fn read_utf_8_error(&mut self) -> Lexeme {
    let start = self.position();
    let ch = self.peekch() as u8;

    let (mut codepoint, size) = if (ch & 0b1000_0000) == 0 {
      ((ch & 0x7F) as u32, 1)
    } else if (ch & 0b1110_0000) == 0b1100_0000 {
      ((ch & 0b1_1111) as u32, 2)
    } else if (ch & 0b1111_0000) == 0b1110_0000 {
      ((ch & 0b1111) as u32, 3)
    } else if (ch & 0b1111_1000) == 0b1111_0000 {
      ((ch & 0b111) as u32, 4)
    } else {
      self.consume();
      return Lexeme::new(Location::new(start, self.position()), Type::BROKEN_UNICODE);
    };

    self.consume();

    let mut i = 1;
    while i < size {
      let next_ch = self.peekch() as u8;
      if (next_ch & 0b1100_0000) != 0b1000_0000 {
        return Lexeme::new(Location::new(start, self.position()), Type::BROKEN_UNICODE);
      }

      codepoint = (codepoint << 6) | ((next_ch & 0b0011_1111) as u32);
      self.consume();
      i += 1;
    }

    let mut result = Lexeme::new(Location::new(start, self.position()), Type::BROKEN_UNICODE);
    result.data = LexemeData { codepoint };
    result
  }
}
