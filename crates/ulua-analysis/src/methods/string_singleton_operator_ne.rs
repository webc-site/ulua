use crate::records::string_singleton::StringSingleton;

impl StringSingleton {
  pub fn operator_ne(&self, rhs: &StringSingleton) -> bool {
    !self.operator_eq(rhs)
  }
}
