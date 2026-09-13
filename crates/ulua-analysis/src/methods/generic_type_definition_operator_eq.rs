use crate::records::generic_type_definition::GenericTypeDefinition;

impl GenericTypeDefinition {
  #[inline]
  pub fn operator_eq(&self, rhs: &GenericTypeDefinition) -> bool {
    self.ty == rhs.ty && self.default_value == rhs.default_value
  }
}
