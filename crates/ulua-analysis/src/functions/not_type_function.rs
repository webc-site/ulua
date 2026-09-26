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
  // Safety: `ctx` 由类型函数派发器从会话栈上独占上下文物化（C++ reducer 契约的
  // NotNull 语境）；本函数只共享读取其字段，降级为只读借用跨全函数使用。
  let ctx_ref = &*ctx;
  if type_params.len() != 1 || !pack_params.is_empty() {
    // Safety: `ctx_ref.ice` 为构造期接线的 NotNull `NonNull<InternalErrorReporter>`，
    // 非空且比本上下文长寿；ice_string 仅顺序上报字符串。
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string("not type function: encountered a type function instance without the required argument structure")
    };
    LUAU_ASSERT!(false);
  }

  let ty = follow_type::follow(type_params[0]);
  if ty == instance {
    // Safety: `ctx_ref.builtins` 是 NotNull 语义的会话级 `BuiltinTypes` 单例
    // 句柄，构造期接线非空、内容只读且比本调用长寿，as_ref 重建共享借用
    // 无并发可变访问。
    return TypeFunctionReductionResult::reduction(unsafe { ctx_ref.builtins.as_ref().never_type });
  }

  // Safety: `is_pending` 为 unsafe fn，契约是 `solver` 为空或指向存活
  // ConstraintSolver——本端口的 `ctx_ref.solver` 正是 C++ 可空裸指针直译，其内
  // 部 `solver.as_mut()` 自带判空；`ty` 是 follow 后的存活 arena 类型句柄。
  if unsafe { is_pending(ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![ty]);
  }

  // Safety: 同 never_type 分支——`ctx_ref.builtins` 为非空存活的 NotNull
  // 单例句柄，只读 boolean_type。
  TypeFunctionReductionResult::reduction(unsafe { ctx_ref.builtins.as_ref().boolean_type })
}
