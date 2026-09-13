use crate::records::type_instantiation_count_mismatch::TypeInstantiationCountMismatch;

impl TypeInstantiationCountMismatch {
  #[inline]
  pub fn operator_eq(&self, rhs: &TypeInstantiationCountMismatch) -> bool {
    self.function_name == rhs.function_name
      && self.function_type == rhs.function_type
      && self.provided_types == rhs.provided_types
      && self.maximum_types == rhs.maximum_types
      && self.provided_type_packs == rhs.provided_type_packs
      && self.maximum_type_packs == rhs.maximum_type_packs
  }
}
