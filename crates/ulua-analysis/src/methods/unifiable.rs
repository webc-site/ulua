//! `unifiable` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use std::ptr::eq;

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_noinline::LUAU_NOINLINE};

use crate::{
  functions::{follow_type, follow_type_pack},
  records::{arena_handle::alias, r#type::Type, type_pack_var::TypePackVar},
  type_aliases::{
    type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
    type_variant::TypeVariant,
  },
};

LUAU_NOINLINE! {
/// cpp `Unifiable::emplaceType<BoundType>`（Type.cpp）：原地把 arena 节点覆写为
/// `Bound(ty_arg)`。cpp 尾部 `getMutable<BoundType>(ty)` 仅为回传内嵌指针，全仓
/// 调用点均丢弃该返回值，属死返回面，按 review.md §7 移除（覆写语义不变）。
    pub fn unifiable_bound_type_id_emplace_type_bound_type(
        ty: &mut Type,
        ty_arg: &mut TypeId,
    ) {

            LUAU_ASSERT!(!eq(ty, follow_type::follow(*ty_arg)));
            // ty->ty.emplace<BoundType>(tyArg)
            ty.ty = TypeVariant::Bound(*ty_arg);

    }
}

// Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:461-466, hand-ported)

LUAU_NOINLINE! {
/// 前置：调用方须保证 `ty` 非空、对齐，指向存活且可独占写的 `TypePackVar`（调用方经 `asMutable` 取得），
/// 且 `*ty_arg` 经 follow 后不等于 `ty`（函数内 LUAU_ASSERT 校验，否则产生自指环）。
/// 原地覆写 `ty->ty` 为 `BoundTypePack`，本 log/arena 独占、单线程。裸指针转借用
/// 由 `arena_handle::alias` 的模块级契约承担，故本函数自身无 unsafe 操作。
/// cpp `Analysis/src/TypePack.cpp:485`（`Unifiable::Bound<TypePackId>* emplaceTypePack(TypePackVar*, TypePackId&)`）；
/// cpp 尾部回传的 `getMutable<BoundTypePack>` 全仓无人消费，按 review.md §7 移除。
    pub(crate) fn emplace_type_pack(ty: *mut TypePackVar, ty_arg: &mut TypePackId) {
            LUAU_ASSERT!(!eq(ty, follow_type_pack::follow(*ty_arg)));
            // ty->ty.emplace<BoundTypePack>(tyArg)
            alias(ty).ty = TypePackVariant::Bound(*ty_arg);
    }
}
