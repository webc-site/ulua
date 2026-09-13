use crate::records::duplicate_generic_parameter::DuplicateGenericParameter;

impl DuplicateGenericParameter {
  #[inline]
  pub fn operator_eq(&self, rhs: &DuplicateGenericParameter) -> bool {
    self.parameter_name == rhs.parameter_name
  }
}
