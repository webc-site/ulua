use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::reduction::Reduction,
  functions::{
    follow_type, is_blocked_or_unsolved_type::is_blocked_or_unsolved_type, is_pending::is_pending,
    simplify_intersection_simplify::simplify_intersection, simplify_union::simplify_union,
  },
  records::{
    arena_handle::Handle, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// `andTypeFunction` / `orTypeFunction`（`BuiltinTypeFunctions.cpp:698` 起）孪生对
/// 的共享骨架：实参形态校验、吸收律短路、操作数未决早退，随后走
/// simplifyIntersection/Union（and 过滤 falsy、or 过滤 truthy）。`is_or` 选择
/// 语义侧：false=and（pending 判定经 solver），true=or。
///
/// # Safety
/// 本函数由 and/or 包装器原样透传其 # Safety 契约调用：`ctx` 须为整个 reduce
/// 步进期内独占存活的 `TypeFunctionContext` 借用（cpp `NotNull`），`type_params`
/// 须恰 2 项、`pack_params` 须为空，且两切片内嵌 `TypeId`/`TypePackId` 句柄指向
/// 存活类型 arena 节点（体内经 `follow_type_id`/`is_pending` 间接解引用它们）。
pub(crate) unsafe fn reduce_and_or_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
  is_or: bool,
  ice_msg: &str,
) -> TypeFunctionReductionResult {
  if type_params.len() != 2 || !pack_params.is_empty() {
    ctx.ice().ice_string(ice_msg);
    LUAU_ASSERT!(false);
  }

  let lhs_ty = follow_type::follow(type_params[0]);
  let rhs_ty = follow_type::follow(type_params[1]);

  // t1 = and/or<lhs, t1> ~> lhs
  if follow_type::follow(rhs_ty) == instance && lhs_ty != rhs_ty {
    return TypeFunctionReductionResult::reduction(lhs_ty);
  }
  // t1 = and/or<t1, rhs> ~> rhs
  if follow_type::follow(lhs_ty) == instance && lhs_ty != rhs_ty {
    return TypeFunctionReductionResult::reduction(rhs_ty);
  }

  // check to see if both operand types are resolved enough, and wait to reduce if not
  let unresolved = |ty: TypeId| -> bool {
    if is_or {
      is_blocked_or_unsolved_type(ty)
    } else {
      // Safety: `is_pending` 内部对 `solver` 走 `Option::as_mut` 判空（对应 C++
      // 可空 `ConstraintSolver*`），非空时指向检查会话存活的 solver。
      unsafe { is_pending(ty, ctx.solver) }
    }
  };
  if unresolved(lhs_ty) {
    return TypeFunctionReductionResult::no_reduction(Vec::from([lhs_ty]));
  } else if unresolved(rhs_ty) {
    return TypeFunctionReductionResult::no_reduction(Vec::from([rhs_ty]));
  }

  // And evaluates to a boolean if the LHS is falsy, and the RHS type if LHS is truthy.
  // or 对偶：lhs 过滤 truthy 后与 rhs 取并。
  let filter_ty = if is_or {
    ctx.builtins().truthy_type
  } else {
    ctx.builtins().falsy_type
  };
  let filtered_lhs = simplify_intersection(
    Handle::from_ref(ctx.builtins()),
    Handle::from_mut(ctx.arena_mut()),
    lhs_ty,
    filter_ty,
  );
  let overall_result = simplify_union(
    Handle::from_ref(ctx.builtins()),
    Handle::from_mut(ctx.arena_mut()),
    rhs_ty,
    filtered_lhs.result,
  );

  let mut blocked_types: Vec<TypeId> = Vec::new();
  for ty in filtered_lhs.blocked_types.iter() {
    blocked_types.push(*ty);
  }
  for ty in overall_result.blocked_types.iter() {
    blocked_types.push(*ty);
  }

  TypeFunctionReductionResult {
    result: Some(overall_result.result),
    reduction_status: Reduction::MaybeOk,
    blocked_types,
    blocked_packs: Vec::new(),
    error: None,
    messages: Vec::new(),
  }
}
