use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::txn_log::TxnLog, type_aliases::type_or_pack_id::TypeOrPackId};

impl TxnLog {
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
