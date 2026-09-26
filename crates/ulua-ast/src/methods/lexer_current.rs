use crate::records::{lexeme::Lexeme, lexer::Lexer};

impl Lexer {
  pub fn current(&self) -> &Lexeme {
    &self.lexeme
  }
}
