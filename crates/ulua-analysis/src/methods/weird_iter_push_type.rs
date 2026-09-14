use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_mutable_txn_log_alt_c::get_mutable_pending_type_pack,
  records::{type_pack::TypePack, weird_iter::WeirdIter},
  type_aliases::type_id::TypeId,
};

impl WeirdIter {
  pub fn weird_iter_push_type(&mut self, ty: TypeId) {
    LUAU_ASSERT!(!self.pack.is_null());
    let pending_pack = unsafe { (*self.log).queue_type_pack_id(self.pack_id) };
    let pending = unsafe { get_mutable_pending_type_pack::<TypePack>(pending_pack) };
    if !pending.is_null() {
      unsafe {
        (*pending).head.push(ty);
      }
      self.pack = pending;
    } else {
      LUAU_ASSERT!(false);
    }
  }
}
