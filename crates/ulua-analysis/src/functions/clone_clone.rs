use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use crate::{
  records::{
    arena_handle::Handle, arena_id::ArenaId, clone_state::CloneState, type_arena::TypeArena,
    type_cloner::TypeCloner, type_fun::TypeFun,
  },
  type_aliases::{
    collections::HashMap, seen_type_packs_clone::SeenTypePacks as DenseSeenTypePacks,
    seen_types_clone::SeenTypes as DenseSeenTypes, type_id::TypeId, type_pack_id::TypePackId,
  },
};

/// `Type.owning_arena` 只读探针（cpp `getOwningArena(ty)`）：解收口于此，业务
/// 逻辑经返回值比较 arena 身份，不再直接 `(*ty).owning_arena`。
///
/// # Safety
/// ty 为存活对齐的 arena Type 节点句柄，此处只 Copy 其 `owning_arena`
/// （[`ArenaId`] 值身份），不构造长期引用。
pub(crate) fn type_owning_arena(ty: TypeId) -> ArenaId {
  // SAFETY: ty 为存活对齐的 arena Type 节点句柄（与 type_is_persistent 同契约）。
  unsafe { (*ty).owning_arena }
}

/// cpp `isPersistent(ty) || getOwningArena(ty) != arena`：判断类型是否属于
/// 其它 arena 或持久类型（不应由本 arena 就地改写）。
pub(crate) fn type_is_foreign_or_persistent(ty: TypeId, arena: ArenaId) -> bool {
  type_is_persistent(ty) || type_owning_arena(ty) != arena
}

/// `TypePackId`/`TypeId` 的 `persistent` 只读探针（cpp `tp->persistent`）。
///
/// arena 句柄解引用收口在这两个 `pub(crate)` 函数内，令克隆入口对外恢复 safe
/// 签名——与 `get_type::get_impl` 同一写法；句柄有效性仍由调用方按 C++ 同契约保证。
pub(crate) fn pack_is_persistent(tp: TypePackId) -> bool {
  // Safety: tp 为本次克隆所用 type arena（bump 块、地址不移动）中存活对齐的
  // TypePackVar 句柄，此处只拷贝其 `persistent` bool 字段，不构造长期引用。
  unsafe { (*tp).persistent }
}

pub(crate) fn type_is_persistent(ty: TypeId) -> bool {
  // Safety: 同上，ty 为存活对齐的 arena Type 节点句柄，只拷贝 `persistent` bool。
  unsafe { (*ty).persistent }
}

/// 对应 C++ `clone(TypePackId, TypeArena&, CloneState&)`（Clone.cpp）：持久 pack
/// 直接复用，否则按 `clone_state` 的 seen 映射克隆进 `dest`。
///
/// `tp` 为 arena 句柄（同 `follow_type_id`/`get_type_id` 的既有纪律）：调用方须保证
/// 它指向本次克隆所用 type arena 的存活节点；`dest`/`clone_state` 是独占借用，
/// 其 `builtin_types` 句柄目标须比返回的 pack 句柄长寿。
pub fn clone(tp: TypePackId, dest: &mut TypeArena, clone_state: &mut CloneState) -> TypePackId {
  if pack_is_persistent(tp) {
    return tp;
  }

  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = TypeCloner {
        arena: Handle::from_mut(dest),
        builtin_types,
        queue: Vec::new(),
        types: tys as *mut HashMap<TypeId, TypeId>,
        packs: tps as *mut HashMap<TypePackId, TypePackId>,
        force_ty: null(),
        force_tp: null(),
        steps: 0,
        replacement_for_null_scope: null_mut(),
        skip_lazy_type_clone: false,
      };
      cloner.clone_type_pack_id(tp)
    },
  )
}

pub(crate) fn with_clone_maps<R>(
  seen_types: &mut DenseSeenTypes,
  seen_type_packs: &mut DenseSeenTypePacks,
  body: impl FnOnce(&mut HashMap<TypeId, TypeId>, &mut HashMap<TypePackId, TypePackId>) -> R,
) -> R {
  let mut tys: HashMap<TypeId, TypeId> = seen_types.iter().map(|(k, v)| (*k, *v)).collect();
  let mut tps: HashMap<TypePackId, TypePackId> =
    seen_type_packs.iter().map(|(k, v)| (*k, *v)).collect();

  let result = body(&mut tys, &mut tps);

  for (k, v) in tys.iter() {
    *seen_types.get_or_insert(*k) = *v;
  }
  for (k, v) in tps.iter() {
    *seen_type_packs.get_or_insert(*k) = *v;
  }

  result
}

/// 对应 C++ `clone(TypeId, TypeArena&, CloneState&)`（Clone.cpp）：持久类型直接复用，
/// 否则按 `clone_state` 的 seen 映射克隆进 `dest`。
///
/// 前提同 [`clone`]：`type_id` 须指向本次克隆所用 type arena 的存活节点。
pub fn clone_type_id(
  type_id: TypeId,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
) -> TypeId {
  if type_is_persistent(type_id) {
    return type_id;
  }

  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = TypeCloner {
        arena: Handle::from_mut(dest),
        builtin_types,
        queue: Vec::new(),
        types: tys as *mut HashMap<TypeId, TypeId>,
        packs: tps as *mut HashMap<TypePackId, TypePackId>,
        force_ty: null(),
        force_tp: null(),
        steps: 0,
        replacement_for_null_scope: null_mut(),
        skip_lazy_type_clone: false,
      };
      cloner.clone_type_id(type_id)
    },
  )
}

pub fn clone_type_fun(
  type_fun: &TypeFun,
  dest: &mut TypeArena,
  clone_state: &mut CloneState,
) -> TypeFun {
  let builtin_types = clone_state.builtin_types;
  with_clone_maps(
    &mut clone_state.seen_types,
    &mut clone_state.seen_type_packs,
    |tys, tps| {
      let mut cloner = TypeCloner {
        arena: Handle::from_mut(dest),
        builtin_types,
        queue: Vec::new(),
        types: tys as *mut HashMap<TypeId, TypeId>,
        packs: tps as *mut HashMap<TypePackId, TypePackId>,
        force_ty: null(),
        force_tp: null(),
        steps: 0,
        replacement_for_null_scope: null_mut(),
        skip_lazy_type_clone: false,
      };

      let mut copy = type_fun.clone();

      for param in copy.type_params.iter_mut() {
        param.ty = cloner.clone_type_id(param.ty);

        if let Some(default_value) = param.default_value {
          param.default_value = Some(cloner.clone_type_id(default_value));
        }
      }

      for param in copy.type_pack_params.iter_mut() {
        param.tp = cloner.clone_type_pack_id(param.tp);

        if let Some(default_value) = param.default_value {
          param.default_value = Some(cloner.clone_type_pack_id(default_value));
        }
      }

      copy.r#type = cloner.clone_type_id(copy.r#type);

      copy
    },
  )
}
