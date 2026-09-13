//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TxnLog.h:51:get_mutable`
//! Source: `Analysis/include/Luau/TxnLog.h:51-56` (hand-ported)

/// C++ `template<typename T> T* get_mutable(PendingType* pending)`.
use core::ptr::null_mut;

use crate::{
  functions::get_mutable_type::get_mutable_type_id, records::pending_type::PendingType,
  type_aliases::type_variant::TypeVariantMember,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_mutable_pending_type<T: TypeVariantMember + 'static>(
  pending: *mut PendingType,
) -> *mut T {
  unsafe {
    // We use get_mutable here because this state is intended to be mutated freely.
    // SAFETY: pending 有效性由调用方契约保证（C++ TxnLog.h:51 同款）。
    get_mutable_type_id::<T>(&(*pending).pending).map_or(null_mut(), |r| r as *mut T)
  }
}
