use crate::records::swapped_generic_type_parameter::SwappedGenericTypeParameter;

impl SwappedGenericTypeParameter {
  #[inline]
  pub fn operator_eq(&self, rhs: &SwappedGenericTypeParameter) -> bool {
    self.name == rhs.name && self.kind == rhs.kind
  }
}
