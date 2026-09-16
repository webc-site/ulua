//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeFunctionRuntime.h:175:get_mutable`
//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:175-181` (hand-ported)

/// C++ `template<typename T> T* get_mutable(TypeFunctionTypePackId tv)`.
use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::type_function_type_pack_var::TypeFunctionTypePackVar,
  type_aliases::{
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
  },
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_mutable_type_function_type_pack_id<T: TypeFunctionTypePackVariantMember>(
  tv: TypeFunctionTypePackId,
) -> *mut T {
  unsafe {
    LUAU_ASSERT!(!tv.is_null());

    if tv.is_null() {
      return null_mut();
    }
    // C++ `get_if<T>(&const_cast<TypeFunctionTypePackVar*>(tv)->type)`.
    match T::get_if_mut(&mut (*(tv as *mut TypeFunctionTypePackVar)).type_variant) {
      Some(r) => r as *mut T,
      None => null_mut(),
    }
  }
}
