use alloc::{string::String, vec::Vec};

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    as_mutable_type::as_mutable_type_id, find_metatable_entry::find_metatable_entry, follow_type,
    get_type, is_blocked_or_unsolved_type::is_blocked_or_unsolved_type, is_pending::is_pending,
    is_prim::is_number, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type,
  records::{
    arena_handle::Handle, free_type::FreeType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 作为 `ReducerFunction` 由 TypeFunctionReducer 调用，或由 lt/le 等比较类内建函数与自身
/// 递归闭包转发（cpp `comparisonTypeFunction`，`cpp/Analysis/src/BuiltinTypeFunctions.cpp:778`）：
/// `ctx` 为分派点物化的本次调用独占借用，整个 reduce 步进期存活（体内解引用其 ice/builtins/normalizer/arena 读内建
/// 类型、写归一化缓存并追加类型 pack）；`type_params` 须恰 2 项、`pack_params` 须为空，且两
/// 切片句柄均指向存活 arena 节点，否则解引用与写回皆为 UB。
pub unsafe fn comparison_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
  metamethod: String,
) -> TypeFunctionReductionResult {
  // Safety: ctx 是约束派发期存活的 TypeFunctionContext 的独占借用（C++ NotNull 语境），
  // 函数头一次 `&mut *ctx` 再借用全程派生句柄，单线程内无并存其它借用；ice/builtins/normalizer/arena
  // 是构造期接线的 NonNull 字段，as_ptr 非空且指向比 ctx 长寿的目标；solver/constraint
  // 先判空才解引用或传用；两处 emplace 的 &mut 指向类型 arena 稳定节点，两分支互斥、
  // 借用瞬时结束；错误路径 ice 调用后 LUAU_ASSERT(false) 与 C++ 同样不再继续。
  unsafe {
    let ctx_ref = &mut *ctx;
    if type_params.len() != 2 || !pack_params.is_empty() {
      (*ctx_ref.ice.as_ptr())
        .ice_string("encountered a type function instance without the required argument structure");
      LUAU_ASSERT!(false);
    }

    let lhs_ty = follow_type::follow(type_params[0]);
    let rhs_ty = follow_type::follow(type_params[1]);

    if lhs_ty == instance || rhs_ty == instance {
      return TypeFunctionReductionResult::reduction((*ctx_ref.builtins.as_ptr()).never_type);
    }

    if is_blocked_or_unsolved_type(lhs_ty) {
      return TypeFunctionReductionResult::no_reduction(vec![lhs_ty]);
    } else if is_blocked_or_unsolved_type(rhs_ty) {
      return TypeFunctionReductionResult::no_reduction(vec![rhs_ty]);
    }

    let can_submit_constraint = !ctx_ref.solver.is_null() && !ctx_ref.constraint.is_null();
    let lhs_free = get_type::get::<FreeType>(lhs_ty).as_ref().is_some();
    let rhs_free = get_type::get::<FreeType>(rhs_ty).as_ref().is_some();

    if can_submit_constraint {
      if lhs_free && is_number(rhs_ty) {
        let mut number_type = (*ctx_ref.builtins.as_ptr()).number_type;
        unifiable_bound_type_id_emplace_type_bound_type(
          &mut *as_mutable_type_id(lhs_ty),
          &mut number_type,
        );
      } else if rhs_free && is_number(lhs_ty) {
        let mut number_type = (*ctx_ref.builtins.as_ptr()).number_type;
        unifiable_bound_type_id_emplace_type_bound_type(
          &mut *as_mutable_type_id(rhs_ty),
          &mut number_type,
        );
      }
    }

    let lhs_ty = follow_type::follow(lhs_ty);
    let rhs_ty = follow_type::follow(rhs_ty);

    // C++ 中归一化失败返回空指针：不归约，对驻留性一无所知
    let (Some(norm_lhs_ty), Some(norm_rhs_ty)) = (
      (*ctx_ref.normalizer.as_ptr()).try_normalize(lhs_ty),
      (*ctx_ref.normalizer.as_ptr()).try_normalize(rhs_ty),
    ) else {
      return TypeFunctionReductionResult::no_reduction(Vec::new());
    };
    let lhs_inhabited =
      (*ctx_ref.normalizer.as_ptr()).is_inhabited_normalized_type(norm_lhs_ty.as_ref());
    let rhs_inhabited =
      (*ctx_ref.normalizer.as_ptr()).is_inhabited_normalized_type(norm_rhs_ty.as_ref());

    if lhs_inhabited == NormalizationResult::HitLimits
      || rhs_inhabited == NormalizationResult::HitLimits
    {
      return TypeFunctionReductionResult::no_reduction(Vec::new());
    }

    if norm_lhs_ty.should_suppress_errors() || norm_rhs_ty.should_suppress_errors() {
      return TypeFunctionReductionResult::reduction((*ctx_ref.builtins.as_ptr()).boolean_type);
    }

    if lhs_inhabited == NormalizationResult::False || rhs_inhabited == NormalizationResult::False {
      return TypeFunctionReductionResult::reduction((*ctx_ref.builtins.as_ptr()).boolean_type);
    }

    if norm_lhs_ty.is_subtype_of_string() && norm_rhs_ty.is_subtype_of_string() {
      return TypeFunctionReductionResult::reduction((*ctx_ref.builtins.as_ptr()).boolean_type);
    }

    if norm_lhs_ty.is_exactly_number() && norm_rhs_ty.is_exactly_number() {
      return TypeFunctionReductionResult::reduction((*ctx_ref.builtins.as_ptr()).boolean_type);
    }

    if let Some(result) = try_distribute_type_function_app(
      |instance, type_params, pack_params, ctx| {
        comparison_type_function(instance, type_params, pack_params, ctx, metamethod.clone())
      },
      instance,
      type_params,
      pack_params,
      &mut *ctx_ref,
    ) {
      return result;
    }

    let mut dummy: ErrorVec = Vec::new();
    let location = Location::new(Position::default(), Position::default());

    let mut mm_type = find_metatable_entry(
      Handle::from_ptr(ctx_ref.builtins.as_ptr()),
      &mut dummy,
      lhs_ty,
      &metamethod,
      location,
    );
    if mm_type.is_none() {
      mm_type = find_metatable_entry(
        Handle::from_ptr(ctx_ref.builtins.as_ptr()),
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
    if is_pending(mm_type, ctx_ref.solver) {
      return TypeFunctionReductionResult::no_reduction(vec![mm_type]);
    }

    let args_pack =
      (*ctx_ref.arena.as_ptr()).add_type_pack_t(TypePack::from_vec(vec![lhs_ty, rhs_ty]));

    let call_location = if !ctx_ref.constraint.is_null() {
      (*ctx_ref.constraint).location
    } else {
      location
    };
    if solve_function_call(&*ctx_ref, call_location, mm_type, args_pack).is_none() {
      return TypeFunctionReductionResult::erroneous();
    }

    TypeFunctionReductionResult::reduction((*ctx_ref.builtins.as_ptr()).boolean_type)
  }
}
