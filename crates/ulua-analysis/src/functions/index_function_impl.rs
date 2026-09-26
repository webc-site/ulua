//! C++ `TypeFunctionReductionResult<TypeId> indexFunctionImpl(
//! const std::vector<TypeId>& typeParams, const std::vector<TypePackId>&
//! packParams, NotNull<TypeFunctionContext> ctx, bool isRaw)`
//! (BuiltinTypeFunctions.cpp:2077-2209). Shared implementation behind `index`
//! and `rawget`.
//!
//! Vocabulary note: indexee refers to the type that contains the properties,
//! indexer refers to the type that is used to access indexee.
//! `index<Person, "name">` => `Person` is the indexee and `"name"` is the indexer.
use alloc::{vec, vec::Vec};

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::normalized_part::TABLE_OR_EXTERN_PARTS,
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type, get_type, is_pending::is_pending,
    search_props_and_indexer::search_props_and_indexer,
    tbl_index_into_builtin_type_functions::tbl_index_into,
  },
  records::{
    arena_handle::Handle, extern_type::ExternType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, union_type::UnionType,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
fn empty_location() -> Location {
  Location::new(
    Position { line: 0, column: 0 },
    Position { line: 0, column: 0 },
  )
}

/// C++ overload `bool tblIndexInto(TypeId indexer, TypeId indexee,
/// DenseHashSet<TypeId>& result, NotNull<TypeFunctionContext> ctx, bool isRaw)`
/// (BuiltinTypeFunctions.cpp:2068-2072): seeds an empty seen-set and delegates to
/// the recursive form.
fn tbl_index_into_2(
  indexer: TypeId,
  indexee: TypeId,
  result: &mut DenseHashSet<TypeId>,
  ctx: &TypeFunctionContext,
  is_raw: bool,
) -> bool {
  let mut seen_set: DenseHashSet<TypeId> = DenseHashSet::default();
  // Safety: seen_set/result 均为本函数拥有对象的 &mut 借用，调用期内有效；
  // indexer/indexee/ctx/is_raw 由调用方 index_function_impl 逐参数原样转递，
  // 其存活与只读借用论证继承函数级契约，callee 形参要求随之成立。
  unsafe { tbl_index_into(indexer, indexee, result, &mut seen_set, ctx, is_raw) }
}

fn erroneous() -> TypeFunctionReductionResult {
  TypeFunctionReductionResult::erroneous()
}

fn maybe_ok_blocked(blocked: Vec<TypeId>) -> TypeFunctionReductionResult {
  TypeFunctionReductionResult::no_reduction(blocked)
}

/// # Safety
/// 对应 cpp `indexFunctionImpl(const std::vector<TypeId>&, const
/// std::vector<TypePackId>&, NotNull<TypeFunctionContext>, bool)`
/// （BuiltinTypeFunctions.cpp:2077）。逐参数契约：
/// - `type_params`：长度 ≥2（arity 由 typeFunctionRuntime 在进入前按类型函数
///   声明校验），元素为指向 TypeArena 存活节点的 TypeId；[0] 是 indexee、
///   [1] 是 indexer，本函数只 follow 不解引用它们。
/// - `_pack_params`：对应 cpp packParams 向量，本实现从不读取，可空可短。
/// - `ctx`：只读借用，指向当前类型函数实例的 TypeFunctionContext（C++ NotNull
///   按值转递、驱动栈帧上的对象，存活期覆盖本次调用）；其 arena/builtins/
///   normalizer/solver 字段满足各字段自身的 NotNull 构造契约，solver 指向
///   正在驱动本次求值的 ConstraintSolver；调用期间无第二并发访问路径
///   （单线程，驱动方栈帧内）。
/// - `is_raw`：区分 index/rawget 语义的标量，无 UB 约束。
pub unsafe fn index_function_impl(
  type_params: &[TypeId],
  _pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
  is_raw: bool,
) -> TypeFunctionReductionResult {
  let indexee_ty = follow_type::follow(type_params[0]);

  // Safety: is_pending 需要非空存活的 solver——ctx.solver 按函数级契约
  // 指向驱动本次求值的 ConstraintSolver；indexee_ty 是 type_params[0] 的
  // follow 值（arena 活 TypeId），callee 内仅做状态只读判定。
  if unsafe { is_pending(indexee_ty, ctx.solver) } {
    return maybe_ok_blocked(vec![indexee_ty]);
  }

  let Some(indexee_norm_ty) = ctx.normalizer_mut().try_normalize(indexee_ty) else {
    return maybe_ok_blocked(vec![]);
  };

  // if the indexee is `any`, then indexing also gives us `any`.
  if indexee_norm_ty.should_suppress_errors() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().any_type);
  }

  // if we don't have either just tables or just extern types, we've got nothing to index into
  if indexee_norm_ty.has_tables() == indexee_norm_ty.has_extern_types() {
    return erroneous();
  }

  // we're trying to reject any type that has not normalized to a table or
  // extern type or a union of tables or extern types.
  if indexee_norm_ty.has_parts_other_than(&TABLE_OR_EXTERN_PARTS) {
    return erroneous();
  }

  let indexer_ty = follow_type::follow(type_params[1]);

  // Safety: 与 indexee 处同样的 solver 存活论证，此处对象换作 type_params[1]
  // 的 follow 结果；判读只触及实例状态，不写 arena。
  if unsafe { is_pending(indexer_ty, ctx.solver) } {
    return maybe_ok_blocked(vec![indexer_ty]);
  }

  let Some(indexer_norm_ty) = ctx.normalizer_mut().try_normalize(indexer_ty) else {
    return maybe_ok_blocked(vec![]);
  };

  // we're trying to reject any type that is not a string singleton or primitive
  if indexer_norm_ty.has_tops() || indexer_norm_ty.has_errors() {
    return erroneous();
  }

  // indexer can be a union —> break them down into a vector
  let single_type: Vec<TypeId> = vec![indexer_ty];
  let types_to_find: &Vec<TypeId> = if let Some(union_ty) = get_type::get::<UnionType>(indexer_ty) {
    &union_ty.options
  } else {
    &single_type
  };

  let mut properties: DenseHashSet<TypeId> = DenseHashSet::default(); // types that will be returned

  if indexee_norm_ty.has_extern_types() {
    LUAU_ASSERT!(!indexee_norm_ty.has_tables());

    if is_raw {
      // rawget should never reduce for extern types (to match the behavior
      // of the rawget global function)
      return erroneous();
    }

    // at least one class is guaranteed to be in the iterator by .hasExternTypes()
    for &extern_type_iter in &indexee_norm_ty.extern_types.ordering {
      let Some(extern_ref) = get_type::get::<ExternType>(extern_type_iter) else {
        LUAU_ASSERT!(false); // not possible according to normalization's spec
        return erroneous();
      };

      for &ty in types_to_find {
        // Search for all instances of indexer in class->props and class->indexer
        // Safety: ty 是 types_to_find 元素（indexer 类型或其 union options，
        // 皆 arena 活 TypeId）；extern_ref.props 先深拷贝为 owned Props，不留下
        // 跨调用的 arena 借用；ctx 原样转递（函数级契约的存活+独占）；
        // properties 为本函数 owned 集合，与 ctx/arena 内存不相交。
        if unsafe {
          search_props_and_indexer(
            ty,
            extern_ref.props.clone(),
            extern_ref.indexer,
            &mut properties,
            ctx,
          )
        } {
          continue; // found in this class, move to the next type
        }

        let mut parent = extern_ref.parent;
        let mut found_in_parent = false;
        while let Some(parent_ty) = parent {
          if found_in_parent {
            break;
          }
          // C++: parent 链上节点按不变量必可下转为 ExternType（get<> 必命中）
          let parent_ref = get_type::get::<ExternType>(follow_type::follow(parent_ty))
            .expect("parent 链节点按 cpp 不变量必为 ExternType，下转必命中");
          // Safety: parent_ref 命中即随 arena 存活，其 props/indexer 取值形状
          // 与上同；ty 与 ctx 的论证不变，properties 仍是本函数唯一可变借用。
          found_in_parent = unsafe {
            search_props_and_indexer(
              ty,
              parent_ref.props.clone(),
              parent_ref.indexer,
              &mut properties,
              ctx,
            )
          };
          parent = parent_ref.parent;
        }

        // we move on to the next type if any of the parents had the property.
        if found_in_parent {
          continue;
        }

        // property not found -> check in the metatable's __index.
        // findMetatableEntry demands the ability to emit errors, so we
        // must give it the state to do that, even if we eat the errors.
        let mut dummy: ErrorVec = vec![];
        let mm_type = find_metatable_entry(
          Handle::from_ref(ctx.builtins()),
          &mut dummy,
          extern_type_iter,
          "__index",
          empty_location(),
        );
        let mm_type = match mm_type {
          Some(mm) => mm,
          None => return erroneous(), // no metatable -> nowhere else to look
        };

        if !tbl_index_into_2(ty, mm_type, &mut properties, ctx, is_raw) {
          // if indexer is not in the metatable, we fail to reduce
          return erroneous();
        }
      }
    }
  }

  if indexee_norm_ty.has_tables() {
    LUAU_ASSERT!(!indexee_norm_ty.has_extern_types());

    // at least one table is guaranteed to be in the iterator by .hasTables()
    for &tables_iter in &indexee_norm_ty.tables.order {
      for &ty in types_to_find {
        if !tbl_index_into_2(ty, tables_iter, &mut properties, ctx, is_raw) {
          if is_raw {
            properties.insert(ctx.builtins().nil_type);
          } else {
            return erroneous();
          }
        }
      }
    }
  }

  // If the type being reduced to is a single type, no need to union
  if properties.size() == 1 {
    let only = *properties
      .iter()
      .next()
      .expect("properties has exactly one element");
    return TypeFunctionReductionResult::reduction(only);
  }

  let options: Vec<TypeId> = properties.iter().copied().collect();
  let union_ty = ctx.arena_mut().add_type(UnionType { options });
  TypeFunctionReductionResult::reduction(union_ty)
}
