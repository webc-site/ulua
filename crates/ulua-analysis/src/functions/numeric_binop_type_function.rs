use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;

use crate::{
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry, follow_type,
    get_mutable_type_pack, get_type, is_pending::is_pending,
    solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  macros::check_arity,
  records::{
    arena_handle::Handle, never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn numeric_binop_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
  metamethod: String,
) -> TypeFunctionReductionResult {
  unsafe {
    // SAFETY: 分派点物化的本次调用独占借用；再取可变再借用供整段归约派生句柄。
    let ctx_ref = &mut *ctx;
    check_arity!(ctx_ref, type_params, pack_params, 2);

    let lhs_ty = follow_type::follow(type_params[0]);
    let rhs_ty = follow_type::follow(type_params[1]);

    if lhs_ty == instance || rhs_ty == instance {
      return TypeFunctionReductionResult::reduction(ctx_ref.builtins.as_ref().never_type);
    }

    if get_type::get::<NeverType>(lhs_ty).is_some() || get_type::get::<NeverType>(rhs_ty).is_some()
    {
      return TypeFunctionReductionResult::reduction(ctx_ref.builtins.as_ref().never_type);
    }

    let location = if !ctx_ref.constraint.is_null() {
      // SAFETY: constraint 非空时指向会话期 Constraint。
      (*ctx_ref.constraint).location
    } else {
      Location::new(Default::default(), Default::default())
    };

    if is_pending(lhs_ty, ctx_ref.solver) {
      return TypeFunctionReductionResult::no_reduction(vec![lhs_ty]);
    } else if is_pending(rhs_ty, ctx_ref.solver) {
      return TypeFunctionReductionResult::no_reduction(vec![rhs_ty]);
    }

    // SAFETY: normalizer 由 ctx 持有，NonNull 契约会话期有效。
    let norm_lhs_ty = ctx_ref.normalizer.as_mut().try_normalize(lhs_ty);
    let norm_rhs_ty = ctx_ref.normalizer.as_mut().try_normalize(rhs_ty);

    let (Some(norm_lhs_ty), Some(norm_rhs_ty)) = (norm_lhs_ty, norm_rhs_ty) else {
      return TypeFunctionReductionResult::no_reduction(Vec::new());
    };

    // if one of the types is error suppressing, we can reduce to `any` since we should
    // suppress errors in the result of the usage.
    if norm_lhs_ty.should_suppress_errors() || norm_rhs_ty.should_suppress_errors() {
      return TypeFunctionReductionResult::reduction(ctx_ref.builtins.as_ref().any_type);
    }

    // if we're adding two `number` types, the result is `number`.
    if norm_lhs_ty.is_exactly_number() && norm_rhs_ty.is_exactly_number() {
      return TypeFunctionReductionResult::reduction(ctx_ref.builtins.as_ref().number_type);
    }

    if let Some(result) = try_distribute_type_function_app(
      |instance, type_params, pack_params, ctx| {
        numeric_binop_type_function(instance, type_params, pack_params, ctx, metamethod.clone())
      },
      instance,
      type_params,
      pack_params,
      &mut *ctx_ref,
    ) {
      return result;
    }

    let mut dummy: ErrorVec = Vec::new();

    // lhs 未命中元表项才回退查 rhs 并翻转实参序（cpp 同位 `reversed` 跟踪）。
    let (mm_type_opt, reversed) = match find_metatable_entry(
      Handle::from_ptr(ctx_ref.builtins.as_ptr()),
      &mut dummy,
      lhs_ty,
      &metamethod,
      location,
    ) {
      Some(mm_type) => (Some(mm_type), false),
      None => (
        find_metatable_entry(
          Handle::from_ptr(ctx_ref.builtins.as_ptr()),
          &mut dummy,
          rhs_ty,
          &metamethod,
          location,
        ),
        true,
      ),
    };

    // 双写合一：`is_none()` 早退与随后 `expect()` 收为 let-else（与 comparison 同构）。
    let Some(mm_type) = mm_type_opt else {
      return TypeFunctionReductionResult::erroneous();
    };

    let mm_type = follow_type::follow(mm_type);

    if is_pending(mm_type, ctx_ref.solver) {
      return TypeFunctionReductionResult::no_reduction(vec![mm_type]);
    }

    let arg_pack = ctx_ref
      .arena
      .as_mut()
      .add_type_pack_initializer_list_type_id(&[lhs_ty, rhs_ty]);

    if reversed
      && let Some(pack_ref) = get_mutable_type_pack::get_mutable::<TypePack>(arg_pack)
      && pack_ref.head.len() >= 2
    {
      pack_ref.head.swap(0, 1);
    }

    // 双写合一：同上，`is_none()` 早退与 `expect()` 收为 let-else。
    let Some(ret_pack_val) = solve_function_call(&*ctx_ref, location, mm_type, arg_pack) else {
      return TypeFunctionReductionResult::erroneous();
    };
    let extracted = extend_type_pack(
      ctx_ref.arena.as_mut(),
      Handle::from_ptr(ctx_ref.builtins.as_ptr()),
      ret_pack_val,
      1,
      Vec::new(),
    );

    if extracted.head.is_empty() {
      return TypeFunctionReductionResult::erroneous();
    }

    TypeFunctionReductionResult::reduction(extracted.head[0])
  }
}
