use core::ptr::NonNull;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::reduction::Reduction,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_nil::is_nil,
    is_pending::is_pending,
  },
  records::{
    negation_type::NegationType, singleton_type::SingletonType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn singleton_type_function(
  _instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // SAFETY: reducer 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效。
  // SAFETY: 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效；
  // NonNull 封装解引用。
  let ctx_ref = unsafe { NonNull::new_unchecked(ctx).as_ref() };
  if type_params.len() != 1 || !pack_params.is_empty() {
    unsafe { ctx_ref.ice.as_ref() }.ice_string("singleton type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let ty = follow_type_id(type_params[0]);
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

  let mut followed = ty;
  if let Some(negation) = get_type_id::<NegationType>(followed) {
    followed = follow_type_id(negation.ty);
  }

  if get_type_id::<SingletonType>(followed).is_some() || is_nil(followed) {
    return TypeFunctionReductionResult {
      result: Some(ty),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  TypeFunctionReductionResult {
    result: Some(unsafe { ctx_ref.builtins.as_ref() }.unknown_type),
    reduction_status: Reduction::MaybeOk,
    blocked_types: vec![],
    blocked_packs: vec![],
    error: None,
    messages: vec![],
  }
}
