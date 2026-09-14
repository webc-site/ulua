#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name {
  pub name: AstName,
  pub location: Location,
}
use crate::records::{ast_name::AstName, location::Location};
