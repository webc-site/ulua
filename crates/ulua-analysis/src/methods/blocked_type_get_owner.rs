use crate::records::{blocked_type::BlockedType, constraint::Constraint};

impl BlockedType {
  pub fn get_owner(&self) -> *const Constraint {
    self.owner
  }
}
