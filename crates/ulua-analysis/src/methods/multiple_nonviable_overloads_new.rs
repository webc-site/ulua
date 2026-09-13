use crate::records::multiple_nonviable_overloads::MultipleNonviableOverloads;

impl MultipleNonviableOverloads {
  pub fn new(attempted_arg_count: usize) -> Self {
    Self {
      attempted_arg_count,
    }
  }
}
