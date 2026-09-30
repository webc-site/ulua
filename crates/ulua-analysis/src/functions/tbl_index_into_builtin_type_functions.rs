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
/// 参数全部为引用/切片/句柄值，签名无需 `unsafe`；体内经 ctx 的 NonNull 字段
/// 重建 arena 可变借用的调用各自留在收窄的 `unsafe {}` 内逐处论证。
pub fn tbl_index_into(
  indexer: TypeId,
  indexee: TypeId,
  result: &mut DenseHashSet<TypeId>,
  seen_set: &mut DenseHashSet<TypeId>,
  ctx: &TypeFunctionContext,
  is_raw: bool,
) -> bool {
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
      res = res && tbl_index_into(indexer, component, result, seen_set, ctx, is_raw);
    }
    return res;
  }

  if get_type::get::<FunctionType>(indexee).is_some() {
    // Safety: ctx.arena 是 TypeFunctionContext 构造期接线的 NonNull<TypeArena>，
    // 指向会话存活 bump arena；此刻无并存可变借用（求解单线程串行驱动），
    // as_ptr→&mut 重建的可变借用仅在 add 调用这一语句内存活。
    let arg_pack = (unsafe { &mut *ctx.arena.as_ptr() }).add_type_pack_t(TypePack::single(indexer));

    // solve_function_call 为安全函数：ctx 各 NonNull 字段的重建与论证收口在其体内。
    // Safety: ctx.scope 为构造期接线的 NonNull<Scope>，非空且在调用期存活；
    // 仅读取其 Copy 的 location 字段，引用不逃逸本语句。
    let scope_location = unsafe { ctx.scope.as_ref() }.location;
    let ret_pack = solve_function_call(ctx, scope_location, indexee, arg_pack);

    let ret_pack = match ret_pack {
      Some(rp) => rp,
      None => return false,
    };

    // Safety: extend_type_pack 收到的是同一 ctx.arena NonNull 重建的 arena 可变
    // 借用（构造期接线、单线程独占）与 builtins 只读句柄；ret_pack 为刚在 arena
    // 上分配的存活 pack 句柄，Vec::new() 不引入额外别名。
    let extracted = extend_type_pack(
      unsafe { &mut *ctx.arena.as_ptr() },
      Handle::from_ptr(ctx.builtins.as_ptr()),
      ret_pack,
      1,
      Vec::new(),
    );
    if extracted.head.is_empty() {
      return false;
    }

    result.insert(follow_type::follow(extracted.head[0]));
    return true;
  }

  // we have a table type to try indexing
  if let Some(table_ty) = get_type::get::<TableType>(indexee) {
    // search_props_and_indexer 为安全函数；table_ty 来自 get_type 对存活 arena
    // 节点的变体判定，props 已 clone 脱离节点借用。
    return search_props_and_indexer(
      indexer,
      table_ty.props.clone(),
      table_ty.indexer,
      result,
      ctx,
    );
  }

  // we have a metatable type to try indexing
  if let Some(metatable_ty) = get_type::get::<MetatableType>(indexee) {
    if let Some(table_ty) = get_type::get::<TableType>(follow_type::follow(metatable_ty.table())) {
      // try finding all properties within the current scope of the table
      // 同上——table_ty 为 follow 后命中 TableType 的 arena 存活节点，
      // props 以 clone 传入，不跨调用持有借用；callee 为安全函数。
      if search_props_and_indexer(
        indexer,
        table_ty.props.clone(),
        table_ty.indexer,
        result,
        ctx,
      ) {
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
        Handle::from_ptr(ctx.builtins.as_ptr()),
        &mut dummy,
        indexee,
        "__index",
        Location::new(
          Position { line: 0, column: 0 },
          Position { line: 0, column: 0 },
        ),
      );
      if let Some(mm_type) = mm_type {
        return tbl_index_into(indexer, mm_type, result, seen_set, ctx, is_raw);
      }
    }
  }

  false
}
