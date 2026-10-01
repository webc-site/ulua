use crate::{
  functions::{follow_type, is_pending::is_pending},
  macros::check_arity,
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn not_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 1, "not");

  let ty = follow_type::follow(type_params[0]);
  if ty == instance {
    return TypeFunctionReductionResult::reduction(ctx.builtins().never_type);
  }

  // `is_pending` 为安全函数：solver 可空由内部判空折叠，ty 是 follow 后的存活 arena 句柄。
  if is_pending(ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![ty]);
  }

  TypeFunctionReductionResult::reduction(ctx.builtins().boolean_type)
}
