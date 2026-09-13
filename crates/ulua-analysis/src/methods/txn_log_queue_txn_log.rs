use core::ptr::null_mut;

use crate::{
  records::{pending_type::PendingType, txn_log::TxnLog},
  type_aliases::type_id::TypeId,
};
impl TxnLog {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn queue_type_id(&mut self, ty: TypeId) -> *mut PendingType {
    unsafe {
      if (*ty).persistent {
        self.radioactive = true;
      }

      if let Some(existing) = self.type_var_changes.find_mut(&ty) {
        if !existing.dead {
          return existing.as_mut() as *mut PendingType;
        }

        let mut pending = (*ty).clone();
        pending.owning_arena = null_mut();
        **existing = PendingType {
          pending,
          dead: false,
        };
        return existing.as_mut() as *mut PendingType;
      }

      let mut pending = (*ty).clone();
      pending.owning_arena = null_mut();
      let (entry, _) = self.type_var_changes.try_insert(
        ty,
        Box::new(PendingType {
          pending,
          dead: false,
        }),
      );

      entry.as_mut() as *mut PendingType
    }
  }
}
