//! `const Lexeme& Lexer::next()` — Ast/src/Lexer.cpp:369.

use crate::records::{lexeme::Lexeme, lexer::Lexer};

impl Lexer {
  #[inline]
  pub fn next_lexeme(&mut self) -> &Lexeme {
    let skip_comments = self.skip_comments;
    self.next_with(skip_comments, true)
  }
}
