use crate::{
  functions::get_type_alt_j::get,
  records::{
    blocked_type::BlockedType, pending_expansion_type::PendingExpansionType, txn_log::TxnLog,
  },
  type_aliases::type_id::TypeId,
};
/// # Safety
/// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub(crate) fn is_blocked_txn_log_type_id(log: &TxnLog, ty: TypeId) -> bool {
  let ty = unsafe { log.follow_type_id(ty) };
  !get::<BlockedType>(ty).is_none() || !get::<PendingExpansionType>(ty).is_none()
}
