use ulua_ast::records::{location::Location, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{normalization_result::NormalizationResult, reduction::Reduction},
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type::follow_type_id,
    get_type_alt_j::get_type_id, is_pending::is_pending, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  records::{
    metatable_type::MetatableType, table_type::TableType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn len_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let ctx_ref = unsafe { &*ctx };
  if type_params.len() != 1 || !pack_params.is_empty() {
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string("len type function: encountered a type function instance without the required argument structure")
    };
    LUAU_ASSERT!(false);
  }

  let operand_ty = follow_type_id(type_params[0]);

  if operand_ty == instance {
    return TypeFunctionReductionResult {
      result: Some(unsafe { ctx_ref.builtins.as_ref().never_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  if unsafe { is_pending(operand_ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![operand_ty],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  let norm_ty = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(operand_ty) };
  let inhabited =
    unsafe { (*ctx_ref.normalizer.as_ptr()).is_inhabited_normalized_type(norm_ty.as_ref()) };

  if inhabited == NormalizationResult::HitLimits {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  if (*norm_ty).should_suppress_errors() {
    return TypeFunctionReductionResult {
      result: Some(unsafe { ctx_ref.builtins.as_ref().number_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  if inhabited == NormalizationResult::False || { (*norm_ty).is_subtype_of_string() } {
    return TypeFunctionReductionResult {
      result: Some(unsafe { ctx_ref.builtins.as_ref().number_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  let normalized_operand =
    unsafe { follow_type_id((*ctx_ref.normalizer.as_ptr()).type_from_normal(norm_ty.as_ref())) };

  if { (*norm_ty).has_top_table() } || !get_type_id::<TableType>(normalized_operand).is_none() {
    return TypeFunctionReductionResult {
      result: Some(unsafe { ctx_ref.builtins.as_ref().number_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  if let Some(result) = unsafe {
    try_distribute_type_function_app(
      &mut |i, tp, pp, c| len_type_function(i, tp, pp, c),
      instance,
      &type_params,
      &pack_params,
      ctx,
    )
  } {
    return result;
  }

  let mut dummy: ErrorVec = vec![];
  let mm_type = find_metatable_entry(
    ctx_ref.builtins.as_ptr(),
    &mut dummy,
    operand_ty,
    "__len",
    Location::new(
      Position { line: 0, column: 0 },
      Position { line: 0, column: 0 },
    ),
  );

  if mm_type.is_none() {
    if !get_type_id::<MetatableType>(normalized_operand).is_none() {
      return TypeFunctionReductionResult {
        result: Some(unsafe { ctx_ref.builtins.as_ref().number_type }),
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
      };
    }

    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::Erroneous,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  let mm_type_followed = follow_type_id(mm_type.unwrap());
  if unsafe { is_pending(mm_type_followed, ctx_ref.solver) } {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![mm_type_followed],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  let args_pack = unsafe {
    (*ctx_ref.arena.as_ptr()).add_type_pack_t(TypePack {
      head: vec![operand_ty],
      tail: None,
    })
  };

  if unsafe {
    solve_function_call(
      ctx,
      if !ctx_ref.constraint.is_null() {
        (*ctx_ref.constraint).location
      } else {
        Location::new(
          Position { line: 0, column: 0 },
          Position { line: 0, column: 0 },
        )
      },
      mm_type_followed,
      args_pack,
    )
  }
  .is_none()
  {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::Erroneous,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  TypeFunctionReductionResult {
    result: Some(unsafe { ctx_ref.builtins.as_ref().number_type }),
    reduction_status: Reduction::MaybeOk,
    blocked_types: vec![],
    blocked_packs: vec![],
    error: None,
    messages: vec![],
  }
}
