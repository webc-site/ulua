use core::ptr::null_mut;

use crate::{
  records::{pending_type::PendingType, txn_log::TxnLog},
  type_aliases::type_id::TypeId,
};
impl TxnLog {
  pub fn pending_type_id(&self, ty: TypeId) -> *mut PendingType {
    // This function will technically work if `this` is nullptr, but this
    // indicates a bug, so we explicitly assert.
    // (In Rust, `&self` is never null, so the C++ `LUAU_ASSERT(this != nullptr)`
    // has no analog here.)

    let mut current: *const TxnLog = self;
    while !current.is_null() {
      let cur = unsafe { &*current };
      if let Some(it) = cur.type_var_changes.find(&ty)
        && !it.dead
      {
        return it.as_ref() as *const PendingType as *mut PendingType;
      }
      current = cur.parent;
    }

    null_mut()
  }
}
