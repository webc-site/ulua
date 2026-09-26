//! Source: `Analysis/include/Luau/TypeUtils.h:229-236` (hand-ported)

/// Dispatch of the inner `get<T>(*ty)` per id type `Ty`: C++ resolves the
/// overload by argument type; Rust needs the trait.
use core::ptr::null;

use crate::{
  functions::{get_type, get_type_pack},
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
    get_type::get::<T>(ty).map_or(null(), |r| r as *const T)
  }
}

impl<T: TypePackVariantMember + 'static> GetThroughId<TypePackId> for T {
  /// # Safety
  /// `tp` 须为指向存活类型 pack arena 节点的有效 `TypePackId`（
  /// `get_type_pack::get::<T>(tp)` 会解引用它）。返回值要么为 null（变体不匹配），
  /// 要么为对该 arena 节点的只读借用，在 `tp` 所借用的 arena 存活期内有效。
  unsafe fn get_through(tp: TypePackId) -> *const T {
    get_type_pack::get::<T>(tp).map_or(null(), |r| r as *const T)
  }
}

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
/// C++ `template<typename T, typename Ty> const T* get(std::optional<Ty> ty)`.
pub unsafe fn get_optional_ty<T: GetThroughId<Ty>, Ty: Copy>(ty: Option<Ty>) -> *const T {
  // Safety: 本函数自身为 `unsafe fn`，其前置条件即 C++ 原实现要求的"Some(ty) 为有效 id 指针"，
  // 由 unsafe 边界从调用方继承；None 分支直接返回 null，仅 Some 时才 `T::get_through(ty)`。
  // 按仓库不变量，TypeId/TypePackId 是类型 arena 裸指针（bump 分配、地址稳定），两个 impl 的
  // get_through 只经 get_type_id/get_type_pack_id 按变体 tag 下转：命中返回随该 arena 存活的
  // 只读借用、否则 null，无越权解引用，单线程只读无别名冲突。
  unsafe {
    if let Some(ty) = ty {
      T::get_through(ty)
    } else {
      null()
    }
  }
}
