use crate::records::type_pack_iterator::TypePackIterator;

impl TypePackIterator {
  pub fn operator_ne(&self, rhs: &TypePackIterator) -> bool {
    !self.operator_eq(rhs)
  }
}
