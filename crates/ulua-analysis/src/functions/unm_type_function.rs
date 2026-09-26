use alloc::vec;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    find_metatable_entry::find_metatable_entry, first::first, follow_type, get_type,
    is_pending::is_pending, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  records::{
    arena_handle::Handle, never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 逐参数契约（对照 cpp `unmTypeFunction`，BuiltinTypeFunctions.cpp:297）：
/// - `instance`：本 type function 的实例类型 Id；与 follow 后的操作数做自指比较以截断
///   循环归约，须指向驱动本次归约的类型 arena 中存活节点。
/// - `type_params`：实参类型切片。契约要求长度恰为 1（一元取负的单一操作数）；不等长
///   时先经 `ice`+`LUAU_ASSERT!(false)` 记录内部错误（`ice_string` 以 panic 发散），其后
///   的 `type_params[0]` 索引仍依赖该形状成立。
/// - `pack_params`：打包实参切片，契约要求为空（`unm` 不接 type pack 参数）。
/// - `ctx`：本次调用的独占借用，指向整段归约期间存活的 `TypeFunctionContext`。其
///   `arena/builtins/normalizer/ice/solver/constraint` 是求解会话持有的
///   NonNull/*mut/*const 句柄，本函数按 cpp `ctx->…` 逐字段解引用：读
///   builtins/ice/constraint 为瞬态共享只读，`try_normalize`/`add_type_pack_t`
///   的可变借用半径止于所属语句；单线程独占驱动（lib.rs 不变量 1）下无并存别名。
pub unsafe fn unm_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // Safety: `ctx` 依函数级契约为整段归约的独占借用且会话期存活；取可变再借用，后续各字段句柄由此派生。
  let ctx_ref = &mut *ctx;
  if type_params.len() != 1 || !pack_params.is_empty() {
    ctx_ref.ice().ice_string("unm type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let mut operand_ty = follow_type::follow(type_params[0]);

  if operand_ty == instance {
    return TypeFunctionReductionResult::reduction(ctx_ref.builtins().never_type);
  }

  // check to see if the operand type is resolved enough, and wait to reduce if not
  // Safety: 调 unsafe fn `is_pending`；`ctx_ref.solver` 是 `*mut ConstraintSolver`，可空，
  // 其内部已判空，非 null 时指向会话存活的求解器并只读查询。
  if unsafe { is_pending(operand_ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![operand_ty]);
  }

  operand_ty = follow_type::follow(operand_ty);

  // C++ 中归一化失败返回空指针：无法归约，但对驻留性一无所知
  let Some(norm_ty) = ctx_ref.normalizer_mut().try_normalize(operand_ty) else {
    return TypeFunctionReductionResult::no_reduction(vec![]);
  };

  // if the operand is error suppressing, we can just go ahead and reduce.
  if (*norm_ty).should_suppress_errors() {
    return TypeFunctionReductionResult::reduction(operand_ty);
  }

  // if we have a `never`, we can never observe that the operation didn't work.
  if get_type::get::<NeverType>(operand_ty).is_some() {
    return TypeFunctionReductionResult::reduction(ctx_ref.builtins().never_type);
  }

  // If the type is exactly `number`, we can reduce now.
  if (*norm_ty).is_exactly_number() {
    return TypeFunctionReductionResult::reduction(ctx_ref.builtins().number_type);
  }

  // Safety: 调 unsafe fn `try_distribute_type_function_app`，其 ctx 为本借用的独占再借用；
  // 闭包以传入的 `c`（同一独占借用）递归回本 `unm_type_function`（unsafe fn），
  // 实参形状约束随每次递归由同一函数级契约覆盖。
  if let Some(result) = unsafe {
    try_distribute_type_function_app(
      |i, tp, pp, c| unm_type_function(i, tp, pp, c),
      instance,
      type_params,
      pack_params,
      &mut *ctx_ref,
    )
  } {
    return result;
  }

  // findMetatableEntry demands the ability to emit errors, so we must give it
  // the necessary state to do that, even if we intend to just eat the errors.
  let mut dummy: ErrorVec = vec![];

  let mm_type = find_metatable_entry(
    Handle::from_ref(ctx_ref.builtins()),
    &mut dummy,
    operand_ty,
    "__unm",
    Location::default(),
  );
  // 双写合一：`is_none()` 早退与块后 `unwrap()` 收为 let-else，Some 直接绑定。
  let Some(mm_type) = mm_type else {
    return TypeFunctionReductionResult::erroneous();
  };

  let mm_type_followed = follow_type::follow(mm_type);
  // Safety: 针对 `__unm` 元方法类型的第二个 `is_pending` 调用；solver 句柄的可空性与
  // 只读性同操作数处的第一次调用论证。
  if unsafe { is_pending(mm_type_followed, ctx_ref.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![mm_type_followed]);
  }

  let args_pack = ctx_ref
    .arena_mut()
    .add_type_pack_t(TypePack::single(operand_ty));

  let location = if !ctx_ref.constraint.is_null() {
    // Safety: `constraint` 是 `*const Constraint`，上一行 `!is_null()` 守卫通过才解引用
    // 读取 Copy 的 location 字段；其指向当前约束，随求解会话存活，只读访问。
    unsafe { (*ctx_ref.constraint).location }
  } else {
    Location::default()
  };

  // Safety: 调 unsafe fn `solve_function_call`，其 ctx 为指向会话有效上下文的只读再借用；
  // `location` 为上一步守卫分支的 Copy 值，`args_pack` 是已写入
  // arena 的 TypePackId（arena 节点随会话存活，lib.rs 不变量 2）。
  let result = unsafe { solve_function_call(&*ctx_ref, location, mm_type_followed, args_pack) };
  let result = match result {
    Some(r) => r,
    None => {
      return TypeFunctionReductionResult::erroneous();
    }
  };

  if let Some(ret) = first(result, true) {
    TypeFunctionReductionResult::reduction(ret)
  } else {
    TypeFunctionReductionResult::erroneous()
  }
}
