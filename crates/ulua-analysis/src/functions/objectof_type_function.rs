use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type, get_type, is_pending::is_pending},
  records::{
    extern_type::ExternType, obj::Obj, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// # Safety
/// `ctx` 为分派点物化的本次调用独占借用，指向在本次 reducer 调用期间存活的
/// `TypeFunctionContext`；`type_params`/`pack_params` 为指向会话 arena 存活节点的
/// 句柄切片，且 `type_params.len() == 1`、`pack_params` 为空（C++
/// `ReducerFunction<TypeId>` 契约）。
pub unsafe fn objectof_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  if type_params.len() != 1 || !pack_params.is_empty() {
    ctx.ice().ice_string("objectof type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let target_ty = follow_type::follow(type_params[0]);
  // Safety: is_pending 要求 ty 为 arena 存活句柄、solver 为可空裸指针——
  // target_ty 由 follow_type_id 从入参 type_params（契约内 arena 句柄）导出；
  // ctx.solver 是 ctx 持有的可空 ConstraintSolver 句柄（旧 solver 下为
  // null），is_pending 内部以 as_mut 判空，与 C++ isPending(ty, solver) 同语义。
  if unsafe { is_pending(target_ty, ctx.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![target_ty]);
  }

  if let Some(klass) = get_type::get::<ExternType>(target_ty)
    && let Some(ref relation) = klass.relation
    && let Some(obj) = relation.get_if::<Obj>()
  {
    return TypeFunctionReductionResult::reduction(obj.ty);
  }

  TypeFunctionReductionResult::reduction(ctx.builtins().error_type)
}
