use crate::records::singleton_type::SingletonType;

impl SingletonType {
  #[inline]
  pub fn operator_eq(&self, rhs: &SingletonType) -> bool {
    self.variant == rhs.variant
  }
}
