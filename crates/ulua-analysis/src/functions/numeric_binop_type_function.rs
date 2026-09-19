use alloc::{string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::reduction::Reduction,
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry,
    follow_type::follow_type_id, get_mutable_type_pack::get_mutable_type_pack_id,
    get_type_alt_j::get_type_id, is_pending::is_pending, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  records::{
    never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn numeric_binop_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
  metamethod: String,
) -> TypeFunctionReductionResult {
  unsafe {
    // SAFETY: reducer 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效。
    // SAFETY: 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效；
    // NonNull 封装解引用。
    let ctx_ref = NonNull::new_unchecked(ctx).as_mut();
    if type_params.len() != 2 || !pack_params.is_empty() {
      ctx_ref
        .ice
        .as_ref()
        .ice_string("encountered a type function instance without the required argument structure");
      LUAU_ASSERT!(false);
    }

    let lhs_ty = follow_type_id(type_params[0]);
    let rhs_ty = follow_type_id(type_params[1]);

    if lhs_ty == instance || rhs_ty == instance {
      return TypeFunctionReductionResult {
        result: Some(ctx_ref.builtins.as_ref().never_type),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    if get_type_id::<NeverType>(lhs_ty).is_some() || get_type_id::<NeverType>(rhs_ty).is_some() {
      return TypeFunctionReductionResult {
        result: Some(ctx_ref.builtins.as_ref().never_type),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    let location = if !ctx_ref.constraint.is_null() {
      // SAFETY: constraint 非空时指向会话期 Constraint。
      (*ctx_ref.constraint).location
    } else {
      Location::new(Default::default(), Default::default())
    };

    if is_pending(lhs_ty, ctx_ref.solver) {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![lhs_ty],
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    } else if is_pending(rhs_ty, ctx_ref.solver) {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![rhs_ty],
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    // SAFETY: normalizer 由 ctx 持有，NonNull 契约会话期有效。
    let norm_lhs_ty = ctx_ref.normalizer.as_mut().try_normalize(lhs_ty);
    let norm_rhs_ty = ctx_ref.normalizer.as_mut().try_normalize(rhs_ty);

    let (Some(norm_lhs_ty), Some(norm_rhs_ty)) = (norm_lhs_ty, norm_rhs_ty) else {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    };

    // if one of the types is error suppressing, we can reduce to `any` since we should
    // suppress errors in the result of the usage.
    if norm_lhs_ty.should_suppress_errors() || norm_rhs_ty.should_suppress_errors() {
      return TypeFunctionReductionResult {
        result: Some(ctx_ref.builtins.as_ref().any_type),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    // if we're adding two `number` types, the result is `number`.
    if norm_lhs_ty.is_exactly_number() && norm_rhs_ty.is_exactly_number() {
      return TypeFunctionReductionResult {
        result: Some(ctx_ref.builtins.as_ref().number_type),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    if let Some(result) = try_distribute_type_function_app(
      |instance, type_params, pack_params, ctx| {
        numeric_binop_type_function(instance, type_params, pack_params, ctx, metamethod.clone())
      },
      instance,
      &type_params,
      &pack_params,
      ctx,
    ) {
      return result;
    }

    let mut dummy: ErrorVec = Vec::new();

    let mm_type = find_metatable_entry(
      ctx_ref.builtins.as_ptr(),
      &mut dummy,
      lhs_ty,
      &metamethod,
      location,
    );

    let mut reversed = false;
    let mut mm_type_opt = mm_type;

    if mm_type_opt.is_none() {
      mm_type_opt = find_metatable_entry(
        ctx_ref.builtins.as_ptr(),
        &mut dummy,
        rhs_ty,
        &metamethod,
        location,
      );
      reversed = true;
    }

    if mm_type_opt.is_none() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    let mm_type = follow_type_id(mm_type_opt.unwrap());

    if is_pending(mm_type, ctx_ref.solver) {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![mm_type],
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    let arg_pack = ctx_ref
      .arena
      .as_mut()
      .add_type_pack_initializer_list_type_id(&[lhs_ty, rhs_ty]);

    if reversed
      && let Some(pack_ref) = get_mutable_type_pack_id::<TypePack>(arg_pack)
      && pack_ref.head.len() >= 2
    {
      pack_ref.head.swap(0, 1);
    }

    let ret_pack = solve_function_call(ctx, location, mm_type, arg_pack);

    if ret_pack.is_none() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    let ret_pack_val = ret_pack.unwrap();
    let extracted = extend_type_pack(
      ctx_ref.arena.as_mut(),
      ctx_ref.builtins.as_ptr(),
      ret_pack_val,
      1,
      Vec::new(),
    );

    if extracted.head.is_empty() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
      };
    }

    TypeFunctionReductionResult {
      result: Some(extracted.head[0]),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
}
