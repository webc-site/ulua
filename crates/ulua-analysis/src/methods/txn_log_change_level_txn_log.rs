use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  functions::get_mutable_txn_log::get_mutable_pending_type,
  records::{
    free_type::FreeType, function_type::FunctionType, pending_type::PendingType,
    table_type::TableType, txn_log::TxnLog, type_level::TypeLevel,
  },
  type_aliases::type_id::TypeId,
};

impl TxnLog {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn change_level_type_id_type_level(
    &mut self,
    ty: TypeId,
    new_level: TypeLevel,
  ) -> *mut PendingType {
    LUAU_ASSERT!(
      self.txn_log_is::<FreeType, TypeId>(ty)
        || self.txn_log_is::<TableType, TypeId>(ty)
        || self.txn_log_is::<FunctionType, TypeId>(ty)
    );

    let new_ty = unsafe { self.queue_type_id(ty) };
    unsafe {
      let ftv = get_mutable_pending_type::<FreeType>(new_ty);
      if !ftv.is_null() {
        (*ftv).level = new_level;
      } else {
        let ttv = get_mutable_pending_type::<TableType>(new_ty);
        if !ttv.is_null() {
          LUAU_ASSERT!((*ttv).state == TableState::Free || (*ttv).state == TableState::Generic);
          (*ttv).level = new_level;
        } else {
          let ftv = get_mutable_pending_type::<FunctionType>(new_ty);
          if !ftv.is_null() {
            (*ftv).level = new_level;
          }
        }
      }
    }

    new_ty
  }
}
