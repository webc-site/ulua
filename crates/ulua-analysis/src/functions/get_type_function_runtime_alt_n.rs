//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:167-173` (hand-ported)
/// C++ `template<typename T> const T* get(TypeFunctionTypePackId tv)`.
use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::{
  type_function_type_pack_id::TypeFunctionTypePackId,
  type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_type_function_type_pack_id<T: TypeFunctionTypePackVariantMember>(
  tv: TypeFunctionTypePackId,
) -> *const T {
  LUAU_ASSERT!(!tv.is_null());
  if tv.is_null() {
    return null();
  }
  unsafe {
    match T::get_if(&(*tv).type_variant) {
      Some(r) => r as *const T,
      None => null(),
    }
  }
}
