use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;

use crate::{
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry, follow_type,
    get_mutable_type_pack, get_type, is_pending::is_pending,
    solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  macros::check_arity,
  records::{
    arena_handle::Handle, never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// 对应 C++ `numericBinopTypeFunction`（`BuiltinTypeFunctions.cpp`），由
/// `add/sub/mul/div/idiv/pow/mod` 包装入口与自身递归闭包转发。会话对象一律经
/// `TypeFunctionContext` 安全访问器取用；仅约束裸指针读取留在收窄的 `unsafe {}` 内。
pub fn numeric_binop_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
  metamethod: String,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 2);

  let lhs_ty = follow_type::follow(type_params[0]);
  let rhs_ty = follow_type::follow(type_params[1]);

  if lhs_ty == instance || rhs_ty == instance {
    return TypeFunctionReductionResult::reduction(ctx.builtins().never_type);
  }

  if get_type::get::<NeverType>(lhs_ty).is_some() || get_type::get::<NeverType>(rhs_ty).is_some() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().never_type);
  }

  let location = if !ctx.constraint.is_null() {
    // Safety: `constraint` 先经 `!is_null()` 守卫才解引用，读取 Copy 的 location
    // 字段；其指向当前约束，随求解会话存活，只读访问。
    unsafe { (*ctx.constraint).location }
  } else {
    Location::default()
  };

  // `is_pending` 为安全函数：solver 可空由内部判空折叠。
  if is_pending(lhs_ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![lhs_ty]);
  } else if is_pending(rhs_ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![rhs_ty]);
  }

  let norm_lhs_ty = ctx.normalizer_mut().try_normalize(lhs_ty);
  let norm_rhs_ty = ctx.normalizer_mut().try_normalize(rhs_ty);

  let (Some(norm_lhs_ty), Some(norm_rhs_ty)) = (norm_lhs_ty, norm_rhs_ty) else {
    return TypeFunctionReductionResult::no_reduction(Vec::new());
  };

  // if one of the types is error suppressing, we can reduce to `any` since we should
  // suppress errors in the result of the usage.
  if norm_lhs_ty.should_suppress_errors() || norm_rhs_ty.should_suppress_errors() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().any_type);
  }

  // if we're adding two `number` types, the result is `number`.
  if norm_lhs_ty.is_exactly_number() && norm_rhs_ty.is_exactly_number() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().number_type);
  }

  // try_distribute_type_function_app 为安全函数：递归闭包捕获 metamethod，透传
  // 同一独占借用递归回安全的本函数，实参形状由每次递归入口的 check_arity 守卫维持。
  if let Some(result) = try_distribute_type_function_app(
    |instance, type_params, pack_params, ctx| {
      numeric_binop_type_function(instance, type_params, pack_params, ctx, metamethod.clone())
    },
    instance,
    type_params,
    pack_params,
    ctx,
  ) {
    return result;
  }

  let mut dummy: ErrorVec = Vec::new();

  // lhs 未命中元表项才回退查 rhs 并翻转实参序（cpp 同位 `reversed` 跟踪）。
  let (mm_type_opt, reversed) = match find_metatable_entry(
    Handle::from_ptr(ctx.builtins.as_ptr()),
    &mut dummy,
    lhs_ty,
    &metamethod,
    location,
  ) {
    Some(mm_type) => (Some(mm_type), false),
    None => (
      find_metatable_entry(
        Handle::from_ptr(ctx.builtins.as_ptr()),
        &mut dummy,
        rhs_ty,
        &metamethod,
        location,
      ),
      true,
    ),
  };

  // 双写合一：`is_none()` 早退与随后 `expect()` 收为 let-else（与 comparison 同构）。
  let Some(mm_type) = mm_type_opt else {
    return TypeFunctionReductionResult::erroneous();
  };

  let mm_type = follow_type::follow(mm_type);

  if is_pending(mm_type, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![mm_type]);
  }

  let arg_pack = ctx
    .arena_mut()
    .add_type_pack_initializer_list_type_id(&[lhs_ty, rhs_ty]);

  if reversed
    && let Some(pack_ref) = get_mutable_type_pack::get_mutable::<TypePack>(arg_pack)
    && pack_ref.head.len() >= 2
  {
    pack_ref.head.swap(0, 1);
  }

  // 双写合一：同上，`is_none()` 早退与 `expect()` 收为 let-else。
  // solve_function_call 为安全函数：NonNull 字段重建收口其体内；`arg_pack` 为
  // 刚写入 arena 的存活句柄（lib.rs 不变量 2）。
  let Some(ret_pack_val) = solve_function_call(ctx, location, mm_type, arg_pack) else {
    return TypeFunctionReductionResult::erroneous();
  };
  // builtins 句柄先物化（NonNull::as_ptr 按值取 self），再取 arena 可变借用，
  // 避免同一语句内并存借用冲突。
  let builtins_handle = Handle::from_ptr(ctx.builtins.as_ptr());
  let extracted = extend_type_pack(
    ctx.arena_mut(),
    builtins_handle,
    ret_pack_val,
    1,
    Vec::new(),
  );

  if extracted.head.is_empty() {
    return TypeFunctionReductionResult::erroneous();
  }

  TypeFunctionReductionResult::reduction(extracted.head[0])
}
