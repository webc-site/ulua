use core::ptr::null;

use crate::records::constraint::Constraint;
#[derive(Debug, Clone)]
pub struct BlockedType {
  pub(crate) index: i32,
  /// The constraint that is intended to unblock this type. Other constraints
  /// should block on this constraint if present.
  pub(crate) owner: *const Constraint,
}

impl Default for BlockedType {
  fn default() -> Self {
    Self {
      index: 0,
      owner: null(),
    }
  }
}

impl BlockedType {
  pub fn blocked_type(&mut self) {
    self.index = 0;
    self.owner = null();
  }
}
