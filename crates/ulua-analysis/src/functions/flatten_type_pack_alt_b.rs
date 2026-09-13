use alloc::vec::Vec;

use crate::{
  functions::end_type_pack::end_type_pack_id,
  records::{txn_log::TxnLog, type_pack_iterator::TypePackIterator},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub(crate) fn flatten(tp: TypePackId, log: &TxnLog) -> (Vec<TypeId>, Option<TypePackId>) {
  let tp = unsafe { log.follow_type_pack_id(tp) };
  let mut flattened = Vec::new();
  let mut it = TypePackIterator::new();
  unsafe { it.type_pack_iterator_type_pack_id_txn_log(tp, log as *const TxnLog) };

  while it.operator_ne(&end_type_pack_id(tp)) {
    flattened.push(*it.operator_deref());
    it.operator_inc();
  }

  let tail = it.tail();
  (flattened, tail)
}
