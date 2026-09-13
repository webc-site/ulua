use crate::records::{
  lexeme::{Lexeme, Type},
  match_lexeme::MatchLexeme,
  position::Position,
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

pub fn parser_match_lexeme_match_lexeme(l: &Lexeme) -> MatchLexeme {
  MatchLexeme::new(l)
}
