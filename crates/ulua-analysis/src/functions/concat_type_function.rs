use alloc::vec::Vec;

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::FFlag;

use crate::{
  enums::reduction::Reduction,
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry,
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_pending::is_pending,
    solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  records::{
    never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn concat_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let ctx_ref = unsafe { &*ctx };
  if type_params.len() != 2 || !pack_params.is_empty() {
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string("concat type function: encountered a type function instance without the required argument structure")
    };
    ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
  }

  let lhs_ty = follow_type_id(type_params[0]);
  let rhs_ty = follow_type_id(type_params[1]);

  if lhs_ty == instance || rhs_ty == instance {
    return TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).never_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  if unsafe { is_pending(lhs_ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::from([lhs_ty]),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  } else if unsafe { is_pending(rhs_ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::from([rhs_ty]),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  let norm_lhs_ty_ref = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(lhs_ty) };
  let norm_rhs_ty_ref = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(rhs_ty) };

  if norm_lhs_ty_ref.should_suppress_errors() || norm_rhs_ty_ref.should_suppress_errors() {
    return TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).any_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  if get_type_id::<NeverType>(lhs_ty).as_ref().is_some()
    || get_type_id::<NeverType>(rhs_ty).as_ref().is_some()
  {
    return TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).never_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  if (norm_lhs_ty_ref.is_subtype_of_string() || norm_lhs_ty_ref.is_exactly_number())
    && (norm_rhs_ty_ref.is_subtype_of_string() || norm_rhs_ty_ref.is_exactly_number())
  {
    return TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).string_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  if let Some(result) = unsafe {
    try_distribute_type_function_app(
      &mut |instance, type_params, pack_params, ctx| {
        concat_type_function(instance, type_params, pack_params, ctx)
      },
      instance,
      &type_params,
      &pack_params,
      ctx,
    )
  } {
    return result;
  }

  let mut dummy: ErrorVec = Vec::new();
  let mut mm_type = find_metatable_entry(
    ctx_ref.builtins.as_ptr(),
    &mut dummy,
    lhs_ty,
    "__concat",
    Location::new(Position::default(), Position::default()),
  );
  let mut reversed = false;
  if mm_type.is_none() {
    mm_type = find_metatable_entry(
      ctx_ref.builtins.as_ptr(),
      &mut dummy,
      rhs_ty,
      "__concat",
      Location::new(Position::default(), Position::default()),
    );
    reversed = true;
  }

  if mm_type.is_none() {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::Erroneous,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  let mm_type_ref = follow_type_id(mm_type.unwrap());
  if unsafe { is_pending(mm_type_ref, ctx_ref.solver) } {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::from([mm_type_ref]),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  let mut inferred_args: Vec<TypeId> = Vec::new();
  if !reversed {
    inferred_args.push(lhs_ty);
    inferred_args.push(rhs_ty);
  } else {
    inferred_args.push(rhs_ty);
    inferred_args.push(lhs_ty);
  }

  if FFlag::LuauConcatDoesntAlwaysReturnString.get() {
    let ret_pack = unsafe {
      solve_function_call(
        ctx,
        if !ctx_ref.constraint.is_null() {
          (*ctx_ref.constraint).location
        } else {
          Location::new(Position::default(), Position::default())
        },
        mm_type_ref,
        (*ctx_ref.arena.as_ptr())
          .add_type_pack_vector_type_id_optional_type_pack_id(inferred_args, None),
      )
    };
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

    let extracted = unsafe {
      extend_type_pack(
        &mut *ctx_ref.arena.as_ptr(),
        ctx_ref.builtins.as_ptr(),
        ret_pack.unwrap(),
        1,
        Vec::new(),
      )
    };
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
  } else {
    if unsafe {
      solve_function_call(
        ctx,
        if !ctx_ref.constraint.is_null() {
          (*ctx_ref.constraint).location
        } else {
          Location::new(Position::default(), Position::default())
        },
        mm_type_ref,
        (*ctx_ref.arena.as_ptr())
          .add_type_pack_vector_type_id_optional_type_pack_id(inferred_args, None),
      )
    }
    .is_none()
    {
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
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).string_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
}
