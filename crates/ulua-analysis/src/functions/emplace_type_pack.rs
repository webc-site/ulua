use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::{
  records::{arena_handle::alias, type_pack_var::TypePackVar},
  type_aliases::type_pack_variant::TypePackVariant,
};

LUAU_NOINLINE! {
/// 前置：`ty` 须指向 type arena 中存活、地址不移动的 `TypePackVar`（非空、对齐）；
/// 本函数就地写入 `variant` 并返回其内部字段的裸指针，故调用方须保证对该节点的独占
/// 可变访问，且返回指针随 arena 比持有者长寿。单线程、无并发写别名。该指针有效性
/// 契约由 `arena_handle::alias` 的模块级契约承担，故本函数自身无 unsafe 操作。
/// 对应 C++ `T* emplaceTypePack(TypePackVar* ty, ...)`
/// (`cpp/Analysis/include/Luau/TypePack.h:255`)。
    pub(crate) fn emplace_type_pack(ty: *mut TypePackVar, variant: TypePackVariant) -> *mut TypePackVariant {
        let ty_ref = alias(ty);
        ty_ref.ty = variant;
        &mut alias(ty).ty
    }
}
