use alloc::{boxed::Box, vec::Vec};
use core::ptr::{null, null_mut};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  pending_type::PendingType, pending_type_pack::PendingTypePack, txn_log::TxnLog,
};
impl TxnLog {
  pub fn inverse(&self) -> TxnLog {
    // C++ `TxnLog inversed(sharedSeen)` — the sharedSeen-taking constructor.
    let mut inversed = TxnLog {
      type_var_changes: DenseHashMap::new(null()),
      type_pack_changes: DenseHashMap::new(null()),
      parent: null_mut(),
      owned_seen: Vec::new(),
      // Borrows the original's seen set (C++ `inversed(sharedSeen)`).
      shared_seen: self.shared_seen,
      owned_seen_box: None,
      radioactive: false,
    };

    for (ty, rep) in self.type_var_changes.iter() {
      if !rep.dead {
        inversed.type_var_changes.try_insert(
          *ty,
          Box::new(PendingType {
            pending: unsafe { (**ty).clone() },
            dead: false,
          }),
        );
      }
    }

    for (tp, _rep) in self.type_pack_changes.iter() {
      inversed.type_pack_changes.try_insert(
        *tp,
        Box::new(PendingTypePack {
          pending: unsafe { (**tp).clone() },
        }),
      );
    }

    inversed.radioactive = self.radioactive;

    inversed
  }
}
