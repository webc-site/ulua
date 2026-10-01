//! Source: `Analysis/include/Luau/TypePack.h` (TypePack.h:215-224, hand-ported)

/// C++ `template<typename T> T* get_mutable(TypePackId tp)`。
/// Rust 形态：`Option<&mut T>` 取代「裸指针 + is_null 检查」，unsafe 收口在本函数内。
use core::any::TypeId;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::as_mutable_type_pack::as_mutable_type_pack,
  records::arena_handle::alias,
  type_aliases::{
    bound_type_pack::BoundTypePack, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariantMember,
  },
};
pub fn get_mutable<T: TypePackVariantMember + 'static>(tp: TypePackId) -> Option<&'static mut T> {
  LUAU_ASSERT!(!tp.is_null());

  let ty = &mut alias(as_mutable_type_pack(tp)).ty;

  if TypeId::of::<T>() != TypeId::of::<BoundTypePack>() {
    LUAU_ASSERT!(BoundTypePack::get_if(ty).is_none());
  }

  T::get_if_mut(ty)
}
