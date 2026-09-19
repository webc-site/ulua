//! C++ `bool searchPropsAndIndexer(TypeId ty, TableType::Props tblProps,
//! std::optional<TableIndexer> tblIndexer, DenseHashSet<TypeId>& result,
//! NotNull<TypeFunctionContext> ctx)` (BuiltinTypeFunctions.cpp:1913). The 2nd
//! parameter is `Props` (a `BTreeMap<Name, Property>`), shared by both
//! `TableType` and `ExternType` callers — NOT a whole `TableType`.
use core::ptr::{NonNull, eq};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    follow_type::follow_type_id, get_singleton_type::get_singleton_type,
    get_type_alt_j::get_type_id, is_subtype_normalize_alt_b::is_subtype,
  },
  records::{
    singleton_type::SingletonType, string_singleton::StringSingleton, table_indexer::TableIndexer,
    type_function::TypeFunction, type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
  },
  type_aliases::{props_type::Props, type_id::TypeId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn search_props_and_indexer(
  ty: TypeId,
  tbl_props: Props,
  tbl_indexer: Option<TableIndexer>,
  result: &mut DenseHashSet<TypeId>,
  ctx: *mut TypeFunctionContext,
) -> bool {
  // SAFETY: 调用链（BuiltinTypeFunctions 注册表 / tbl_index_into）保证 ctx
  // 非空且会话期有效。
  // SAFETY: 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效；
  // NonNull 封装解引用。
  let ctx_ref = unsafe { NonNull::new_unchecked(ctx).as_ref() };
  let ty = follow_type_id(ty);

  if let Some(singleton) = get_type_id::<SingletonType>(ty)
    && let Some(string_singleton) = get_singleton_type::<StringSingleton>(singleton)
    && let Some(prop) = tbl_props.get(&string_singleton.value)
  {
    let Some(prop_ty) = prop.read_ty.or(prop.write_ty) else {
      return false;
    };
    let prop_ty = follow_type_id(prop_ty);

    if let Some(prop_union_ty) = get_type_id::<UnionType>(prop_ty) {
      for &option in &prop_union_ty.options {
        result.insert(follow_type_id(option));
      }
    } else {
      result.insert(prop_ty);
    }

    return true;
  }

  if let Some(tbl_indexer) = tbl_indexer {
    let mut index_type = follow_type_id(tbl_indexer.index_type);

    if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(index_type) {
      let index_func: *const TypeFunction = &unsafe { ctx_ref.builtins.as_ref() }
        .type_functions
        .index_func;
      if eq(tfit.function.as_ptr(), index_func) {
        index_type = follow_type_id(tbl_indexer.index_result_type);
      }
    }

    if is_subtype(
      ty,
      index_type,
      ctx_ref.arena.as_ptr(),
      ctx_ref.builtins.as_ptr(),
      ctx_ref.scope.as_ptr(),
      ctx_ref.normalizer.as_ptr(),
      ctx_ref.type_function_runtime.as_ptr(),
      ctx_ref.ice.as_ptr(),
    ) {
      let idx_result_ty = follow_type_id(tbl_indexer.index_result_type);

      if let Some(idx_res_union_ty) = get_type_id::<UnionType>(idx_result_ty) {
        for &option in &idx_res_union_ty.options {
          result.insert(follow_type_id(option));
        }
      } else {
        result.insert(idx_result_ty);
      }

      return true;
    }
  }

  false
}
