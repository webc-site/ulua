use ulua_ast::records::{location::Location, position::Position};

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type, get_type, is_pending::is_pending,
    solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  macros::check_arity,
  records::{
    arena_handle::Handle, metatable_type::MetatableType, table_type::TableType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// 对应 C++ `lenTypeFunction`（`#` 取长）：实参形态（恰 1 个类型参、0 个类型包参）
/// 由 `check_arity!` 守卫，会话对象一律经 `TypeFunctionContext` 安全访问器取用，
/// 仅约束裸指针读取留在收窄的 `unsafe {}` 内。
pub fn len_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 1, "len");

  let operand_ty = follow_type::follow(type_params[0]);

  if operand_ty == instance {
    return TypeFunctionReductionResult::reduction(ctx.builtins().never_type);
  }

  // `is_pending` 为安全函数：solver 可空由内部判空折叠，非空时指向会话存活求解器。
  if is_pending(operand_ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![operand_ty]);
  }

  // C++ 中归一化失败返回空指针（isInhabited(nullptr) 亦为 HitLimits）：不归约
  let Some(norm_ty) = ctx.normalizer_mut().try_normalize(operand_ty) else {
    return TypeFunctionReductionResult::no_reduction(vec![]);
  };
  let inhabited = ctx
    .normalizer_mut()
    .is_inhabited_normalized_type(norm_ty.as_ref());

  if inhabited == NormalizationResult::HitLimits {
    return TypeFunctionReductionResult::no_reduction(vec![]);
  }

  if norm_ty.should_suppress_errors() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().number_type);
  }

  if inhabited == NormalizationResult::False || norm_ty.is_subtype_of_string() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().number_type);
  }

  let normalized_operand =
    follow_type::follow(ctx.normalizer_mut().type_from_normal(norm_ty.as_ref()));

  if norm_ty.has_top_table() || get_type::get::<TableType>(normalized_operand).is_some() {
    return TypeFunctionReductionResult::reduction(ctx.builtins().number_type);
  }

  // try_distribute_type_function_app 为安全函数：`instance`/切片均为 arena 存活句柄
  // 值原样转发，回调即本安全函数递归，实参形状由每次递归入口的 check_arity 守卫维持。
  if let Some(result) =
    try_distribute_type_function_app(len_type_function, instance, type_params, pack_params, ctx)
  {
    return result;
  }

  let mut dummy: ErrorVec = vec![];
  let mm_type = find_metatable_entry(
    Handle::from_ref(ctx.builtins()),
    &mut dummy,
    operand_ty,
    "__len",
    Location::new(
      Position { line: 0, column: 0 },
      Position { line: 0, column: 0 },
    ),
  );

  // 双写合一：`is_none()` 早退块两分支均 return，收为 let-else 后 Some
  // 直接绑定，与块后 `mm_type.unwrap()` 行为等价。
  let Some(mm_type) = mm_type else {
    if get_type::get::<MetatableType>(normalized_operand).is_some() {
      return TypeFunctionReductionResult::reduction(ctx.builtins().number_type);
    }

    return TypeFunctionReductionResult::erroneous();
  };

  let mm_type_followed = follow_type::follow(mm_type);
  // 针对元方法类型的第二个 `is_pending` 调用，solver 判空语义同前一处。
  if is_pending(mm_type_followed, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(vec![mm_type_followed]);
  }

  let args_pack = ctx
    .arena_mut()
    .add_type_pack_t(TypePack::single(operand_ty));

  let location = if !ctx.constraint.is_null() {
    // Safety: `constraint` 先经 `!is_null()` 守卫才解引用，读取 Copy 的 location
    // 字段；其指向当前约束，随求解会话存活，只读访问。
    unsafe { (*ctx.constraint).location }
  } else {
    Location::new(
      Position { line: 0, column: 0 },
      Position { line: 0, column: 0 },
    )
  };

  // solve_function_call 为安全函数：NonNull 字段重建收口其体内；`args_pack` 为
  // 已写入 arena 的存活句柄（lib.rs 不变量 2）。
  if solve_function_call(ctx, location, mm_type_followed, args_pack).is_none() {
    return TypeFunctionReductionResult::erroneous();
  }

  TypeFunctionReductionResult::reduction(ctx.builtins().number_type)
}
