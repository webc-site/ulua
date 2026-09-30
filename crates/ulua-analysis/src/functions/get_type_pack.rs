//! Source: `Analysis/include/Luau/TypePack.h` (TypePack.h:204-213, hand-ported)

/// C++ `template<typename T> const T* get(TypePackId tp)`。
/// Rust 形态：`Option<&T>` 取代「裸指针 + is_null 检查」。
/// 生命周期见 [`crate::functions::get_type::get`]（同一 TypeArena 契约）。
use core::any::TypeId;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::{
  bound_type_pack::BoundTypePack,
  type_pack_id::TypePackId,
  type_pack_variant::{TypePackVariant, TypePackVariantMember},
};

/// arena 类型包节点的整变体只读视图（C++ `TypePackVar& get(TypePackId)` 的直译）。
/// 与 [`crate::functions::get_type::type_variant_of`] 同一契约、同一收口思路。
///
/// # Safety 说明（类型级契约，同 [`get`]）
/// `tp` 非空、指向类型 arena 内地址稳定的 `TypePackVar` 节点，在被返回引用
/// 使用期间不失效；此处仅解引用一次。
pub(crate) fn type_pack_variant_of(tp: TypePackId) -> &'static TypePackVariant {
  LUAU_ASSERT!(!tp.is_null());
  // SAFETY: tp 的有效性由调用方按 C++ 同契约保证；此处仅解引用一次。
  unsafe { &(*tp).ty }
}

/// arena pack 节点 `persistent` 标志只读视图（C++ `tp->persistent` 的收口）。
///
/// 与 [`type_pack_variant_of`] 同一 arena 节点有效性契约。
pub(crate) fn pack_is_persistent(tp: TypePackId) -> bool {
  // SAFETY: 同 [`type_pack_variant_of`]，契约由调用方保证。
  unsafe { (*tp).persistent }
}

pub fn get<T: TypePackVariantMember + 'static>(tp: TypePackId) -> Option<&'static T> {
  get_impl::<T>(tp)
}

/// 私有实现：变体判别收口在 [`type_pack_variant_of`] 一处（C++ TypePack.h:204 get 的直译）。
fn get_impl<T: TypePackVariantMember + 'static>(tp: TypePackId) -> Option<&'static T> {
  let ty = type_pack_variant_of(tp);

  if TypeId::of::<T>() != TypeId::of::<BoundTypePack>() {
    LUAU_ASSERT!(BoundTypePack::get_if(ty).is_none());
  }

  T::get_if(ty)
}
