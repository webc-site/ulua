use alloc::{boxed::Box, vec::Vec};

use crate::{
  records::txn_log::{SeenStorage, TxnLog, TypeOrPackId},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  pub fn push_seen_type_id_type_id(&mut self, lhs: TypeId, rhs: TypeId) {
    let lhs_top: TypeOrPackId = lhs as TypeOrPackId;
    let rhs_top: TypeOrPackId = rhs as TypeOrPackId;
    self.push_seen_type_or_pack_id_type_or_pack_id(lhs_top, rhs_top);
  }

  pub fn push_seen_type_pack_id_type_pack_id(&mut self, lhs: TypePackId, rhs: TypePackId) {
    self.push_seen_type_or_pack_id_type_or_pack_id(lhs as TypeOrPackId, rhs as TypeOrPackId);
  }

  pub fn push_seen_type_or_pack_id_type_or_pack_id(
    &mut self,
    lhs: TypeOrPackId,
    rhs: TypeOrPackId,
  ) {
    if self.shared_seen.is_null() {
      // Lazily own a fresh seen set (freed on drop) instead of leaking it.
      let mut seen_box = Box::new(SeenStorage(Vec::new()));
      self.shared_seen = &mut seen_box.0 as *mut _;
      self.owned_seen_box = Some(seen_box);
    }

    let sorted_pair = if lhs > rhs { (lhs, rhs) } else { (rhs, lhs) };

    unsafe {
      (*self.shared_seen).push(sorted_pair);
    }
  }
}
