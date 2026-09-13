//! Node: `cxx:Function:Luau.Analysis:Analysis/include/Luau/TypeUtils.h:230:get`
//! Source: `Analysis/include/Luau/TypeUtils.h:229-236` (hand-ported)

/// Dispatch of the inner `get<T>(*ty)` per id type `Ty`: C++ resolves the
/// overload by argument type; Rust needs the trait.
use core::ptr::null;

use crate::{
  functions::{get_type_alt_j::get_type_id, get_type_pack::get_type_pack_id},
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariantMember,
    type_variant::TypeVariantMember,
  },
};
pub trait GetThroughId<Ty: Copy>: Sized {
  /// # Safety
  /// `ty` must be a valid id pointer (the C++ overloads dereference it).
  unsafe fn get_through(ty: Ty) -> *const Self;
}

impl<T: TypeVariantMember + 'static> GetThroughId<TypeId> for T {
  unsafe fn get_through(ty: TypeId) -> *const T {
    get_type_id::<T>(ty).map_or(null(), |r| r as *const T)
  }
}

impl<T: TypePackVariantMember + 'static> GetThroughId<TypePackId> for T {
  unsafe fn get_through(tp: TypePackId) -> *const T {
    get_type_pack_id::<T>(tp).map_or(null(), |r| r as *const T)
  }
}

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
/// C++ `template<typename T, typename Ty> const T* get(std::optional<Ty> ty)`.
pub unsafe fn get_optional_ty<T: GetThroughId<Ty>, Ty: Copy>(ty: Option<Ty>) -> *const T {
  unsafe {
    if let Some(ty) = ty {
      T::get_through(ty)
    } else {
      null()
    }
  }
}
