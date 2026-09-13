use crate::records::{pending_type::PendingType, r#type::Type};

impl PendingType {
  pub fn pending_type(&mut self, state: Type) {
    self.pending = state;
    self.dead = false;
  }
}
