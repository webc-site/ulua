use crate::{
  records::txn_log::{TxnLog, TypeOrPackId},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  #[inline]
  pub fn have_seen_type_id_type_id(&self, lhs: TypeId, rhs: TypeId) -> bool {
    self.have_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId)
  }

  #[inline]
  pub fn have_seen_type_pack_id_type_pack_id(&self, lhs: TypePackId, rhs: TypePackId) -> bool {
    self.have_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId)
  }

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
