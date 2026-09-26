use crate::{
  functions::reduce_and_or_type_function::reduce_and_or_type_function,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// 对应 C++ `orTypeFunction`（BuiltinTypeFunctions.cpp，and 的对偶）：化简
/// `or<A, B>`——吸收律短路、blocked 早退，否则走 simplifyIntersection/Union。
///
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn or_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // Safety: ctx/参数切片与 or 形态错误消息原样透传给 and/or 共享核心，其 # Safety
  // 前置条件即本函数承接的 C++ 调用契约——同一借用一次性下传，不并存第二别名。
  unsafe {
    reduce_and_or_type_function(
      instance,
      type_params,
      pack_params,
      ctx,
      true,
      "or type function: encountered a type function instance without the required argument structure",
    )
  }
}
