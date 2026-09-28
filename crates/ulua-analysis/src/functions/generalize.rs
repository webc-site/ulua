use core::ptr::from_ref;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    collapse_free_type_cycles::collapse_free_type_cycles, follow_type, follow_type_pack,
    generalize_type::generalize_type, generalize_type_pack::generalize_type_pack, get_mutable_type,
    get_type, seal_table::seal_table,
  },
  methods::{
    free_type_searcher_visit_generalization::searcher_traverse_type_id,
    type_cacher_visit_generalization::cacher_traverse_type_id,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, free_type::FreeType,
    free_type_searcher::FreeTypeSearcher, function_type::FunctionType,
    generalization_result::GeneralizationResult, scope::Scope, type_arena::TypeArena,
    type_cacher::TypeCacher,
  },
  type_aliases::type_id::TypeId,
};
pub fn generalize(
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  scope: &Scope,
  cached_types: &mut DenseHashSet<TypeId>,
  ty: TypeId,
  generalization_target: Option<TypeId>,
) -> Option<TypeId> {
  generalize_impl(
    arena,
    builtin_types,
    scope,
    cached_types,
    ty,
    generalization_target,
  )
}

// 内部实现：arena/builtin_types 为单例句柄（Handle），scope/cached_types 裸指针
// 解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
fn generalize_impl(
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  scope: &Scope,
  cached_types: &mut DenseHashSet<TypeId>,
  ty: TypeId,
  generalization_target: Option<TypeId>,
) -> Option<TypeId> {
  let mut ty = follow_type::follow(ty);

  // Safety: `follow_type_id` 返回已折叠到实际节点的存活 `TypeId`（非空、指向类型 arena 中的
  // `Type`），此处仅取共享引用读取 `owning_arena`/`persistent` 字段；单线程泛化，无 &mut 别名。
  let ty_ptr = unsafe { &*ty };
  if ty_ptr.owning_arena != arena.get().arena_id || ty_ptr.persistent {
    return Some(ty);
  }

  // 边界收口：FreeTypeSearcher 记录的 `scope` 字段保持裸指针布局，入口引用在此还原。
  let mut fts = FreeTypeSearcher::new(from_ref(scope).cast_mut(), cached_types);
  searcher_traverse_type_id(&mut fts, ty);

  // 批量预处理（cpp Generalization.cpp:1467/1549）：泛化前沿中的直接界环先
  // 塌缩，避免环成员被 generalize_type 的防自指逻辑错误保留为泛型。带
  // generalization_target 的逐点泛化不跑批扫描（cpp 同形）；Rust 侧亦未移植
  // cpp generalizeType 的逐点塌缩，见 generalize_type 头部 DEVIATION 注释。
  if generalization_target.is_none() {
    collapse_free_type_cycles(arena, builtin_types, &fts.types);
    // 塌缩可能把根自身绑定到代表（或经代表落到具体界），使用前重新收敛。
    ty = follow_type::follow(ty);
  }

  let mut function_ty = get_mutable_type::get_mutable::<FunctionType>(ty);

  for (free_ty, params) in &fts.types {
    if generalization_target.is_none_or(|target| *free_ty == target) {
      // cpp `generalizeJustOne`（Generalization.cpp:1471/1553）：前沿项若已被
      // 绑定到非自由类型（塌缩预处理或求解器就地绑定所致），跳过不再泛化。
      if get_type::get::<FreeType>(follow_type::follow(*free_ty)).is_none() {
        continue;
      }

      // 形参全为受检句柄/引用，安全调用。
      let res: GeneralizationResult =
        generalize_type(arena, builtin_types, scope, *free_ty, params);

      if res.resource_limits_exceeded {
        return None;
      }

      if res.was_replaced_by_generic
        && let Some(result) = res.result
        && let Some(ftv) = function_ty.as_deref_mut()
      {
        ftv.generics.push(result);
      }
    }
  }

  let unsealed_tables_iter = fts.unsealed_tables.iter();
  for unsealed_table_ty in unsealed_tables_iter {
    if generalization_target.is_none_or(|target| *unsealed_table_ty == target) {
      seal_table(scope, *unsealed_table_ty);
    }
  }

  for (free_pack_id, params) in &fts.type_packs {
    let free_pack = follow_type_pack::follow(*free_pack_id);
    if generalization_target.is_none() {
      // 形参全为受检句柄/引用，安全调用。
      let generalized_tp: GeneralizationResult =
        generalize_type_pack(arena, builtin_types, scope, free_pack, params);

      if generalized_tp.resource_limits_exceeded {
        return None;
      }

      if generalized_tp.was_replaced_by_generic
        && generalized_tp.result.is_some()
        && let Some(ftv) = function_ty.as_deref_mut()
      {
        ftv.generic_packs.push(free_pack);
      }
    }
  }

  let mut cacher = TypeCacher::new(cached_types);
  cacher_traverse_type_id(&mut cacher, ty);

  Some(ty)
}
