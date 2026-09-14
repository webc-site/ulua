use crate::{
  records::{pending_type_pack::PendingTypePack, txn_log::TxnLog, type_pack_var::TypePackVar},
  type_aliases::type_pack_id::TypePackId,
};

impl TxnLog {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn replace_type_pack_id_type_pack_var(
    &mut self,
    tp: TypePackId,
    replacement: TypePackVar,
  ) -> *mut PendingTypePack {
    let new_tp = unsafe { self.queue_type_pack_id(tp) };
    unsafe {
      (*new_tp).pending.reassign(&replacement);
    }
    new_tp
  }
}
