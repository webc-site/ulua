use crate::records::unsupported_type::UnsupportedType;

impl UnsupportedType {
  #[inline]
  pub fn operator_eq(&self, rhs: &UnsupportedType) -> bool {
    self.r#type == rhs.r#type
  }
}
