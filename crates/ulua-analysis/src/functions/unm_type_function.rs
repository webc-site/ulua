use alloc::{vec, vec::Vec};

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::reduction::Reduction,
  functions::{
    find_metatable_entry::find_metatable_entry, first::first, follow_type::follow_type_id,
    get_type_alt_j::get_type_id, is_pending::is_pending, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  records::{
    never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
fn empty_location() -> Location {
  Location::new(
    Position { line: 0, column: 0 },
    Position { line: 0, column: 0 },
  )
}

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn unm_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let ctx_ref = unsafe { &*ctx };
  if type_params.len() != 1 || !pack_params.is_empty() {
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string("unm type function: encountered a type function instance without the required argument structure")
    };
    LUAU_ASSERT!(false);
  }

  let mut operand_ty = follow_type_id(type_params[0]);

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

  // check to see if the operand type is resolved enough, and wait to reduce if not
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

  operand_ty = follow_type_id(operand_ty);

  let norm_ty = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(operand_ty) };

  // NOTE: in C++ a failed normalization yields a null shared_ptr; the Rust
  // normalizer returns an owning handle, so we treat `should_suppress_errors`
  // and the subsequent checks directly, mirroring the inhabited/suppress flow.

  // if the operand is error suppressing, we can just go ahead and reduce.
  if (*norm_ty).should_suppress_errors() {
    return TypeFunctionReductionResult {
      result: Some(operand_ty),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  // if we have a `never`, we can never observe that the operation didn't work.
  if !get_type_id::<NeverType>(operand_ty).is_none() {
    return TypeFunctionReductionResult {
      result: Some(unsafe { ctx_ref.builtins.as_ref().never_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  // If the type is exactly `number`, we can reduce now.
  if (*norm_ty).is_exactly_number() {
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
      &mut |i, tp, pp, c| unm_type_function(i, tp, pp, c),
      instance,
      &type_params,
      &pack_params,
      ctx,
    )
  } {
    return result;
  }

  // findMetatableEntry demands the ability to emit errors, so we must give it
  // the necessary state to do that, even if we intend to just eat the errors.
  let mut dummy: ErrorVec = vec![];

  let mm_type = find_metatable_entry(
    ctx_ref.builtins.as_ptr(),
    &mut dummy,
    operand_ty,
    "__unm",
    empty_location(),
  );
  if mm_type.is_none() {
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

  let location = if !ctx_ref.constraint.is_null() {
    unsafe { (*ctx_ref.constraint).location }
  } else {
    empty_location()
  };

  let result = unsafe { solve_function_call(ctx, location, mm_type_followed, args_pack) };
  let result = match result {
    Some(r) => r,
    None => {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
      };
    }
  };

  if let Some(ret) = first(result, true) {
    TypeFunctionReductionResult {
      result: Some(ret),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    }
  } else {
    TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::Erroneous,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    }
  }
}
