use crate::{
  functions::get_mutable_txn_log::get_mutable_pending_type,
  records::{pending_type::PendingType, table_type::TableType, txn_log::TxnLog},
  type_aliases::type_id::TypeId,
};
impl TxnLog {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn bind_table(
    &mut self,
    ty: TypeId,
    new_bound_to: Option<TypeId>,
  ) -> *mut PendingType {
    // SAFETY: We assume get<TableType>(ty) is valid per the C++ assertion
    // and that queue_type_id returns a valid PendingType pointer
    let new_ty = unsafe { self.queue_type_id(ty) };

    // SAFETY: get_mutable_pending_type is the Rust analog of get_mutable<TableType>
    // We assume the type is a TableType as per the C++ assertion
    unsafe {
      let table_type = get_mutable_pending_type::<TableType>(new_ty);
      if !table_type.is_null() {
        (*table_type).bound_to = new_bound_to;
      }
    }

    new_ty
  }
}
