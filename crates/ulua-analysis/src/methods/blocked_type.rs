//! `blocked_type` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{blocked_type::BlockedType, constraint::Constraint};

impl BlockedType {
  pub fn get_owner(&self) -> *const Constraint {
    self.owner
  }
}

impl BlockedType {
  pub fn replace_owner(&mut self, new_owner: *const Constraint) {
    self.owner = new_owner;
  }
}

impl BlockedType {
  pub fn set_owner(&mut self, _new_owner: *const Constraint) {
    LUAU_ASSERT!(self.owner.is_null());

    if !self.owner.is_null() {
      return;
    }

    self.owner = _new_owner;
  }
}
