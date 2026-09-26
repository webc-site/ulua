/// C++ `GeneralizationResult<TypePackId> generalizeTypePack(...)`
/// (Generalization.cpp:839-868). Generalize one type pack.
use crate::enums::polarity::Polarity;
use crate::{
  functions::{
    follow_type_pack, fresh_index::fresh_index, get_type_pack, subsumes_scope::subsumes,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, free_type_pack::FreeTypePack,
    generalization_params::GeneralizationParams, generalization_result::GeneralizationResult,
    generic_type_pack::GenericTypePack, scope::Scope, type_arena::TypeArena,
    type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn generalize_type_pack(
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  scope: *mut Scope,
  tp: TypePackId,
  params: &GeneralizationParams,
) -> GeneralizationResult {
  let tp = follow_type_pack::follow(tp);

  // SAFETY: tp 指向类型 arena 中的 TypePackVar
  if unsafe { (*tp).owning_arena } != arena.get().arena_id {
    return not_replaced(tp);
  }

  let Some(ftp) = get_type_pack::get::<FreeTypePack>(tp) else {
    return not_replaced(tp);
  };

  if !subsumes(scope, ftp.scope) {
    return not_replaced(tp);
  }

  if params.use_count == 1 {
    // emplaceTypePack<BoundTypePack>(asMutable(tp), builtinTypes->unknown_type_pack)
    // SAFETY: tp 指向类型 arena 中的 TypePackVar
    unsafe {
      (*(tp as *mut TypePackVar)).ty =
        TypePackVariant::Bound(builtin_types.get().unknown_type_pack);
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
  // Safety: 调用方 generalize_type_pack 已通过 `(*tp).owning_arena ==
  // arena.get().arena_id` 确认
  // `tp` 属于本 arena 且为存活的 TypePackVar；TypePackId 与 TypePackVar 基址重合，
  // `tp as *mut TypePackVar` 得到的指针非空、对齐并指向该对象。arena 以 bump 方式
  // 分配、块地址不移动，单线程串行泛化中对该节点持有独占写权，故原地写 `.ty` 无别名冲突。
  unsafe {
    (*(tp as *mut TypePackVar)).ty = TypePackVariant::Generic(gtp);
  }
}
