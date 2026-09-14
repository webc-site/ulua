use alloc::string::String;

use crate::records::{
  cannot_extend_table::CannotExtendTable, duplicate_type_definition::DuplicateTypeDefinition,
  unknown_property::UnknownProperty,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidNameChecker {
  invalid_name: String,
}

impl InvalidNameChecker {
  pub fn new() -> Self {
    Self {
      invalid_name: "%error-id%".to_owned(),
    }
  }
}

impl Default for InvalidNameChecker {
  fn default() -> Self {
    Self::new()
  }
}

impl InvalidNameChecker {
  pub fn operator_unknown_property(&self, e: &UnknownProperty) -> bool {
    e.key() == self.invalid_name
  }

  pub fn operator_cannot_extend_table(&self, e: &CannotExtendTable) -> bool {
    e.prop() == self.invalid_name
  }

  pub fn operator_duplicate_type_definition(&self, e: &DuplicateTypeDefinition) -> bool {
    e.name() == self.invalid_name
  }

  pub fn operator_fallback<T>(&self, _other: &T) -> bool {
    false
  }
}

unsafe impl Send for InvalidNameChecker {}
unsafe impl Sync for InvalidNameChecker {}
