//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/Type.h:1101:get_mutable`
//! Source: `Analysis/include/Luau/Type.h` (Type.h:1101-1110, hand-ported)

/// C++ `template<typename T> T* get_mutable(TypeId tv)` — call as `get_mutable::<TableType>(tv)`。
/// Rust 形态：`Option<&mut T>` 取代「裸指针 + is_null 检查」，unsafe 收口在本函数内。
///
/// 注意：与 C++ 一致，可变访问允许对同一 TypeId 的变体重写；别名安全由
/// 类型检查器的使用模式（单线程、顺序改写）保证。
use core::any::TypeId as CoreTypeId;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::as_mutable_type::as_mutable_type_id,
  type_aliases::{bound_type::BoundType, type_id::TypeId, type_variant::TypeVariantMember},
};
pub fn get_mutable<T: TypeVariantMember + 'static>(tv: TypeId) -> Option<&'static mut T> {
  LUAU_ASSERT!(!tv.is_null());

  // SAFETY: tv 的有效性由调用方按 C++ 同契约保证；as_mutable_type_id 仅去除
  // const（C++ const_cast 同义），此处解引用一次。
  let ty = unsafe { &mut (*as_mutable_type_id(tv)).ty };

  if CoreTypeId::of::<T>() != CoreTypeId::of::<BoundType>() {
    LUAU_ASSERT!(BoundType::get_if(ty).is_none());
  }

  T::get_if_mut(ty)
}

pub use get_mutable as get_mutable_type_id;
