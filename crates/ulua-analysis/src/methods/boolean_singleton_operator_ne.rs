use crate::records::boolean_singleton::BooleanSingleton;

impl BooleanSingleton {
  #[inline]
  pub fn operator_ne(&self, rhs: &BooleanSingleton) -> bool {
    !self.operator_eq(rhs)
  }
}
