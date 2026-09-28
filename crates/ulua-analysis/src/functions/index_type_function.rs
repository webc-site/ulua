use crate::{
  functions::index_function_impl::index_function_impl,
  macros::check_arity,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn index_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(type_params, pack_params, 2, assert_only);

  // Safety: 逐条满足 `index_function_impl` 的 fn 级契约——`ctx` 为分派帧物化的
  // 独占存活借用（此处降级为只读下传，对应 C++ NotNull 按值转递）、
  // `type_params` 为该 index 类型函数声明的 [indexee, indexer] 二元参数、元素是存活
  // arena TypeId，`pack_params` 为空（callee 不读取）；单线程驱动栈帧内无并发访问。
  unsafe { index_function_impl(type_params, pack_params, ctx, false) }
}
