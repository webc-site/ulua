use core::ptr::null;

use crate::{
  records::{table_type::TableType, txn_log::TxnLog},
  type_aliases::{bound_type::BoundType, type_id::TypeId},
};
pub fn follow_once(log: &mut TxnLog, ty: TypeId) -> TypeId {
  if let Some(bound) = log.txn_log_get::<BoundType, TypeId>(ty) {
    return bound.bound_to;
  }

  if let Some(tt) = log.txn_log_get::<TableType, TypeId>(ty) {
    return tt.bound_to.unwrap_or(null());
  }

  null()
}
