//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:461-466, hand-ported)

use core::ptr::{from_mut, null_mut};
use std::ptr::eq;

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_noinline::LUAU_NOINLINE};

use crate::{
  functions::{follow_type_pack, get_mutable_type_pack},
  records::{bound::Bound, type_pack_var::TypePackVar},
  type_aliases::{
    bound_type_pack::BoundTypePack, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};
LUAU_NOINLINE! {
/// # Safety
/// 调用方须保证 `ty` 非空、对齐，指向存活且可独占写的 `TypePackVar`（调用方经 `asMutable` 取得），
/// 且 `*ty_arg` 经 follow 后不等于 `ty`（函数内 LUAU_ASSERT 校验，否则产生自指环）。
/// 原地覆写 `ty->ty` 为 `BoundTypePack` 后据 RTTI 取回内嵌 `Bound` 可变指针，本 log/arena 独占、单线程。
/// cpp `Analysis/src/TypePack.cpp:485`（`Unifiable::Bound<TypePackId>* emplaceTypePack<BoundTypePack>(TypePackVar*, TypePackId&)`）。
    pub unsafe fn emplace_type_pack(ty: *mut TypePackVar, ty_arg: &mut TypePackId) -> *mut Bound<TypePackId> {
        unsafe {
            LUAU_ASSERT!(!eq(ty, follow_type_pack::follow(*ty_arg)));
            // ty->ty.emplace<BoundTypePack>(tyArg)
            (*ty).ty = TypePackVariant::Bound(*ty_arg);
            get_mutable_type_pack::get_mutable::<BoundTypePack>(ty as *const TypePackVar as TypePackId)
              .map_or(null_mut(), from_mut)
        }
    }
}
