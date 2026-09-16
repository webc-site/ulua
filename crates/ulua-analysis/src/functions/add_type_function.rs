use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::numeric_binop_type_function::numeric_binop_type_function,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn add_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  let _ctx_ref = unsafe { &*ctx };
  if type_params.len() != 2 || !pack_params.is_empty() {
    LUAU_ASSERT!(false);
  }

  unsafe {
    numeric_binop_type_function(instance, type_params, pack_params, ctx, "__add".to_string())
  }
}
