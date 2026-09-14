use crate::records::string_singleton::StringSingleton;

impl StringSingleton {
  #[inline]
  pub fn operator_eq(&self, rhs: &StringSingleton) -> bool {
    self.value == rhs.value
  }
}
