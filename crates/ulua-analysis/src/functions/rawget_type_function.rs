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
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  if type_params.len() != 2 || !pack_params.is_empty() {
    LUAU_ASSERT!(false);
  }

  // Safety: index_function_impl 的 /// # Safety 要求 ctx 为独占存活的
  // TypeFunctionContext 借用（其 arena/normalizer/solver 字段满足构造契约，且此处
  // 降级为只读下传）且切片元素为存活 TypeId——type_params/pack_params 由分派器在
  // arena 快照上构造、存活覆盖本次调用；solver 单线程驱动，调用期内无第二
  // 并发访问路径，透传借用不产生新别名。
  unsafe { index_function_impl(type_params, pack_params, ctx, true) }
}
