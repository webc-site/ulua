use crate::records::type_pack_iterator::TypePackIterator;

impl TypePackIterator {
  pub fn operator_eq(&self, rhs: &TypePackIterator) -> bool {
    self.tp == rhs.tp && self.current_index == rhs.current_index
  }
}
