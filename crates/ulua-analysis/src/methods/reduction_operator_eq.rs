use crate::records::reduction::Reduction;

impl Reduction {
  pub fn operator_eq(&self, other: &Reduction) -> bool {
    self.result_type == other.result_type
  }
}
