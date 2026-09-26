use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{follow_type, get_type, is_pending::is_pending, is_prim::is_nil},
  records::{
    negation_type::NegationType, singleton_type::SingletonType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn singleton_type_function(
  _instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  if type_params.len() != 1 || !pack_params.is_empty() {
    ctx.ice().ice_string("singleton type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let ty = follow_type::follow(type_params[0]);
  // Safety: solver 为 ctx 构造期接线的裸指针（C++ `ConstraintSolver*` 可空形参，
  // from_solver 路径非空、纯归约路径为 null 均合法）；is_pending 内部以
  // `solver.as_mut()` 判空并仅解引用存活 TypeId——ty 已经 follow_type_id 收敛到
  // arena 存活节点，单线程读无别名。
  if unsafe { is_pending(ty, ctx.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![ty]);
  }

  let mut followed = ty;
  if let Some(negation) = get_type::get::<NegationType>(followed) {
    followed = follow_type::follow(negation.ty);
  }

  if get_type::get::<SingletonType>(followed).is_some() || is_nil(followed) {
    return TypeFunctionReductionResult::reduction(ty);
  }

  TypeFunctionReductionResult::reduction(ctx.builtins().unknown_type)
}
