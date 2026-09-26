use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type, is_pending::is_pending},
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn not_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  if type_params.len() != 1 || !pack_params.is_empty() {
    ctx.ice().ice_string("not type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let ty = follow_type::follow(type_params[0]);
  if ty == instance {
    return TypeFunctionReductionResult::reduction(ctx.builtins().never_type);
  }

  // Safety: `is_pending` 为 unsafe fn，契约是 `solver` 为空或指向存活
  // ConstraintSolver——本端口的 `ctx.solver` 正是 C++ 可空裸指针直译，其内
  // 部 `solver.as_mut()` 自带判空；`ty` 是 follow 后的存活 arena 类型句柄。
  if unsafe { is_pending(ty, ctx.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![ty]);
  }

  TypeFunctionReductionResult::reduction(ctx.builtins().boolean_type)
}
