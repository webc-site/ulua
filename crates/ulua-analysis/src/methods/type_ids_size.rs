use crate::records::type_ids::TypeIds;

impl TypeIds {
  pub fn size(&self) -> usize {
    self.order.len()
  }
}
