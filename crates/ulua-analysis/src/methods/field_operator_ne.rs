use crate::records::field::Field;

impl Field {
  pub fn operator_ne(&self, rhs: &Field) -> bool {
    !self.operator_eq(rhs)
  }
}
