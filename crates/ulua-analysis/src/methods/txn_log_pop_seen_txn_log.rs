use crate::{
  records::txn_log::TxnLog,
  type_aliases::{type_id::TypeId, type_or_pack_id::TypeOrPackId},
};

impl TxnLog {
  pub fn pop_seen_type_id_type_id(&mut self, lhs: TypeId, rhs: TypeId) {
    self.pop_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId);
  }
}
