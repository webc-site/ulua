use alloc::vec::Vec;

use ulua_analysis::{
  enums::reduction::Reduction,
  functions::{
    follow_type::follow_type_id, is_number::is_number, is_pending::is_pending, is_string::is_string,
  },
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

pub(crate) fn type_function_fixture_swap_reducer(
  _instance: TypeId,
  tys: Vec<TypeId>,
  tps: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  LUAU_ASSERT!(tys.len() == 1);
  LUAU_ASSERT!(tps.is_empty());

  let ctx_ref = unsafe { &*ctx };
  let param = follow_type_id(tys[0]);

  if is_string(param) {
    TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).number_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  } else if is_number(param) {
    TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).string_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  } else if unsafe { is_pending(param, ctx_ref.solver) } {
    TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: alloc::vec![param],
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  } else {
    TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::Erroneous,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
}
