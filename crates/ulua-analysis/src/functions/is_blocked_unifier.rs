use crate::{
  functions::{get_type::get, get_type_pack::get as get_pack},
  records::{
    blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    pending_expansion_type::PendingExpansionType, txn_log::TxnLog,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub(crate) fn is_blocked_txn_log_type_id(log: &TxnLog, ty: TypeId) -> bool {
  let ty = log.follow_type_id(ty);
  get::<BlockedType>(ty).is_some() || get::<PendingExpansionType>(ty).is_some()
}

pub(crate) fn is_blocked_txn_log_type_pack_id(log: &TxnLog, tp: TypePackId) -> bool {
  let tp = log.follow_type_pack_id(tp);
  get_pack::<BlockedTypePack>(tp).is_some()
}
