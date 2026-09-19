use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::comparison_type_function::comparison_type_function,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn lt_type_function(
  _instance: TypeId,
  _type_params: Vec<TypeId>,
  _pack_params: Vec<TypePackId>,
  _ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let _ctx_ref = unsafe { &*_ctx };
  if _type_params.len() != 2 || !_pack_params.is_empty() {
    LUAU_ASSERT!(false);
  }

  unsafe {
    comparison_type_function(
      _instance,
      _type_params,
      _pack_params,
      _ctx,
      "__lt".to_string(),
    )
  }
}
