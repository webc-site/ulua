use crate::{
  records::{txn_log::TxnLog, type_pack_iterator::TypePackIterator},
  type_aliases::type_pack_id::TypePackId,
};

impl TypePackIterator {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn type_pack_iterator_type_pack_id(&mut self, _type_pack: TypePackId) {
    unsafe { self.type_pack_iterator_type_pack_id_txn_log(_type_pack, TxnLog::empty()) };
  }
}
