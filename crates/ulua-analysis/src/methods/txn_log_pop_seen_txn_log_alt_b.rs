use crate::{records::txn_log::TxnLog, type_aliases::type_pack_id::TypePackId};

impl TxnLog {
  pub fn pop_seen_type_pack_id_type_pack_id(&mut self, lhs: TypePackId, rhs: TypePackId) {
    self.pop_seen_type_or_pack_id_type_or_pack_id(lhs as _, rhs as _);
  }
}
