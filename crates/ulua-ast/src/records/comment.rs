use crate::records::{lexeme::Type, location::Location};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comment {
  pub r#type: Type,
  pub location: Location,
}
