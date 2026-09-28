use crate::{enums::type_lexer::Type, records::position::Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatchLexeme {
  pub type_: Type,
  pub position: Position,
}
