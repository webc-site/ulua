use crate::records::type_ids::TypeIds;

impl TypeIds {
  pub fn reserve(&mut self, n: usize) {
    self.order.reserve(n);
  }
}
