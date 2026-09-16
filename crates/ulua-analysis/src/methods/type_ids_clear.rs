use crate::records::type_ids::TypeIds;

impl TypeIds {
  pub fn clear(&mut self) {
    self.order.clear();
    self.types.clear();
    self.hash = 0;
  }
}
