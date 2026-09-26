use alloc::string::String;
use core::option::Option;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeInstantiationCountMismatch {
  pub(crate) function_name: Option<String>,
  pub(crate) function_type: TypeId,
  pub(crate) provided_types: usize,
  pub(crate) maximum_types: usize,
  pub(crate) provided_type_packs: usize,
  pub(crate) maximum_type_packs: usize,
}

impl TypeInstantiationCountMismatch {
  pub fn function_name(&self) -> Option<&str> {
    self.function_name.as_deref()
  }

  pub fn function_type(&self) -> TypeId {
    self.function_type
  }

  pub fn provided_types(&self) -> usize {
    self.provided_types
  }

  pub fn maximum_types(&self) -> usize {
    self.maximum_types
  }

  pub fn provided_type_packs(&self) -> usize {
    self.provided_type_packs
  }

  pub fn maximum_type_packs(&self) -> usize {
    self.maximum_type_packs
  }
}
