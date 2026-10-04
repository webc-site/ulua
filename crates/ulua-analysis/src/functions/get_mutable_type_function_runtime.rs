use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::arena_handle::alias_opt_mut,
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

// Source: `Analysis/include/Luau/TypeFunctionRuntime.h:167-173` (hand-ported)
/// C++ `template<typename T> T* getMutable(TypeFunctionTypePackId tv)`：按变体 tag
/// 探测 pack 节点负载并回其可变借用，未命中（含空句柄这一契约违例的确定性兜底）以
/// `None` 表达。与只读侧 [`get_type_function_type_pack_id`][crate::functions::get_type_function_runtime::get_type_function_type_pack_id]
/// 同构，可变借用形如 `Handle::get_mut<'a>`：存活/单线程独占改写前提由
/// `records/arena_handle.rs` 模块头契约承载（type_pack_arena 为 chunked bump 分配，
/// 元素地址随 arena 存活期不迁移），解引用与可变性经 `alias_opt_mut` 门面收口，
/// `None` 分支不产生任何借用。
pub(crate) fn get_mutable_type_function_type_pack_id<'a, T: TypeFunctionTypePackVariantMember>(
  tv: TypeFunctionTypePackId,
) -> Option<&'a mut T> {
  LUAU_ASSERT!(!tv.is_null());
  T::get_if_mut(&mut alias_opt_mut(tv.cast_mut())?.type_variant)
}

// Source: `Analysis/include/Luau/TypeFunctionRuntime.h:275-281` (hand-ported)
/// C++ `template<typename T> T* getMutable(TypeFunctionTypeId tv)`：[`get_mutable_type_function_type_pack_id`]
/// 的 type 侧对偶，节点属 `type_arena`，前提同该函数头。
pub(crate) fn get_mutable_type_function_type_id<'a, T: TypeFunctionTypeVariantMember>(
  tv: TypeFunctionTypeId,
) -> Option<&'a mut T> {
  LUAU_ASSERT!(!tv.is_null());
  T::get_if_mut(&mut alias_opt_mut(tv.cast_mut())?.type_variant)
}
