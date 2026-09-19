use crate::{
  enums::type_lexer::Type,
  records::{lexeme::Lexeme, match_lexeme::MatchLexeme, position::Position},
};

impl MatchLexeme {
  pub fn new(l: &Lexeme) -> Self {
    Self {
      type_: l.r#type,
      position: l.location.begin,
    }
  }

  pub fn missing() -> Self {
    Self {
      type_: Type::EOF,
      position: Position::missing(),
    }
  }
}
