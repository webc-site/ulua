use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{normalization_result::NormalizationResult, reduction::Reduction},
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_pending::is_pending},
  records::{
    never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn weakoptional_type_func(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let ctx_ref = unsafe { &*ctx };
  if type_params.len() != 1 || !pack_params.is_empty() {
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string("weakoptional type function: encountered a type function instance without the required argument structure")
    };
    LUAU_ASSERT!(false);
  }

  let target_ty = follow_type_id(type_params[0]);

  if unsafe { is_pending(target_ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: alloc::vec![target_ty],
      blocked_packs: alloc::vec![],
      error: None,
      messages: alloc::vec![],
    };
  }

  if get_type_id::<NeverType>(instance).as_ref().is_some() {
    return TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).nil_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: alloc::vec![],
      blocked_packs: alloc::vec![],
      error: None,
      messages: alloc::vec![],
    };
  }

  let target_norm = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(target_ty) };

  let result =
    unsafe { (*ctx_ref.normalizer.as_ptr()).is_inhabited_normalized_type(target_norm.as_ref()) };
  if result == NormalizationResult::False {
    return TypeFunctionReductionResult {
      result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).nil_type }),
      reduction_status: Reduction::MaybeOk,
      blocked_types: alloc::vec![],
      blocked_packs: alloc::vec![],
      error: None,
      messages: alloc::vec![],
    };
  }

  TypeFunctionReductionResult {
    result: Some(target_ty),
    reduction_status: Reduction::MaybeOk,
    blocked_types: alloc::vec![],
    blocked_packs: alloc::vec![],
    error: None,
    messages: alloc::vec![],
  }
}
