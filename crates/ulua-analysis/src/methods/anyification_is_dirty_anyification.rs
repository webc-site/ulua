use crate::{
  enums::table_state::TableState,
  records::{anyification::Anyification, free_type::FreeType, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl Anyification {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    unsafe {
      if (*ty).persistent {
        return false;
      }

      let log = self.base.base.log;

      let ttv = (*log).txn_log_get_mutable::<TableType, TypeId>(ty);
      if !ttv.is_null() {
        return (*ttv).state == TableState::Free || (*ttv).state == TableState::Unsealed;
      }

      let ftv = (*log).txn_log_get_mutable::<FreeType, TypeId>(ty);
      !ftv.is_null()
    }
  }
}
