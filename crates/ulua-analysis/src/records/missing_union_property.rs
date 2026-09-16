use alloc::{string::String, vec::Vec};

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MissingUnionProperty {
  pub(crate) r#type: TypeId,
  pub(crate) missing: Vec<TypeId>,
  pub(crate) key: String,
}

impl MissingUnionProperty {
  pub fn r#type(&self) -> TypeId {
    self.r#type
  }

  pub fn missing(&self) -> &[TypeId] {
    &self.missing
  }

  pub fn key(&self) -> &str {
    &self.key
  }
}
