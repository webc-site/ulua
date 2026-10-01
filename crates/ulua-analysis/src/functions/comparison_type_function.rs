use alloc::{string::String, vec::Vec};

use ulua_ast::records::{location::Location, position::Position};

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    as_mutable_type::as_mutable_type_id, find_metatable_entry::find_metatable_entry, follow_type,
    get_type, is_blocked_or_unsolved_type::is_blocked_or_unsolved_type, is_pending::is_pending,
    is_prim::is_number, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  macros::check_arity,
  methods::unifiable::unifiable_bound_type_id_emplace_type_bound_type,
  records::{
    arena_handle::{Handle, alias, alias_ref},
    free_type::FreeType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
    type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// 对应 C++ `comparisonTypeFunction`（`cpp/Analysis/src/BuiltinTypeFunctions.cpp:778`），
/// 由 `lt`/`le` 等比较类包装入口与自身递归闭包转发。作为 `ReducerFunction` 的共享实现，
/// 其会话对象经 `TypeFunctionContext` 安全访问器取用；仅裸指针读取（constraint、arena
/// 节点 emplace）留在收窄的 `unsafe {}` 内。
pub fn comparison_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
  metamethod: String,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 2);

  let lhs_ty = follow_type::follow(type_params[0]);
  let rhs_ty = follow_type::follow(type_params[1]);

  if lhs_ty == instance || rhs_ty == instance {
    return TypeFunctionReductionResult::reduction(ctx.builtins().never_type);
  }

  if is_blocked_or_unsolved_type(lhs_ty) {
    return TypeFunctionReductionResult::no_reduction(vec![lhs_ty]);
  } else if is_blocked_or_unsolved_type(rhs_ty) {
    return TypeFunctionReductionResult::no_reduction(vec![rhs_ty]);
  }

  let can_submit_constraint = !ctx.solver.is_null() && !ctx.constraint.is_null();
  let lhs_free = get_type::get::<FreeType>(lhs_ty).is_some();
  let rhs_free = get_type::get::<FreeType>(rhs_ty).is_some();

  if can_submit_constraint {
    if lhs_free && is_number(rhs_ty) {
      let mut number_type = ctx.builtins().number_type;
      unifiable_bound_type_id_emplace_type_bound_type(
        alias(as_mutable_type_id(lhs_ty)),
        &mut number_type,
      );
    } else if rhs_free && is_number(lhs_ty) {
      let mut number_type = ctx.builtins().number_type;
      unifiable_bound_type_id_emplace_type_bound_type(
        alias(as_mutable_type_id(rhs_ty)),
        &mut number_type,
      );
    }
  }

  let lhs_ty = follow_type::follow(lhs_ty);
  let rhs_ty = follow_type::follow(rhs_ty);

  // C++ 中归一化失败返回空指针：不归约，对驻留性一无所知
  let norm_lhs_ty = ctx.normalizer_mut().try_normalize(lhs_ty);
  let norm_rhs_ty = ctx.normalizer_mut().try_normalize(rhs_ty);
  let (Some(norm_lhs_ty), Some(norm_rhs_ty)) = (norm_lhs_ty, norm_rhs_ty) else {
    return TypeFunctionReductionResult::no_reduction(Vec::new());
  };
  let lhs_inhabited = ctx
    .normalizer_mut()
    .is_inhabited_normalized_type(norm_lhs_ty.as_ref());
  let rhs_inhabited = ctx
    .normalizer_mut()
    .is_inhabited_normalized_type(norm_rhs_ty.as_ref());

  if lhs_inhabited == NormalizationResult::HitLimits
    || rhs_inhabited == NormalizationResult::HitLimits
  {
    return TypeFunctionReductionResult::no_reduction(Vec::new());
  }

  if norm_lhs_ty.should_suppress_errors() || norm_rhs_ty.should_suppress_errors() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().boolean_type);
  }

  if lhs_inhabited == NormalizationResult::False || rhs_inhabited == NormalizationResult::False {
    return TypeFunctionReductionResult::reduction(ctx.builtins().boolean_type);
  }

  if norm_lhs_ty.is_subtype_of_string() && norm_rhs_ty.is_subtype_of_string() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().boolean_type);
  }

  if norm_lhs_ty.is_exactly_number() && norm_rhs_ty.is_exactly_number() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().boolean_type);
  }

  // try_distribute_type_function_app 为安全函数：递归闭包捕获 metamethod，透传
  // 同一独占借用递归回安全的本函数，实参形状由每次递归入口的 check_arity 守卫维持。
  if let Some(result) = try_distribute_type_function_app(
    |instance, type_params, pack_params, ctx| {
      comparison_type_function(instance, type_params, pack_params, ctx, metamethod.clone())
    },
    instance,
    type_params,
    pack_params,
    ctx,
  ) {
    return result;
  }

  let mut dummy: ErrorVec = Vec::new();
  let location = Location::new(Position::default(), Position::default());

  let mut mm_type = find_metatable_entry(
    Handle::from_ptr(ctx.builtins.as_ptr()),
    &mut dummy,
    lhs_ty,
    &metamethod,
    location,
  );
  if mm_type.is_none() {
    mm_type = find_metatable_entry(
      Handle::from_ptr(ctx.builtins.as_ptr()),
      &mut dummy,
      rhs_ty,
      &metamethod,
      location,
    );
  }

  // 双写合一：`is_none()` 早退与随后 `unwrap()` 收为 let-else，Some 直接绑定。
  let Some(mm_type) = mm_type else {
    return TypeFunctionReductionResult::erroneous();
  };

  let mm_type = follow_type::follow(mm_type);
  // `is_pending` 为安全函数：solver 可空由内部判空折叠。
  if is_pending(mm_type, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![mm_type]);
  }

  let args_pack = ctx
    .arena_mut()
    .add_type_pack_t(TypePack::from_vec(vec![lhs_ty, rhs_ty]));

  let call_location = if !ctx.constraint.is_null() {
    alias_ref(ctx.constraint).location
  } else {
    location
  };
  // solve_function_call 为安全函数：NonNull 字段重建收口其体内；`args_pack`
  // 为已写入 arena 的存活句柄（lib.rs 不变量 2）。
  if solve_function_call(ctx, call_location, mm_type, args_pack).is_none() {
    return TypeFunctionReductionResult::erroneous();
  }

  TypeFunctionReductionResult::reduction(ctx.builtins().boolean_type)
}
