//! Source: `Analysis/include/Luau/Type.h:252-259` (hand-ported)
//!
//! The body lives in `get_singleton_type.rs` (the signature-pinned overload
//! name callers are instructed to use); this node re-exports it.

use alloc::string::String;
use core::any::TypeId as CoreTypeId;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::{
  bound_type::BoundType,
  type_id::TypeId,
  type_variant::{TypeVariant, TypeVariantMember},
};

/// arena 节点的整变体只读视图（C++ `Type& get(TypeId)` 的直译）。
///
/// [`get`] 按变体取子字段，但遍历器的分派需要整个 [`TypeVariant`]。把
/// `&(*tv).ty` 的裸指针解引用统一收拢在此处（与 [`get_impl`] 同一 arena 节点
/// 有效性契约），业务侧（`IterativeTypeVisitor` 等）恢复为普通引用读取/匹配，
/// 不再散落 `unsafe { (*id).ty }`。
///
/// # Safety 说明（类型级契约，同 [`get`]）
/// `tv` 的有效性由调用方按 C++ `get(TypeId)` 同契约保证：非空、指向类型 arena
/// 内地址稳定的 `Type` 节点，且在被返回引用使用期间不失效；此处仅解引用一次。
/// 收口为 crate 内门面（同 `clone_clone::type_is_persistent` 写法），公开 API
/// 不接收裸指针解引用。
pub(crate) fn type_variant_of(tv: TypeId) -> &'static TypeVariant {
  LUAU_ASSERT!(!tv.is_null());
  // SAFETY: tv 的有效性由调用方按 C++ 同契约保证；此处仅解引用一次。
  unsafe { &(*tv).ty }
}

/// arena 节点 `persistent` 标志只读视图（C++ `ty->persistent` 的收口）。
///
/// 与 [`type_variant_of`] 同一 arena 节点有效性契约，业务侧不再散落
/// `unsafe { (*ty).persistent }`。
pub(crate) fn is_persistent(tv: TypeId) -> bool {
  // SAFETY: 同 [`type_variant_of`]，契约由调用方保证。
  unsafe { (*tv).persistent }
}

/// arena 节点 `documentation_symbol` 只读克隆（C++ `ty->documentationSymbol` 的收口）。
///
/// 与 [`type_variant_of`] 同一 arena 节点有效性契约。
pub(crate) fn documentation_symbol_of(tv: TypeId) -> Option<String> {
  // SAFETY: 同 [`type_variant_of`]，契约由调用方保证。
  unsafe { (*tv).documentation_symbol.clone() }
}

pub fn get<T: TypeVariantMember + 'static>(tv: TypeId) -> Option<&'static T> {
  get_impl::<T>(tv)
}

/// 私有实现：变体判别收口在 [`type_variant_of`] 一处（C++ Type.h:1089 get 的直译）。
fn get_impl<T: TypeVariantMember + 'static>(tv: TypeId) -> Option<&'static T> {
  let ty = type_variant_of(tv);

  if CoreTypeId::of::<T>() != CoreTypeId::of::<BoundType>() {
    LUAU_ASSERT!(BoundType::get_if(ty).is_none());
  }

  T::get_if(ty)
}
