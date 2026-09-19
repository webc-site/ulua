use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::keyof_function_impl::keyof_function_impl,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn keyof_type_function(
  _instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let _ctx_ref = unsafe { &*ctx };
  if type_params.len() != 1 || !pack_params.is_empty() {
    LUAU_ASSERT!(false);
  }

  unsafe { keyof_function_impl(type_params, pack_params, ctx, false) }
}
