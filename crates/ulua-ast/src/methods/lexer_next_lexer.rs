//! `const Lexeme& Lexer::next()` — Ast/src/Lexer.cpp:369.
//! `const Lexeme& Lexer::next(bool skipComments, bool updatePrevLocation)`
//! — Ast/src/Lexer.cpp:374.

use crate::{
  enums::type_lexer::Type,
  functions::char_classifier::is_space,
  records::{lexeme::Lexeme, lexer::Lexer},
};

impl Lexer {
  #[inline]
  pub fn next_lexeme(&mut self) -> &Lexeme {
    let skip_comments = self.skip_comments;
    self.next_with(skip_comments, true)
  }
}

impl Lexer {
  pub fn next_with(&mut self, skip_comments: bool, mut update_prev_location: bool) -> &Lexeme {
    // in skipComments mode we reject valid comments
    loop {
      // consume whitespace before the token
      while is_space(self.peekch()) {
        self.consume_any();
      }

      if update_prev_location {
        self.prev_location = self.lexeme.location;
      }

      self.lexeme = self.read_next();
      update_prev_location = false;

      if !(skip_comments
        && (self.lexeme.r#type == Type::COMMENT || self.lexeme.r#type == Type::BLOCK_COMMENT))
      {
        break;
      }
    }

    &self.lexeme
  }
}
