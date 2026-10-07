//! `const Location& Lexer::previous_location() const` — Ast/include/Luau/Lexer.h:171.

use crate::records::{lexer::Lexer, location::Location};

impl Lexer {
  pub fn previous_location(&self) -> &Location {
    &self.prev_location
  }
}
