use core::ptr::null_mut;

use crate::{
  records::{pending_type_pack::PendingTypePack, txn_log::TxnLog},
  type_aliases::type_pack_id::TypePackId,
};
impl TxnLog {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn queue_type_pack_id(&mut self, tp: TypePackId) -> *mut PendingTypePack {
    unsafe {
      if (*tp).persistent {
        self.radioactive = true;
      }

      if let Some(existing) = self.type_pack_changes.find_mut(&tp) {
        return existing.as_mut() as *mut PendingTypePack;
      }

      let mut pending = (*tp).clone();
      pending.owning_arena = null_mut();
      let (entry, _) = self
        .type_pack_changes
        .try_insert(tp, Box::new(PendingTypePack { pending }));

      entry.as_mut() as *mut PendingTypePack
    }
  }
}
