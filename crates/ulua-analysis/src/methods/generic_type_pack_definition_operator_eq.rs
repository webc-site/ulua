use crate::records::generic_type_pack_definition::GenericTypePackDefinition;

impl GenericTypePackDefinition {
  #[inline]
  pub fn operator_eq(&self, rhs: &GenericTypePackDefinition) -> bool {
    self.tp == rhs.tp && self.default_value == rhs.default_value
  }
}
