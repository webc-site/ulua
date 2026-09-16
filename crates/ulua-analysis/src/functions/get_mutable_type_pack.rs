//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypePack.h:215:get_mutable`
//! Source: `Analysis/include/Luau/TypePack.h` (TypePack.h:215-224, hand-ported)

/// C++ `template<typename T> T* get_mutable(TypePackId tp)`。
/// Rust 形态：`Option<&mut T>` 取代「裸指针 + is_null 检查」，unsafe 收口在本函数内。
use core::any::TypeId;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::as_mutable_type_pack::as_mutable_type_pack_id,
  type_aliases::{
    bound_type_pack::BoundTypePack, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariantMember,
  },
};
pub fn get_mutable<T: TypePackVariantMember + 'static>(tp: TypePackId) -> Option<&'static mut T> {
  LUAU_ASSERT!(!tp.is_null());

  // SAFETY: tp 的有效性由调用方按 C++ 同契约保证；as_mutable_type_pack_id 仅去除
  // const（C++ const_cast 同义），此处解引用一次。
  let ty = unsafe { &mut (*as_mutable_type_pack_id(tp)).ty };

  if TypeId::of::<T>() != TypeId::of::<BoundTypePack>() {
    LUAU_ASSERT!(BoundTypePack::get_if(ty).is_none());
  }

  T::get_if_mut(ty)
}

pub use get_mutable as get_mutable_type_pack_id;
