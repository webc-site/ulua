use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::index_function_impl::index_function_impl,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn rawget_type_function(
  _instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let _ctx_ref = unsafe { &*ctx };
  if type_params.len() != 2 || !pack_params.is_empty() {
    LUAU_ASSERT!(false);
  }

  unsafe { index_function_impl(type_params, pack_params, ctx, true) }
}
