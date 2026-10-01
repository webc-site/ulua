//! `unifiable` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::{from_mut, null_mut};
use std::ptr::eq;

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_noinline::LUAU_NOINLINE};

use crate::{
  functions::{follow_type, follow_type_pack, get_mutable_type, get_mutable_type_pack},
  records::{arena_handle::alias, bound::Bound, r#type::Type, type_pack_var::TypePackVar},
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};

LUAU_NOINLINE! {
    pub fn unifiable_bound_type_id_emplace_type_bound_type(
        ty: &mut Type,
        ty_arg: &mut TypeId,
    ) -> *mut Bound<TypeId> {

            LUAU_ASSERT!(!eq(ty, follow_type::follow(*ty_arg)));
            // ty->ty.emplace<BoundType>(tyArg)
            ty.ty = TypeVariant::Bound(*ty_arg);
            get_mutable_type::get_mutable::<BoundType>(ty as *const Type as TypeId)
              .map_or(null_mut(), from_mut)

    }
}

// Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:461-466, hand-ported)

LUAU_NOINLINE! {
/// # Safety
/// 调用方须保证 `ty` 非空、对齐，指向存活且可独占写的 `TypePackVar`（调用方经 `asMutable` 取得），
/// 且 `*ty_arg` 经 follow 后不等于 `ty`（函数内 LUAU_ASSERT 校验，否则产生自指环）。
/// 原地覆写 `ty->ty` 为 `BoundTypePack` 后据 RTTI 取回内嵌 `Bound` 可变指针，本 log/arena 独占、单线程。
/// cpp `Analysis/src/TypePack.cpp:485`（`Unifiable::Bound<TypePackId>* emplaceTypePack<BoundTypePack>(TypePackVar*, TypePackId&)`）。
    pub(crate) unsafe fn emplace_type_pack(ty: *mut TypePackVar, ty_arg: &mut TypePackId) -> *mut Bound<TypePackId> {
            LUAU_ASSERT!(!eq(ty, follow_type_pack::follow(*ty_arg)));
            // ty->ty.emplace<BoundTypePack>(tyArg)
            alias(ty).ty = TypePackVariant::Bound(*ty_arg);
            get_mutable_type_pack::get_mutable::<BoundTypePack>(ty as *const TypePackVar as TypePackId)
              .map_or(null_mut(), from_mut)
    }
}
