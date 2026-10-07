use alloc::vec;

use ulua_ast::records::location::Location;

use crate::{
  functions::{
    find_metatable_entry::find_metatable_entry, first::first, follow_type, get_type,
    is_pending::is_pending, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  macros::check_arity,
  records::{
    arena_handle::{Handle, alias_ref},
    never_type::NeverType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
    type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// 对应 C++ `unmTypeFunction`（BuiltinTypeFunctions.cpp:297）：一元取负 `-x` 的
/// type function 归约。实参形态（恰 1 个类型参、0 个类型包参）由 `check_arity!`
/// 守卫，arena/builtins/normalizer 等会话对象一律经 `TypeFunctionContext` 的
/// 安全访问器取用，故签名无需 `unsafe`。
pub fn unm_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 1, "unm");

  let mut operand_ty = follow_type::follow(type_params[0]);

  if operand_ty == instance {
    return TypeFunctionReductionResult::reduction(ctx.builtins().never_type);
  }

  // check to see if the operand type is resolved enough, and wait to reduce if not
  // `is_pending` 为安全函数：solver 可空由内部判空折叠，非空时指向会话存活求解器。
  if is_pending(operand_ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![operand_ty]);
  }

  operand_ty = follow_type::follow(operand_ty);

  // C++ 中归一化失败返回空指针：无法归约，但对驻留性一无所知
  let Some(norm_ty) = ctx.normalizer_mut().try_normalize(operand_ty) else {
    return TypeFunctionReductionResult::no_reduction(vec![]);
  };

  // if the operand is error suppressing, we can just go ahead and reduce.
  if norm_ty.should_suppress_errors() {
    return TypeFunctionReductionResult::reduction(operand_ty);
  }

  // if we have a `never`, we can never observe that the operation didn't work.
  if get_type::get::<NeverType>(operand_ty).is_some() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().never_type);
  }

  // If the type is exactly `number`, we can reduce now.
  if norm_ty.is_exactly_number() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().number_type);
  }

  // try_distribute_type_function_app 为安全函数：`instance`/切片均为 arena 存活句柄
  // 值原样转发，回调递归回安全的本函数，实参形状由函数体内 check_arity 守卫覆盖。
  if let Some(result) =
    try_distribute_type_function_app(unm_type_function, instance, type_params, pack_params, ctx)
  {
    return result;
  }

  // findMetatableEntry demands the ability to emit errors, so we must give it
  // the necessary state to do that, even if we intend to just eat the errors.
  let mut dummy: ErrorVec = vec![];

  let mm_type = find_metatable_entry(
    Handle::from_ref(ctx.builtins()),
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
  // 针对 `__unm` 元方法类型的第二个 `is_pending` 调用，solver 判空语义同上。
  if is_pending(mm_type_followed, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![mm_type_followed]);
  }

  let args_pack = ctx
    .arena_mut()
    .add_type_pack_t(TypePack::single(operand_ty));

  let location = if !ctx.constraint.is_null() {
    alias_ref(ctx.constraint).location
  } else {
    Location::default()
  };

  // solve_function_call 为安全函数：NonNull 字段重建收口其体内；`location` 为守卫
  // 分支的 Copy 值，`args_pack` 是已写入 arena 的存活句柄（lib.rs 不变量 2）。
  let Some(result) = solve_function_call(ctx, location, mm_type_followed, args_pack) else {
    return TypeFunctionReductionResult::erroneous();
  };

  if let Some(ret) = first(result, true) {
    TypeFunctionReductionResult::reduction(ret)
  } else {
    TypeFunctionReductionResult::erroneous()
  }
}
