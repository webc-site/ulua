use crate::{records::txn_log::TxnLog, type_aliases::type_or_pack_id::TypeOrPackId};

impl TxnLog {
  pub fn have_seen_type_or_pack_id_type_or_pack_id(
    &self,
    lhs: TypeOrPackId,
    rhs: TypeOrPackId,
  ) -> bool {
    let sorted_pair = if lhs > rhs { (lhs, rhs) } else { (rhs, lhs) };

    if self.shared_seen.is_null() {
      return false;
    }

    let shared_seen = unsafe { &*self.shared_seen };
    shared_seen.contains(&sorted_pair)
  }
}
