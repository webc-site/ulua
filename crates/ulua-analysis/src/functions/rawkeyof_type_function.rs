use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::keyof_function_impl::keyof_function_impl,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub(crate) fn rawkeyof_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  if type_params.len() != 1 || !pack_params.is_empty() {
    LUAU_ASSERT!(false);
  }

  // Safety: keyof_function_impl 的 fn 级契约（ctx 指向本次调用独占存活的
  // TypeFunctionContext、切片元素为存活 arena 句柄）由分派器帧保证，此处把
  // 独占借用降级为只读下传，不并存第二可变别名。
  unsafe { keyof_function_impl(type_params, pack_params, ctx, true) }
}
