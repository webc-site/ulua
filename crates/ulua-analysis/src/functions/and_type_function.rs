use crate::{
  functions::reduce_and_or_type_function::reduce_and_or_type_function,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// 对应 C++ `andTypeFunction`（BuiltinTypeFunctions.cpp:698 起）：化简
/// `and<A, B>`——吸收律短路、pending 阻塞，否则走 simplifyIntersection/Union。
///
/// # Safety
/// 本函数作为 `ReducerFunction` 函数指针由 TypeFunctionReducer 从 `self.ctx`
/// （`Handle<TypeFunctionContext>`，非空由类型编码）物化独占借调用（对应 cpp
/// `NotNull<TypeFunctionContext> ctx`），
/// 调用方须保证：`ctx` 在整个 reduce 步进期内独占存活；`type_params`/`pack_params` 及其
/// 内嵌 `TypeId`/`TypePackId` 句柄指向存活类型 arena 节点（体内经 `follow_type_id`、
/// `is_pending` 间接解引用它们）。
pub unsafe fn and_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // Safety: ctx/参数切片与 and 形态错误消息原样透传给 and/or 共享核心，其 # Safety
  // 前置条件与本函数契约逐字一致——同一借用一次性下传，不并存第二别名。
  unsafe {
    reduce_and_or_type_function(
      instance,
      type_params,
      pack_params,
      ctx,
      false,
      "and type function: encountered a type function instance without the required argument structure",
    )
  }
}
