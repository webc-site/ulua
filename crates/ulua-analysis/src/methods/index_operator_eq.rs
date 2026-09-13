use crate::records::index::Index;

impl Index {
  pub fn operator_eq(&self, other: &Index) -> bool {
    self.index == other.index
  }
}
