use core::ptr::null_mut;

use crate::{
  records::{pending_type_pack::PendingTypePack, txn_log::TxnLog},
  type_aliases::type_pack_id::TypePackId,
};
impl TxnLog {
  pub fn pending_type_pack_id(&self, tp: TypePackId) -> *mut PendingTypePack {
    // This function will technically work if `this` is nullptr, but this
    // indicates a bug, so we explicitly assert.
    // (In Rust, `&self` is never null, so the C++ `LUAU_ASSERT(this != nullptr)`
    // has no analog here.)

    let mut current: *const TxnLog = self;
    while !current.is_null() {
      let cur = unsafe { &*current };
      if let Some(it) = cur.type_pack_changes.find(&tp) {
        return it.as_ref() as *const PendingTypePack as *mut PendingTypePack;
      }
      current = cur.parent;
    }

    null_mut()
  }
}
