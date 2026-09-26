use alloc::vec::Vec;

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::normalized_part::TABLE_ONLY_PARTS,
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type, get_type, is_pending::is_pending,
    simplify_union::simplify_union,
  },
  records::{
    arena_handle::Handle, metatable_type::MetatableType, table_type::TableType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// C++ `setmetatableTypeFunction`（BuiltinTypeFunctions.cpp:2283-2385）：把
/// `setmetatable(T, M)` 归约为带 M 元表的 T，逐表部件检查 `__metatable` 锁。
///
/// # Safety
/// - `instance`：本实现不使用（C++ 侧亦只用于日志），无指针契约。
/// - `type_params`：必须恰有两个元素，且二者都是 arena 里存活的 `TypeId`；本函数只按
///   值把它们交给 `follow`。若 arity 不符，首行 `LUAU_ASSERT!` 先触发，随后的 `[0]`/`[1]`
///   越界只会 panic（安全失败），不会造成未定义行为。
/// - `pack_params`：必须是空切片；非空同样只触发断言，不被解引用。
/// - `ctx`：对应 C++ `NotNull<TypeFunctionContext>`——本次调用独占存活的借用。其
///   `arena`/`builtins`/`normalizer` 通过安全访问器（`ctx.arena_mut()`、`ctx.builtins()`、
///   `ctx.normalizer_mut()`）使用，`solver`/`constraint` 允许为空，由被调方各自判空。
pub unsafe fn setmetatable_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  if type_params.len() != 2 || !pack_params.is_empty() {
    LUAU_ASSERT!(false);
  }

  // Safety: `constraint` 是 ctx 里可空的 `*const Constraint`（solver 之外的归约路径为 null），
  // 先判空再解引用；非空时它指向驱动本次归约、在 ctx 生命周期内存活的约束记录，
  // 这里只 Copy 出它的 location。
  let location = unsafe {
    if !ctx.constraint.is_null() {
      (*ctx.constraint).location
    } else {
      Location::new(Position::default(), Position::default())
    }
  };

  let target_ty = follow_type::follow(type_params[0]);
  let metatable_ty = follow_type::follow(type_params[1]);

  // Safety: `ctx.solver` 允许为 null（非 solver 路径），is_pending 内部用
  // `Option::as_mut` 判空；非空时它指向驱动本次归约的 ConstraintSolver，调用期内存活。
  if unsafe { is_pending(target_ty, ctx.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![target_ty]);
  }

  // C++ 中归一化失败返回空指针：不归约，对驻留性一无所知
  let Some(target_norm) = ctx.normalizer_mut().try_normalize(target_ty) else {
    return TypeFunctionReductionResult::no_reduction(Vec::new());
  };

  let target_norm_ref = target_norm.as_ref();
  if !target_norm_ref.has_tables() {
    return TypeFunctionReductionResult::erroneous();
  }

  if target_norm_ref.has_parts_other_than(&TABLE_ONLY_PARTS) {
    return TypeFunctionReductionResult::erroneous();
  }

  // Safety: 与 target 侧同一 solver 判空契约；此处只把 metatable_ty 送去问驻留性，
  // 借用覆盖 is_pending 这一次同步调用。
  if unsafe { is_pending(metatable_ty, ctx.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![metatable_ty]);
  }

  let metatable_is_table_or_metatable = get_type::get::<TableType>(metatable_ty).is_some()
    || get_type::get::<MetatableType>(metatable_ty).is_some();

  if !metatable_is_table_or_metatable {
    return TypeFunctionReductionResult::erroneous();
  }

  if target_norm_ref.tables.size() == 1 {
    let table = target_norm_ref.tables.front();

    let mut dummy: ErrorVec = Vec::new();

    let metatable_metamethod = find_metatable_entry(
      Handle::from_ref(ctx.builtins()),
      &mut dummy,
      table,
      "__metatable",
      location,
    );

    if metatable_metamethod.is_some() {
      return TypeFunctionReductionResult::erroneous();
    }

    let with_metatable = ctx.arena_mut().add_type(MetatableType {
      table,
      metatable: metatable_ty,
      synthetic_name: None,
    });

    return TypeFunctionReductionResult::reduction(with_metatable);
  }

  let mut result = ctx.builtins().never_type;

  for component_ty in target_norm_ref.tables.order.iter().copied() {
    let mut dummy: ErrorVec = Vec::new();

    let metatable_metamethod = find_metatable_entry(
      Handle::from_ref(ctx.builtins()),
      &mut dummy,
      component_ty,
      "__metatable",
      location,
    );

    if metatable_metamethod.is_some() {
      return TypeFunctionReductionResult::erroneous();
    }

    let with_metatable = ctx.arena_mut().add_type(MetatableType {
      table: component_ty,
      metatable: metatable_ty,
      synthetic_name: None,
    });

    let simplified = simplify_union(
      Handle::from_ref(ctx.builtins()),
      Handle::from_nonnull(ctx.arena),
      result,
      with_metatable,
    );

    if !simplified.blocked_types.empty() {
      let mut blocked_types = Vec::new();
      for ty in simplified.blocked_types.iter() {
        blocked_types.push(*ty);
      }
      return TypeFunctionReductionResult::no_reduction(blocked_types);
    }

    result = simplified.result;
  }

  TypeFunctionReductionResult::reduction(result)
}
