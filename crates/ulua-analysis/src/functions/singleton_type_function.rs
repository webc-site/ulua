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
  // SAFETY: 分派点自 NonNull 物化的本次调用独占借用，会话期有效；此处仅
  // 再降级为只读借用读取 ice/solver/builtins 字段。
  let ctx_ref = &*ctx;
  if type_params.len() != 1 || !pack_params.is_empty() {
    // Safety: ice 是 NonNull<InternalErrorReporter>（构造期接线，NonNull 不变量
    // 保证非空、对齐），指向 Frontend 全程持有的报告器；as_ref 重建的共享借用
    // 仅在 ice_string 调用内使用，ctx_ref 本身在 reducer 分派栈帧内存活。
    unsafe { ctx_ref.ice.as_ref() }.ice_string("singleton type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let ty = follow_type::follow(type_params[0]);
  // Safety: solver 为 ctx 构造期接线的裸指针（C++ `ConstraintSolver*` 可空形参，
  // from_solver 路径非空、纯归约路径为 null 均合法）；is_pending 内部以
  // `solver.as_mut()` 判空并仅解引用存活 TypeId——ty 已经 follow_type_id 收敛到
  // arena 存活节点，单线程读无别名。
  if unsafe { is_pending(ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![ty]);
  }

  let mut followed = ty;
  if let Some(negation) = get_type::get::<NegationType>(followed) {
    followed = follow_type::follow(negation.ty);
  }

  if get_type::get::<SingletonType>(followed).is_some() || is_nil(followed) {
    return TypeFunctionReductionResult::reduction(ty);
  }

  // Safety: builtins 是 NonNull<BuiltinTypes>（C++ NotNull 形参的 Rust 对应，
  // 构造期从会话级 BuiltinTypes 接线，NonNull 保证非空且指向存活对象）；
  // unknown_type 是 Copy 的 TypeId 字段，此处仅只读取值，无别名冲突。
  TypeFunctionReductionResult::reduction(unsafe { ctx_ref.builtins.as_ref().unknown_type })
}
