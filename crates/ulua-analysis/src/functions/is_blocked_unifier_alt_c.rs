use crate::{
  functions::get_type_pack::get,
  records::{blocked_type_pack::BlockedTypePack, txn_log::TxnLog},
  type_aliases::type_pack_id::TypePackId,
};
/// # Safety
/// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub(crate) fn is_blocked_txn_log_type_pack_id(log: &TxnLog, tp: TypePackId) -> bool {
  let tp = unsafe { log.follow_type_pack_id(tp) };
  !get::<BlockedTypePack>(tp).is_none()
}
