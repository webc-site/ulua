//! C++ recursive `bool tblIndexInto(TypeId indexer, TypeId indexee,
//! DenseHashSet<TypeId>& result, DenseHashSet<TypeId>& seenSet,
//! NotNull<TypeFunctionContext> ctx, bool isRaw)`
//! (BuiltinTypeFunctions.cpp:1988-2066). Collects the types reachable by indexing
//! `indexee` with `indexer` into `result`.
use alloc::{vec, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry,
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    search_props_and_indexer::search_props_and_indexer, solve_function_call::solve_function_call,
  },
  records::{
    function_type::FunctionType, metatable_type::MetatableType, table_type::TableType,
    type_function_context::TypeFunctionContext, type_pack::TypePack, union_type::UnionType,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn tbl_index_into(
  indexer: TypeId,
  indexee: TypeId,
  result: &mut DenseHashSet<TypeId>,
  seen_set: &mut DenseHashSet<TypeId>,
  ctx: *mut TypeFunctionContext,
  is_raw: bool,
) -> bool {
  // SAFETY: 调用链（BuiltinTypeFunctions 注册表/递归）保证 ctx 非空且会话期有效。
  // SAFETY: 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效；
  // NonNull 封装解引用。
  let ctx_ref = unsafe { NonNull::new_unchecked(ctx).as_mut() };

  let indexer = follow_type_id(indexer);
  let indexee = follow_type_id(indexee);

  if seen_set.contains(&indexee) {
    return false;
  }
  seen_set.insert(indexee);

  if let Some(union_ty) = get_type_id::<UnionType>(indexee) {
    let mut res = true;
    for &component in &union_ty.options {
      // if the component is in the seen set and isn't the indexee itself,
      // we can skip it cause it means we encountered it in an earlier
      // component in the union.
      if seen_set.contains(&component) && component != indexee {
        continue;
      }
      res = res && unsafe { tbl_index_into(indexer, component, result, seen_set, ctx, is_raw) };
    }
    return res;
  }

  if get_type_id::<FunctionType>(indexee).is_some() {
    let arg_pack = unsafe { ctx_ref.arena.as_mut() }.add_type_pack_t(TypePack {
      head: vec![indexer],
      tail: None,
    });

    let ret_pack =
      unsafe { solve_function_call(ctx, ctx_ref.scope.as_ref().location, indexee, arg_pack) };

    let ret_pack = match ret_pack {
      Some(rp) => rp,
      None => return false,
    };

    let extracted = unsafe {
      extend_type_pack(
        ctx_ref.arena.as_mut(),
        ctx_ref.builtins.as_ptr(),
        ret_pack,
        1,
        Vec::new(),
      )
    };
    if extracted.head.is_empty() {
      return false;
    }

    result.insert(follow_type_id(extracted.head[0]));
    return true;
  }

  // we have a table type to try indexing
  if let Some(table_ty) = get_type_id::<TableType>(indexee) {
    return unsafe {
      search_props_and_indexer(
        indexer,
        table_ty.props.clone(),
        table_ty.indexer,
        result,
        ctx,
      )
    };
  }

  // we have a metatable type to try indexing
  if let Some(metatable_ty) = get_type_id::<MetatableType>(indexee) {
    if let Some(table_ty) = get_type_id::<TableType>(follow_type_id(metatable_ty.table())) {
      // try finding all properties within the current scope of the table
      if unsafe {
        search_props_and_indexer(
          indexer,
          table_ty.props.clone(),
          table_ty.indexer,
          result,
          ctx,
        )
      } {
        return true;
      }
    }

    // if the code reached here, it means we weren't able to find all
    // properties -> look into __index metamethod
    if !is_raw {
      // findMetatableEntry demands the ability to emit errors, so we must
      // give it the necessary state to do that, even if we intend to just
      // eat the errors.
      let mut dummy: ErrorVec = vec![];
      let mm_type = find_metatable_entry(
        ctx_ref.builtins.as_ptr(),
        &mut dummy,
        indexee,
        "__index",
        Location::new(
          Position { line: 0, column: 0 },
          Position { line: 0, column: 0 },
        ),
      );
      if let Some(mm_type) = mm_type {
        return unsafe { tbl_index_into(indexer, mm_type, result, seen_set, ctx, is_raw) };
      }
    }
  }

  false
}
