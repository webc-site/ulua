use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::reduction::Reduction,
  functions::{follow_type::follow_type_id, is_pending::is_pending},
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn not_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let ctx_ref = unsafe { &*ctx };
  if type_params.len() != 1 || !pack_params.is_empty() {
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string("not type function: encountered a type function instance without the required argument structure")
    };
    LUAU_ASSERT!(false);
  }

  let ty = follow_type_id(type_params[0]);
  if ty == instance {
    return TypeFunctionReductionResult {
      result: Some(unsafe { ctx_ref.builtins.as_ref().never_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  if unsafe { is_pending(ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![ty],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  TypeFunctionReductionResult {
    result: Some(unsafe { ctx_ref.builtins.as_ref().boolean_type }),
    reduction_status: Reduction::MaybeOk,
    blocked_types: vec![],
    blocked_packs: vec![],
    error: None,
    messages: vec![],
  }
}
