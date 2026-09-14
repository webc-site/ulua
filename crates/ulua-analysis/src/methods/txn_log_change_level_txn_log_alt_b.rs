use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_mutable_txn_log_alt_c::get_mutable_pending_type_pack,
  records::{
    free_type_pack::FreeTypePack, pending_type_pack::PendingTypePack, txn_log::TxnLog,
    type_level::TypeLevel,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl TxnLog {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn change_level_type_pack_id_type_level(
    &mut self,
    tp: TypePackId,
    new_level: TypeLevel,
  ) -> *mut PendingTypePack {
    LUAU_ASSERT!(self.txn_log_is::<FreeTypePack, TypePackId>(tp));

    let new_tp = unsafe { self.queue_type_pack_id(tp) };
    unsafe {
      let ftp = get_mutable_pending_type_pack::<FreeTypePack>(new_tp);
      if !ftp.is_null() {
        (*ftp).level = new_level;
      }
    }

    new_tp
  }
}
