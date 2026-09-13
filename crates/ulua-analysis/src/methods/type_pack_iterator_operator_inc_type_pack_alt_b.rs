use crate::records::type_pack_iterator::TypePackIterator;

impl TypePackIterator {
  pub fn operator_inc_i32(&mut self) -> TypePackIterator {
    let copy = self.clone();
    self.operator_inc();
    copy
  }
}
