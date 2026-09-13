use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{substitution::Substitution, txn_log::TxnLog, type_arena::TypeArena};

impl Substitution {
  pub fn substitution_txn_log_type_arena(&mut self, log_: *const TxnLog, arena: *mut TypeArena) {
    self.arena = arena;
    self.base.log = log_;
    LUAU_ASSERT!(!log_.is_null());
  }
}
