use alloc::vec::Vec;

use crate::{
  enums::reduction::Reduction,
  functions::{
    follow_type, is_blocked_or_unsolved_type::is_blocked_or_unsolved_type, is_pending::is_pending,
    simplify_intersection_simplify::simplify_intersection, simplify_union::simplify_union,
  },
  macros::check_arity,
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
/// 体已全 safe：`ctx` 借用与句柄解引用分别由 `is_pending`/`get_type` 等收口
/// 门面承担。前置条件（`check_arity` 断言）：`type_params` 恰 2 项、`pack_params` 为空。
pub(crate) fn reduce_and_or_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
  is_or: bool,
  ice_msg: &str,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 2, ice_msg ice_msg);

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
      // `is_pending` 内部对 `solver` 判空收口（对应 C++ 可空 `ConstraintSolver*`）。
      is_pending(ty, ctx.solver)
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
  blocked_types.extend(filtered_lhs.blocked_types.iter().copied());
  blocked_types.extend(overall_result.blocked_types.iter().copied());

  TypeFunctionReductionResult {
    result: Some(overall_result.result),
    reduction_status: Reduction::MaybeOk,
    blocked_types,
    blocked_packs: Vec::new(),
    error: None,
    messages: Vec::new(),
  }
}
