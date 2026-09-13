//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/Type.h:1089:get`
//! Source: `Analysis/include/Luau/Type.h` (Type.h:1089-1098, hand-ported)

/// C++ `template<typename T> const T* get(TypeId tv)` — call as `get::<TableType>(tv)`。
/// Rust 形态：`Option<&T>` 取代「裸指针 + is_null 检查」。
///
/// 生命周期取 `'static`：TypeId 指向 TypeArena 的分配，arena 与类型检查会话
/// 同寿，与 C++ 指针语义一致。
use core::any::TypeId as CoreTypeId;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::{
  bound_type::BoundType, type_id::TypeId, type_variant::TypeVariantMember,
};
pub fn get<T: TypeVariantMember + 'static>(tv: TypeId) -> Option<&'static T> {
  get_impl::<T>(tv)
}

/// 私有实现：裸指针解引用收口在此（C++ Type.h:1089 get 的直译）。
fn get_impl<T: TypeVariantMember + 'static>(tv: TypeId) -> Option<&'static T> {
  LUAU_ASSERT!(!tv.is_null());

  // SAFETY: tv 的有效性由调用方按 C++ 同契约保证；此处仅解引用一次。
  let ty = unsafe { &(*tv).ty };

  if CoreTypeId::of::<T>() != CoreTypeId::of::<BoundType>() {
    LUAU_ASSERT!(BoundType::get_if(ty).is_none());
  }

  T::get_if(ty)
}

pub use get as get_type_id;
