/// C++ `GeneralizationResult<TypePackId> generalizeTypePack(...)`
/// (Generalization.cpp:839-868). Generalize one type pack.
use crate::enums::polarity::Polarity;
use crate::{
  functions::{
    follow_type_pack::follow_type_pack_id, fresh_index::fresh_index,
    get_type_pack::get_type_pack_id, subsumes_scope::subsumes,
  },
  records::{
    builtin_types::BuiltinTypes, free_type_pack::FreeTypePack,
    generalization_params::GeneralizationParams, generalization_result::GeneralizationResult,
    generic_type_pack::GenericTypePack, scope::Scope, type_arena::TypeArena,
    type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn generalize_type_pack(
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  scope: *mut Scope,
  tp: TypePackId,
  params: &GeneralizationParams,
) -> GeneralizationResult {
  let tp = unsafe { follow_type_pack_id(tp) };

  // SAFETY: tp 指向类型 arena 中的 TypePackVar
  if unsafe { (*tp).owning_arena } != arena {
    return not_replaced(tp);
  }

  let Some(ftp) = get_type_pack_id::<FreeTypePack>(tp) else {
    return not_replaced(tp);
  };

  if !subsumes(scope, ftp.scope) {
    return not_replaced(tp);
  }

  if params.use_count == 1 {
    // emplaceTypePack<BoundTypePack>(asMutable(tp), builtinTypes->unknown_type_pack)
    // SAFETY: tp 指向类型 arena 中的 TypePackVar；builtin_types 指向常驻的 BuiltinTypes
    unsafe {
      (*(tp as *mut TypePackVar)).ty = TypePackVariant::Bound((*builtin_types).unknown_type_pack);
    }
  } else {
    // emplaceTypePack<GenericTypePack>(asMutable(tp), scope, params.polarity)
    emplace_generic_pack(tp, scope, params.polarity);
    return GeneralizationResult {
      // `GeneralizationResult` is monomorphized to `TypeId` in this port;
      // the pack id is stored via a pointer cast (the driver only checks
      // `result.is_some()` / `was_replaced_by_generic` and forwards the
      // original `freePack`, never dereferencing `result`).
      result: Some(tp as TypeId),
      was_replaced_by_generic: true,
      resource_limits_exceeded: false,
    };
  }

  not_replaced(tp)
}

#[inline]
fn not_replaced(tp: TypePackId) -> GeneralizationResult {
  GeneralizationResult {
    result: Some(tp as TypeId),
    was_replaced_by_generic: false,
    resource_limits_exceeded: false,
  }
}

/// C++ `emplaceTypePack<GenericTypePack>(asMutable(tp), scope, polarity)`:
/// matches `GenericTypePack{scope, polarity}`.
fn emplace_generic_pack(tp: TypePackId, scope: *mut Scope, polarity: Polarity) {
  let gtp = GenericTypePack {
    index: fresh_index(),
    level: Default::default(),
    scope,
    name: Default::default(),
    explicit_name: false,
    polarity,
  };
  unsafe {
    (*(tp as *mut TypePackVar)).ty = TypePackVariant::Generic(gtp);
  }
}
