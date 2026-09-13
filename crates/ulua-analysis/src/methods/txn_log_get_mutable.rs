//! @interface-stub
use core::ptr::{from_mut, null_mut};

use crate::{
  functions::{
    get_mutable_txn_log::get_mutable_pending_type,
    get_mutable_txn_log_alt_c::get_mutable_pending_type_pack,
    get_mutable_type::get_mutable_type_id, get_mutable_type_pack::get_mutable_type_pack_id,
  },
  records::txn_log::TxnLog,
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember,
    type_variant::TypeVariantMember,
  },
};
pub trait TxnLogGetMutable<TID>: Sized {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  unsafe fn get_mutable_from_log(log: &TxnLog, ty: TID) -> *mut Self;
}

impl<T: TypeVariantMember + 'static> TxnLogGetMutable<TypeId> for T {
  unsafe fn get_mutable_from_log(log: &TxnLog, ty: TypeId) -> *mut Self {
    unsafe {
      let pending_ty = log.pending_type_id(ty);
      if !pending_ty.is_null() {
        return get_mutable_pending_type::<T>(pending_ty);
      }

      get_mutable_type_id::<T>(ty).map_or(null_mut(), from_mut)
    }
  }
}

impl<T: TypePackVariantMember + 'static> TxnLogGetMutable<TypePackId> for T {
  unsafe fn get_mutable_from_log(log: &TxnLog, tp: TypePackId) -> *mut Self {
    unsafe {
      let pending_tp = log.pending_type_pack_id(tp);
      if !pending_tp.is_null() {
        return get_mutable_pending_type_pack::<T>(pending_tp);
      }

      get_mutable_type_pack_id::<T>(tp).map_or(null_mut(), from_mut)
    }
  }
}

impl TxnLog {
  pub fn txn_log_get_mutable<T, TID>(&self, ty: TID) -> *mut T
  where
    T: TxnLogGetMutable<TID>,
  {
    unsafe { T::get_mutable_from_log(self, ty) }
  }
}
