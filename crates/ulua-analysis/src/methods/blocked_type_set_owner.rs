use crate::records::{blocked_type::BlockedType, constraint::Constraint};

impl BlockedType {
  pub fn set_owner(&mut self, _new_owner: *const Constraint) {
    ulua_common::macros::luau_assert::LUAU_ASSERT!(self.owner.is_null());

    if !self.owner.is_null() {
      return;
    }

    self.owner = _new_owner;
  }
}
