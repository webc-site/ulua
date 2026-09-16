use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct Error {
  pub index: i32,
  pub synthetic: Option<TypeId>,
}

impl Error {
  pub fn error_error() -> Self {
    Self {
      index: 0,
      synthetic: None,
    }
  }
}
