//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeFunctionRuntime.h:283:get_mutable`
//! Source: `Analysis/include/Luau/TypeFunctionRuntime.h:283-289` (hand-ported)

/// C++ `template<typename T> T* get_mutable(TypeFunctionTypeId tv)`.
use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::type_function_type::TypeFunctionType,
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_mutable_type_function_type_id<T: TypeFunctionTypeVariantMember>(
  tv: TypeFunctionTypeId,
) -> *mut T {
  unsafe {
    LUAU_ASSERT!(!tv.is_null());

    if tv.is_null() {
      return null_mut();
    }
    // C++ `get_if<T>(&const_cast<TypeFunctionType*>(tv)->type)`.
    match T::get_if_mut(&mut (*(tv as *mut TypeFunctionType)).type_variant) {
      Some(r) => r as *mut T,
      None => null_mut(),
    }
  }
}
