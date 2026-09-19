use crate::{enums::type_lexer::Type, records::location::Location};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comment {
  pub r#type: Type,
  pub location: Location,
}
