use crate::records::multiple_nonviable_overloads::MultipleNonviableOverloads;

impl MultipleNonviableOverloads {
  #[inline]
  pub fn operator_eq(&self, rhs: &MultipleNonviableOverloads) -> bool {
    self.attempted_arg_count == rhs.attempted_arg_count
  }
}
