use crate::records::duplicate_type_definition::DuplicateTypeDefinition;

impl DuplicateTypeDefinition {
  #[inline]
  pub fn operator_eq(&self, rhs: &DuplicateTypeDefinition) -> bool {
    self.name == rhs.name && self.previous_location == rhs.previous_location
  }
}
