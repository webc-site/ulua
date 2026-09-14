//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TxnLog.h:58:get_mutable`
//! Source: `Analysis/include/Luau/TxnLog.h:58-63` (hand-ported)

/// C++ `template<typename T> T* get_mutable(PendingTypePack* pending)`.
use core::ptr::null_mut;

use crate::{
  functions::get_mutable_type_pack::get_mutable_type_pack_id,
  records::pending_type_pack::PendingTypePack,
  type_aliases::type_pack_variant::TypePackVariantMember,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_mutable_pending_type_pack<T: TypePackVariantMember + 'static>(
  pending: *mut PendingTypePack,
) -> *mut T {
  unsafe {
    // We use get_mutable here because this state is intended to be mutated freely.
    // SAFETY: pending 有效性由调用方契约保证（C++ TxnLog.h:58 同款）。
    get_mutable_type_pack_id::<T>(&(*pending).pending).map_or(null_mut(), |r| r as *mut T)
  }
}
