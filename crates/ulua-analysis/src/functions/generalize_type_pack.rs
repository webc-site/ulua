use core::ptr::from_ref;

/// C++ `GeneralizationResult<TypePackId> generalizeTypePack(...)`
/// (Generalization.cpp:839-868). Generalize one type pack.
use crate::enums::polarity::Polarity;
use crate::{
  functions::{
    follow_type_pack, fresh_index::fresh_index, get_type_pack, subsumes_scope::subsumes,
  },
  records::{
    arena_handle::{Handle, alias, alias_opt, alias_ref},
    builtin_types::BuiltinTypes,
    free_type_pack::FreeTypePack,
    generalization_params::GeneralizationParams,
    generalization_result::GeneralizationResult,
    generic_type_pack::GenericTypePack,
    scope::Scope,
    type_arena::TypeArena,
    type_pack_var::TypePackVar,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_pack_variant::TypePackVariant},
};
/// 形参全为受检类型/引用（arena/builtin_types 为句柄、`scope` 为非空
/// `&Scope`、`tp` 为 arena 包句柄），契约由类型承载；函数体内无 `unsafe`。
pub fn generalize_type_pack(
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  scope: &Scope,
  tp: TypePackId,
  params: &GeneralizationParams,
) -> GeneralizationResult {
  let tp = follow_type_pack::follow(tp);

  if alias_ref(tp).owning_arena != arena.get().arena_id {
    return not_replaced(tp);
  }

  let Some(ftp) = get_type_pack::get::<FreeTypePack>(tp) else {
    return not_replaced(tp);
  };

  if !subsumes(Some(scope), alias_opt(ftp.scope)) {
    return not_replaced(tp);
  }

  if params.use_count == 1 {
    // emplaceTypePack<BoundTypePack>(asMutable(tp), builtinTypes->unknown_type_pack)
    alias(tp as *mut TypePackVar).ty =
      TypePackVariant::Bound(builtin_types.get().unknown_type_pack);
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
fn emplace_generic_pack(tp: TypePackId, scope: &Scope, polarity: Polarity) {
  // 边界收口：GenericTypePack 记录的 `scope` 字段保持裸指针布局，入口引用在此还原。
  let scope = from_ref(scope).cast_mut();
  let gtp = GenericTypePack {
    index: fresh_index(),
    level: Default::default(),
    scope,
    name: Default::default(),
    explicit_name: false,
    polarity,
  };
  alias(tp as *mut TypePackVar).ty = TypePackVariant::Generic(gtp);
}
