use crate::records::type_ids::TypeIds;

impl TypeIds {
  pub fn empty(&self) -> bool {
    self.order.is_empty()
  }
}
