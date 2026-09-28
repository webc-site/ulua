use core::ptr::null;

use crate::{
  functions::clone_clone::{pack_is_persistent, type_is_persistent, with_clone_maps},
  records::{
    arena_handle::Handle, binding::Binding, clone_state::CloneState,
    fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner, scope::Scope,
    type_arena::TypeArena, type_fun::TypeFun,
  },
  type_aliases::{collections::HashMap, type_id::TypeId, type_pack_id::TypePackId},
};

/// 对应 C++ `cloneIncremental(TypePackId, ...)`（Clone.cpp）：持久 pack 直接复用，
/// 否则用 `FragmentAutocompleteTypeCloner` 把 stale arena 的子图克隆进 `dest`。
///
/// `tp` 为 arena 句柄：须指向本次克隆所用 type arena 的存活节点（同 `clone_clone::clone`
/// 的纪律）；`fresh_scope_for_free_types` 是 free/lazy 类型落点的目标 scope，
/// 下游 `FragmentAutocompleteTypeCloner::new` 以 `LUAU_ASSERT!` 断言其非空——
/// `&mut` 形参已从类型上排除 null。
pub fn clone_incremental(
  tp: TypePackId,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
  fresh_scope_for_free_types: &mut Scope,
) -> TypePackId {
  if pack_is_persistent(tp) {
    return tp;
  }

  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = FragmentAutocompleteTypeCloner::new(
        Handle::from_mut(dest),
        builtin_types,
        tys as *mut HashMap<TypeId, TypeId>,
        tps as *mut HashMap<TypePackId, TypePackId>,
        null(),
        null(),
        fresh_scope_for_free_types as *mut Scope,
      );
      cloner.base.clone_type_pack_id(tp)
    },
  )
}

/// 对应 C++ `cloneIncremental(TypeId, ...)`（Clone.cpp），前提同 [`clone_incremental`]。
pub fn clone_incremental_type_id(
  type_id: TypeId,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
  fresh_scope_for_free_types: &mut Scope,
) -> TypeId {
  if type_is_persistent(type_id) {
    return type_id;
  }

  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = FragmentAutocompleteTypeCloner::new(
        Handle::from_mut(dest),
        builtin_types,
        tys as *mut HashMap<TypeId, TypeId>,
        tps as *mut HashMap<TypePackId, TypePackId>,
        null(),
        null(),
        fresh_scope_for_free_types as *mut Scope,
      );
      cloner.base.clone_type_id(type_id)
    },
  )
}

/// 对应 C++ `cloneIncremental(TypeFun, ...)`（Clone.cpp），参数含义同上。
pub fn clone_incremental_type_fun(
  type_fun: &TypeFun,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
  fresh_scope_for_free_types: &mut Scope,
) -> TypeFun {
  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = FragmentAutocompleteTypeCloner::new(
        Handle::from_mut(dest),
        builtin_types,
        tys as *mut HashMap<TypeId, TypeId>,
        tps as *mut HashMap<TypePackId, TypePackId>,
        null(),
        null(),
        fresh_scope_for_free_types as *mut Scope,
      );

      let mut copy = type_fun.clone();

      for param in copy.type_params.iter_mut() {
        param.ty = cloner.base.clone_type_id(param.ty);

        if let Some(default_value) = param.default_value {
          param.default_value = Some(cloner.base.clone_type_id(default_value));
        }
      }

      for param in copy.type_pack_params.iter_mut() {
        param.tp = cloner.base.clone_type_pack_id(param.tp);

        if let Some(default_value) = param.default_value {
          param.default_value = Some(cloner.base.clone_type_pack_id(default_value));
        }
      }

      copy.r#type = cloner.base.clone_type_id(copy.r#type);

      copy
    },
  )
}

/// 对应 C++ `cloneIncremental(Binding, ...)`（Clone.cpp），参数含义同上。
pub fn clone_incremental_binding(
  binding: &Binding,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
  fresh_scope_for_free_types: &mut Scope,
) -> Binding {
  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = FragmentAutocompleteTypeCloner::new(
        Handle::from_mut(dest),
        builtin_types,
        tys as *mut HashMap<TypeId, TypeId>,
        tps as *mut HashMap<TypePackId, TypePackId>,
        null(),
        null(),
        fresh_scope_for_free_types as *mut Scope,
      );

      Binding {
        deprecated: binding.deprecated,
        deprecated_suggestion: binding.deprecated_suggestion.clone(),
        documentation_symbol: binding.documentation_symbol.clone(),
        location: binding.location,
        // Clone.cpp:729: `binding.typeId->persistent ? binding.typeId : cloner.clone(binding.typeId)`.
        // `binding.type_id` 为 arena 存活句柄（调用方 cloneTypesFromFragment 传入的 binding
        // 均来自 stale 模块里已定型的 scope，且同处已用同一 type_id 调过
        // clone_incremental_type_id），故 persistent 探针读取合法。
        type_id: if type_is_persistent(binding.type_id) {
          binding.type_id
        } else {
          cloner.base.clone_type_id(binding.type_id)
        },
      }
    },
  )
}
