use crate::records::singleton_type::SingletonType;

impl SingletonType {
  pub fn operator_ne(&self, rhs: &SingletonType) -> bool {
    !self.operator_eq(rhs)
  }
}
