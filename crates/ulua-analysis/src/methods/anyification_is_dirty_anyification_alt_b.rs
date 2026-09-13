use crate::{
  records::{anyification::Anyification, free_type_pack::FreeTypePack},
  type_aliases::type_pack_id::TypePackId,
};

impl Anyification {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  /// `bool Anyification::isDirty(TypePackId tp)` (Anyification.cpp:54-62).
  pub unsafe fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    if unsafe { (*tp).persistent } {
      return false;
    }

    // C++: `if (log->get_mutable<FreeTypePack>(tp)) return true; else return false;`
    let log = self.base.base.log;
    let ftp = unsafe { (*log).txn_log_get_mutable::<FreeTypePack, TypePackId>(tp) };
    !ftp.is_null()
  }
}
