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
/// 体全 safe：契约已由各收口门面承担（见 `reduce_and_or_type_function`）。
pub fn or_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // ctx/参数切片与 or 形态错误消息原样透传给 and/or 共享核心。
  reduce_and_or_type_function(
    instance,
    type_params,
    pack_params,
    ctx,
    true,
    "or type function: encountered a type function instance without the required argument structure",
  )
}
