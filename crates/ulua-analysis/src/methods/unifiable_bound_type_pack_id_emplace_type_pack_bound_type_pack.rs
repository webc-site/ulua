//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:461:emplaceTypePack`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:461-466, hand-ported)

use core::ptr::{from_mut, null_mut};
use std::ptr::eq;

use ulua_common::macros::{luau_assert::LUAU_ASSERT, luau_noinline::LUAU_NOINLINE};

use crate::{
  functions::{
    follow_type_pack::follow_type_pack_id, get_mutable_type_pack::get_mutable_type_pack_id,
  },
  records::{bound::Bound, type_pack_var::TypePackVar},
  type_aliases::{
    bound_type_pack::BoundTypePack, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};
LUAU_NOINLINE! {
/// # Safety
/// 调用方须保证 `ty` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
    pub unsafe fn emplace_type_pack(ty: *mut TypePackVar, ty_arg: &mut TypePackId) -> *mut Bound<TypePackId> {
        unsafe {
            LUAU_ASSERT!(!eq(ty, follow_type_pack_id(*ty_arg)));
            // ty->ty.emplace<BoundTypePack>(tyArg)
            (*ty).ty = TypePackVariant::Bound(*ty_arg);
            get_mutable_type_pack_id::<BoundTypePack>(ty as *const TypePackVar as TypePackId)
              .map_or(null_mut(), from_mut)
        }
    }
}
