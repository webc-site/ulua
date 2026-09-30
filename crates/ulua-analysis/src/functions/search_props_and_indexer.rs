//! C++ `bool searchPropsAndIndexer(TypeId ty, TableType::Props tblProps,
//! std::optional<TableIndexer> tblIndexer, DenseHashSet<TypeId>& result,
//! NotNull<TypeFunctionContext> ctx)` (BuiltinTypeFunctions.cpp:1913). The 2nd
//! parameter is `Props` (a `BTreeMap<Name, Property>`), shared by both
//! `TableType` and `ExternType` callers — NOT a whole `TableType`.
use core::ptr::eq;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    follow_type, get_singleton_type::get_singleton_type, get_type, is_subtype_normalize::is_subtype,
  },
  records::{
    arena_handle::Handle, singleton_type::SingletonType, string_singleton::StringSingleton,
    table_indexer::TableIndexer, type_function::TypeFunction,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
  },
  type_aliases::{props_type::Props, type_id::TypeId},
};
/// 参数为句柄值/owned 快照/独占借用，签名无需 `unsafe`；体内对 ctx NonNull
/// 字段的引用重建收口在各自的 `unsafe {}` 内论证。
pub fn search_props_and_indexer(
  ty: TypeId,
  tbl_props: Props,
  tbl_indexer: Option<TableIndexer>,
  result: &mut DenseHashSet<TypeId>,
  ctx: &TypeFunctionContext,
) -> bool {
  let ty = follow_type::follow(ty);

  if let Some(singleton) = get_type::get::<SingletonType>(ty)
    && let Some(string_singleton) = get_singleton_type::<StringSingleton>(singleton)
    && let Some(prop) = tbl_props.get(&string_singleton.value)
  {
    let Some(prop_ty) = prop.read_ty.or(prop.write_ty) else {
      return false;
    };
    let prop_ty = follow_type::follow(prop_ty);

    if let Some(prop_union_ty) = get_type::get::<UnionType>(prop_ty) {
      for &option in &prop_union_ty.options {
        result.insert(follow_type::follow(option));
      }
    } else {
      result.insert(prop_ty);
    }

    return true;
  }

  if let Some(tbl_indexer) = tbl_indexer {
    let mut index_type = follow_type::follow(tbl_indexer.index_type);

    if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(index_type) {
      let index_func: *const TypeFunction =
        &unsafe { ctx.builtins.as_ref() }.type_functions.index_func;
      if eq(tfit.function.as_ptr(), index_func) {
        index_type = follow_type::follow(tbl_indexer.index_result_type);
      }
    }

    if is_subtype(
      ty,
      index_type,
      Handle::from_nonnull(ctx.arena),
      Handle::from_nonnull(ctx.builtins),
      // Safety: ctx.scope 为构造期接线的 NonNull<Scope>，非空且在调用期存活。
      unsafe { ctx.scope.as_ref() },
      Some(Handle::from_nonnull(ctx.normalizer)),
      Handle::from_nonnull(ctx.type_function_runtime),
      Handle::from_nonnull(ctx.ice),
    ) {
      let idx_result_ty = follow_type::follow(tbl_indexer.index_result_type);

      if let Some(idx_res_union_ty) = get_type::get::<UnionType>(idx_result_ty) {
        for &option in &idx_res_union_ty.options {
          result.insert(follow_type::follow(option));
        }
      } else {
        result.insert(idx_result_ty);
      }

      return true;
    }
  }

  false
}
