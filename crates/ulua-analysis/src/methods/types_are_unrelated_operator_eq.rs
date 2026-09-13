use crate::records::types_are_unrelated::TypesAreUnrelated;

impl TypesAreUnrelated {
  #[inline]
  pub fn operator_eq(&self, rhs: &TypesAreUnrelated) -> bool {
    self.left == rhs.left && self.right == rhs.right
  }
}
