//! Faithful port of `TypeFunctionCloner::shallowClone(TypeFunctionTypePackId tp)`
//! (Analysis/src/TypeFunctionRuntime.cpp:2715-2734).
use alloc::vec::Vec;
use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id,
  records::{
    type_function_cloner::TypeFunctionCloner,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_type_pack_var::TypeFunctionTypePackVar,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    type_function_kind::TypeFunctionKind, type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariant,
  },
};
impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn shallow_clone_type_function_type_pack_id(
    &mut self,
    tp: TypeFunctionTypePackId,
  ) -> TypeFunctionTypePackId {
    // if (auto it = find(tp))
    //     return *it;
    if let Some(it) = self.find_type_function_type_pack_id(tp) {
      return it;
    }

    unsafe {
      let runtime = self.type_function_runtime;

      // Create a shallow serialization
      // TypeFunctionTypePackId target = {};
      let mut target: TypeFunctionTypePackId = null();

      if !get_type_function_type_pack_id::<TypeFunctionTypePack>(tp).is_null() {
        target = (*runtime)
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
              head: Vec::new(),
              tail: None,
            }),
          ));
      } else if !get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp).is_null() {
        target = (*runtime)
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V1(TypeFunctionVariadicTypePack { type_id: null() }),
          ));
      } else if let Some(g_pack) =
        get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp).as_ref()
      {
        target = (*runtime)
          .type_pack_arena
          .allocate(TypeFunctionTypePackVar::new(
            TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
              is_named: g_pack.is_named,
              name: g_pack.name.clone(),
            }),
          ));
      } else {
        LUAU_ASSERT!(false /* "Unknown type" */);
      }

      // packs[tp] = target;
      *self.packs.get_or_insert(tp) = target;
      // queue.emplace_back(tp, target);
      self
        .queue
        .push((TypeFunctionKind::V1(tp), TypeFunctionKind::V1(target)));
      target
    }
  }
}
