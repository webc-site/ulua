use crate::{
  records::{pending_type::PendingType, txn_log::TxnLog, r#type::Type},
  type_aliases::type_id::TypeId,
};

impl TxnLog {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn replace_type_id_type_item(
    &mut self,
    _ty: TypeId,
    _replacement: Type,
  ) -> *mut PendingType {
    let new_ty = unsafe { self.queue_type_id(_ty) };
    unsafe {
      (*new_ty).pending.reassign(&_replacement);
    }
    new_ty
  }
}
