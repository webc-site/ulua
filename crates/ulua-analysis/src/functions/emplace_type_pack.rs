use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::{
  records::type_pack_var::TypePackVar, type_aliases::type_pack_variant::TypePackVariant,
};

LUAU_NOINLINE! {
/// # Safety
/// `ty` 须指向 type arena 中存活、地址不移动的 `TypePackVar`（非空、对齐）；本函数就地写入
/// `variant` 并返回其内部字段的裸指针，故调用方须保证对该节点的独占可变访问，且返回指针随 arena
/// 比持有者长寿。单线程、无并发写别名。对应 C++ `T* emplaceTypePack(TypePackVar* ty, ...)`
/// (`cpp/Analysis/include/Luau/TypePack.h:255`)。
    pub unsafe fn emplace_type_pack(ty: *mut TypePackVar, variant: TypePackVariant) -> *mut TypePackVariant {
        // Safety: ty 为调用方持有的非空 TypePackVar 指针，指向 type arena（bump 块、地址
        // 不移动）中存活节点；两个 &mut 按语句先后创建——ty_ref 的独占借用在
        // operator_assign 返回后即结束，随后才取字段 &mut，同一对象上无重叠可变借用；
        // 返回的字段裸指针随 arena 比持有者长寿。
        unsafe {
            let ty_ref = &mut *ty;
            ty_ref.ty = variant;
            &mut (*ty).ty
        }
    }
}
