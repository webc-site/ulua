use alloc::string::String;

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownProperty {
  pub(crate) table: TypeId,
  pub(crate) key: String,
}

impl UnknownProperty {
  pub fn table(&self) -> TypeId {
    self.table
  }

  pub fn key(&self) -> &str {
    &self.key
  }
}
