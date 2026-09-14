//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypePack.h:204:get`
//! Source: `Analysis/include/Luau/TypePack.h` (TypePack.h:204-213, hand-ported)

/// C++ `template<typename T> const T* get(TypePackId tp)`。
/// Rust 形态：`Option<&T>` 取代「裸指针 + is_null 检查」。
/// 生命周期见 [`crate::functions::get_type_alt_j::get`]（同一 TypeArena 契约）。
use core::any::TypeId;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::{
  bound_type_pack::BoundTypePack, type_pack_id::TypePackId,
  type_pack_variant::TypePackVariantMember,
};
pub fn get<T: TypePackVariantMember + 'static>(tp: TypePackId) -> Option<&'static T> {
  get_impl::<T>(tp)
}

/// 私有实现：裸指针解引用收口在此（C++ TypePack.h:204 get 的直译）。
fn get_impl<T: TypePackVariantMember + 'static>(tp: TypePackId) -> Option<&'static T> {
  LUAU_ASSERT!(!tp.is_null());

  // SAFETY: tp 的有效性由调用方按 C++ 同契约保证；此处仅解引用一次。
  let ty = unsafe { &(*tp).ty };

  if TypeId::of::<T>() != TypeId::of::<BoundTypePack>() {
    LUAU_ASSERT!(BoundTypePack::get_if(ty).is_none());
  }

  T::get_if(ty)
}

pub use get as get_type_pack_id;
