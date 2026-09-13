use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::{
  records::type_pack_var::TypePackVar, type_aliases::type_pack_variant::TypePackVariant,
};

LUAU_NOINLINE! {
/// # Safety
/// 调用方须保证 `ty` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
    pub unsafe fn emplace_type_pack(ty: *mut TypePackVar, variant: TypePackVariant) -> *mut TypePackVariant {
        unsafe {
            let ty_ref = &mut *ty;
            ty_ref.operator_assign_type_pack_variant(variant);
            &mut (*ty).ty
        }
    }
}
