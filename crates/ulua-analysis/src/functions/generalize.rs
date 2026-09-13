use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    generalize_type::generalize_type, generalize_type_pack::generalize_type_pack,
    get_mutable_type::get_mutable_type_id, seal_table::seal_table,
  },
  methods::{
    free_type_searcher_visit_generalization_alt_c::searcher_traverse_type_id,
    type_cacher_visit_generalization_alt_i::cacher_traverse_type_id,
  },
  records::{
    builtin_types::BuiltinTypes, free_type_searcher::FreeTypeSearcher, function_type::FunctionType,
    generalization_result::GeneralizationResult, scope::Scope, type_arena::TypeArena,
    type_cacher::TypeCacher,
  },
  type_aliases::type_id::TypeId,
};
pub fn generalize(
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  scope: *mut Scope,
  cached_types: *mut DenseHashSet<TypeId>,
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

// 内部实现：裸指针解引用由 unsafe 块承担（私有可见性，不触发签名契约告警）。
fn generalize_impl(
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  scope: *mut Scope,
  cached_types: *mut DenseHashSet<TypeId>,
  ty: TypeId,
  generalization_target: Option<TypeId>,
) -> Option<TypeId> {
  let ty = follow_type_id(ty);

  let ty_ptr = unsafe { &*ty };
  if ty_ptr.owning_arena != arena || ty_ptr.persistent {
    return Some(ty);
  }

  let mut fts = FreeTypeSearcher::new(scope, cached_types);
  searcher_traverse_type_id(&mut fts, ty);

  let mut function_ty = get_mutable_type_id::<FunctionType>(ty);

  for (free_ty, params) in &fts.types {
    if !generalization_target.is_some() || *free_ty == generalization_target.unwrap() {
      let res: GeneralizationResult =
        unsafe { generalize_type(arena, builtin_types, scope, *free_ty, params) };

      if res.resource_limits_exceeded {
        return None;
      }

      if res.was_replaced_by_generic
        && res.result.is_some()
        && let Some(ftv) = function_ty.as_deref_mut()
      {
        ftv.generics.push(res.result.unwrap());
      }
    }
  }

  let unsealed_tables_iter = fts.unsealed_tables.iter();
  for unsealed_table_ty in unsealed_tables_iter {
    if !generalization_target.is_some() || *unsealed_table_ty == generalization_target.unwrap() {
      seal_table(scope, *unsealed_table_ty);
    }
  }

  for (free_pack_id, params) in &fts.type_packs {
    let free_pack = unsafe { follow_type_pack_id(*free_pack_id) };
    if generalization_target.is_none() {
      let generalized_tp: GeneralizationResult =
        unsafe { generalize_type_pack(arena, builtin_types, scope, free_pack, params) };

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
