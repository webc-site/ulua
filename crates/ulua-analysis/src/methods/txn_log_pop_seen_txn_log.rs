use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::txn_log::{TxnLog, TypeOrPackId},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  pub fn pop_seen_type_id_type_id(&mut self, lhs: TypeId, rhs: TypeId) {
    self.pop_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId);
  }

  pub fn pop_seen_type_pack_id_type_pack_id(&mut self, lhs: TypePackId, rhs: TypePackId) {
    self.pop_seen_type_or_pack_id_type_or_pack_id(lhs as _, rhs as _);
  }

  pub fn pop_seen_type_or_pack_id_type_or_pack_id(&mut self, lhs: TypeOrPackId, rhs: TypeOrPackId) {
    if self.shared_seen.is_null() {
      return;
    }

    let sorted_pair = if lhs > rhs { (lhs, rhs) } else { (rhs, lhs) };

    unsafe {
      let shared_seen = &*self.shared_seen;
      LUAU_ASSERT!(shared_seen.last() == Some(&sorted_pair));
      (*self.shared_seen).pop();
    }
  }
}
