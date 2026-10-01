use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::{
  records::{arena_handle::alias, type_pack_var::TypePackVar},
  type_aliases::type_pack_variant::TypePackVariant,
};

LUAU_NOINLINE! {
/// # Safety
/// `ty` 须指向 type arena 中存活、地址不移动的 `TypePackVar`（非空、对齐）；本函数就地写入
/// `variant` 并返回其内部字段的裸指针，故调用方须保证对该节点的独占可变访问，且返回指针随 arena
/// 比持有者长寿。单线程、无并发写别名。对应 C++ `T* emplaceTypePack(TypePackVar* ty, ...)`
/// (`cpp/Analysis/include/Luau/TypePack.h:255`)。
    pub(crate) unsafe fn emplace_type_pack(ty: *mut TypePackVar, variant: TypePackVariant) -> *mut TypePackVariant {
        let ty_ref = alias(ty);
        ty_ref.ty = variant;
        &mut alias(ty).ty
    }
}
