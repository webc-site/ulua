//! C++ recursive `bool tblIndexInto(TypeId indexer, TypeId indexee,
//! DenseHashSet<TypeId>& result, DenseHashSet<TypeId>& seenSet,
//! NotNull<TypeFunctionContext> ctx, bool isRaw)`
//! (BuiltinTypeFunctions.cpp:1988-2066). Collects the types reachable by indexing
//! `indexee` with `indexer` into `result`.
use alloc::{vec, vec::Vec};

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    begin_type::begin_union_type, extend_type_pack::extend_type_pack,
    find_metatable_entry::find_metatable_entry, follow_type, get_type,
    search_props_and_indexer::search_props_and_indexer, solve_function_call::solve_function_call,
  },
  records::{
    arena_handle::Handle, function_type::FunctionType, metatable_type::MetatableType,
    table_type::TableType, type_function_context::TypeFunctionContext, type_pack::TypePack,
    union_type::UnionType,
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
  ctx: &TypeFunctionContext,
  is_raw: bool,
) -> bool {
  // SAFETY: 调用链（index_function_impl / 递归）保证 ctx 为指向会话有效上下文的只读借用；
  // arena 写回经由其 NonNull 字段的 as_ptr 派生独占借用，符合单线程串行契约。
  let ctx_ref = ctx;

  let indexer = follow_type::follow(indexer);
  let indexee = follow_type::follow(indexee);

  if seen_set.contains(&indexee) {
    return false;
  }
  seen_set.insert(indexee);

  if let Some(union_ty) = get_type::get::<UnionType>(indexee) {
    let mut res = true;
    // C++ `for (auto component : unionTy)`——UnionTypeIterator 展平嵌套
    // union 并 follow,裸遍历 options 会漏掉嵌套成员。
    for component in begin_union_type(union_ty) {
      // if the component is in the seen set and isn't the indexee itself,
      // we can skip it cause it means we encountered it in an earlier
      // component in the union.
      if seen_set.contains(&component) && component != indexee {
        continue;
      }
      res = res
        && unsafe {
          // Safety: 递归透传的 ctx 即函数头已证成「非空且会话期存活」的同一指针，
          // result/seen_set 为独占 &mut 借用；component 是 union 展平迭代给出的
          // arena 存活 TypeId 句柄，递归前置条件与首次调用完全相同。
          tbl_index_into(indexer, component, result, seen_set, ctx, is_raw)
        };
    }
    return res;
  }

  if get_type::get::<FunctionType>(indexee).is_some() {
    // Safety: ctx_ref.arena 是 TypeFunctionContext 构造期接线的 NonNull<TypeArena>，
    // 指向会话存活 bump arena；此刻无并存可变借用（本函数单线程串行，arena 经 ctx
    // 独占访问），as_ptr→&mut 重建的可变借用仅在 add 调用这一语句内存活。
    let arg_pack =
      unsafe { &mut *ctx_ref.arena.as_ptr() }.add_type_pack_t(TypePack::single(indexer));

    // Safety: solve_function_call 的前置条件（ctx 为指向存活 TypeFunctionContext 的只读
    // 借用）与函数头 ctx_ref 的证成同源同址；scope 亦是构造期接线的 NonNull<Scope>，
    // as_ref 只读取其 location 字段（Copy），不产生写入。
    let ret_pack =
      unsafe { solve_function_call(ctx, ctx_ref.scope.as_ref().location, indexee, arg_pack) };

    let ret_pack = match ret_pack {
      Some(rp) => rp,
      None => return false,
    };

    // Safety: extend_type_pack 收到的是同一证成下的 arena 可变句柄与构造期接线的
    // builtins 只读指针（NonNull::as_ptr 保真）；ret_pack 为刚在 arena 上分配的
    // 存活 pack 句柄，Vec::new() 不引入额外别名。
    let extracted = unsafe {
      extend_type_pack(
        &mut *ctx_ref.arena.as_ptr(),
        Handle::from_ptr(ctx_ref.builtins.as_ptr()),
        ret_pack,
        1,
        Vec::new(),
      )
    };
    if extracted.head.is_empty() {
      return false;
    }

    result.insert(follow_type::follow(extracted.head[0]));
    return true;
  }

  // we have a table type to try indexing
  if let Some(table_ty) = get_type::get::<TableType>(indexee) {
    // Safety: search_props_and_indexer 的 ctx 前置条件即函数头对 ctx_ref 证成的
    // 同一「非空且会话存活」前提（透传的是原指针，非派生别名）；table_ty 来自
    // get_type_id 对存活 arena 节点的 class 判定，props 已 clone 脱离节点借用。
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
  if let Some(metatable_ty) = get_type::get::<MetatableType>(indexee) {
    if let Some(table_ty) = get_type::get::<TableType>(follow_type::follow(metatable_ty.table())) {
      // try finding all properties within the current scope of the table
      // Safety: 同上——ctx 透传函数头证成的非空存活前提；table_ty 为 follow 后
      // 命中 TableType 的 arena 存活节点，props 以 clone 传入，不跨调用持有借用。
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
        Handle::from_ptr(ctx_ref.builtins.as_ptr()),
        &mut dummy,
        indexee,
        "__index",
        Location::new(
          Position { line: 0, column: 0 },
          Position { line: 0, column: 0 },
        ),
      );
      if let Some(mm_type) = mm_type {
        // Safety: 递归透传函数头已证成非空存活的同一 ctx 与独占 result/seen_set 借用；
        // mm_type 是 find_metatable_entry 从存活 __index 类型解析出的 arena TypeId 句柄。
        return unsafe { tbl_index_into(indexer, mm_type, result, seen_set, ctx, is_raw) };
      }
    }
  }

  false
}
