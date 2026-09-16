use crate::records::boolean_singleton::BooleanSingleton;

impl BooleanSingleton {
  #[inline]
  pub fn operator_eq(&self, rhs: &BooleanSingleton) -> bool {
    self.value == rhs.value
  }
}
