use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack,
    occurs_txn_log::occurs_txn_log_type_id_type_id,
  },
  records::{
    arena_handle::{alias, alias_ref},
    pending_type::PendingType,
    pending_type_pack::PendingTypePack,
    txn_log::TxnLog,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TxnLog {
  pub fn commit(&mut self) {
    LUAU_ASSERT!(!self.radioactive);

    // The change maps are not mutated by occurs()/followOnce() (those only read
    // pending state), and PendingSlot boxes its payload, so we snapshot the raw
    // pointers up front. This mirrors C++ iterating the map while passing
    // `*this` to occurs().
    let type_var_entries: Vec<(TypeId, *const PendingType, bool)> = self
      .type_var_changes
      .iter()
      .map(|(ty, rep)| (*ty, rep.get() as *const PendingType, rep.get().dead))
      .collect();

    for (ty, rep, dead) in type_var_entries {
      if !dead {
        let unfollowed: TypeId = &alias_ref(rep).pending as *const _;

        if !occurs_txn_log_type_id_type_id(self, unfollowed, ty) {
          alias(as_mutable_type_id(ty)).reassign(alias_ref(unfollowed));
        }
      }
    }

    let type_pack_entries: Vec<(TypePackId, *const PendingTypePack)> = self
      .type_pack_changes
      .iter()
      .map(|(tp, rep)| (*tp, rep.get() as *const PendingTypePack))
      .collect();

    for (tp, rep) in type_pack_entries {
      alias(as_mutable_type_pack(tp)).reassign(&alias_ref(rep).pending);
    }

    self.clear();
  }
}
