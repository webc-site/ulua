use core::ptr::{null, null_mut};

use crate::{
  records::{txn_log::TxnLog, type_pack::TypePack, type_pack_iterator::TypePackIterator},
  type_aliases::type_pack_id::TypePackId,
};
impl TypePackIterator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn type_pack_iterator_type_pack_id_txn_log(
    &mut self,
    type_pack: TypePackId,
    log: *const TxnLog,
  ) {
    self.current_type_pack = unsafe { (*log).follow_type_pack_id(type_pack) };
    self.tp = unsafe { (*log).txn_log_get::<TypePack, TypePackId>(self.current_type_pack) };
    self.current_index = 0;
    self.log = log;

    while !self.tp.is_null() && unsafe { (*self.tp).head.is_empty() } {
      self.current_type_pack = if let Some(tail) = unsafe { (*self.tp).tail } {
        unsafe { (*log).follow_type_pack_id(tail) }
      } else {
        null()
      };

      self.tp = if !self.current_type_pack.is_null() {
        unsafe { (*log).txn_log_get_mutable::<TypePack, TypePackId>(self.current_type_pack) }
      } else {
        null_mut()
      };
    }
  }
}
