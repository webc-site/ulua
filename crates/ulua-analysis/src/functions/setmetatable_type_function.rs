use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  enums::normalized_part::TABLE_ONLY_PARTS,
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type, get_type, is_pending::is_pending,
    simplify_union::simplify_union,
  },
  macros::check_arity,
  records::{
    arena_handle::{Handle, alias_ref},
    metatable_type::MetatableType,
    table_type::TableType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// C++ `setmetatableTypeFunction`（BuiltinTypeFunctions.cpp:2283-2385）：把
/// `setmetatable(T, M)` 归约为带 M 元表的 T，逐表部件检查 `__metatable` 锁。
pub fn setmetatable_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(type_params, pack_params, 2, assert_only);

  let location = if ctx.constraint.is_null() {
    Location::default()
  } else {
    alias_ref(ctx.constraint).location
  };

  let target_ty = follow_type::follow(type_params[0]);
  let metatable_ty = follow_type::follow(type_params[1]);

  // `is_pending` 为安全函数：solver 允许为 null（非 solver 路径），内部判空折叠。
  if is_pending(target_ty, ctx.solver) {
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

  // 与 target 侧同一 solver 判空契约；此处只把 metatable_ty 送去问驻留性。
  if is_pending(metatable_ty, ctx.solver) {
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
      let blocked_types = simplified.blocked_types.iter().copied().collect();
      return TypeFunctionReductionResult::no_reduction(blocked_types);
    }

    result = simplified.result;
  }

  TypeFunctionReductionResult::reduction(result)
}
