use alloc::vec::Vec;

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::fflag;

use crate::{
  enums::reduction::Reduction,
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry, follow_type,
    get_type, is_pending::is_pending, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  macros::check_arity,
  records::{
    arena_handle::Handle, never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// 对照 cpp `concatTypeFunction`（`cpp/Analysis/src/BuiltinTypeFunctions.cpp:603`）。
/// 入参均为句柄值/切片/独占借用，无裸指针参数：`instance` 只做指针值自指比较，
/// `type_params`/`pack_params` 只 follow 不解引用，实参形状由 `check_arity!` 守卫；
/// 会话对象一律经 `TypeFunctionContext` 安全访问器取用，仅约束裸指针读取留在
/// 收窄的 `unsafe {}` 内，签名无需 `unsafe`。
pub fn concat_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, type_params, pack_params, 2, "concat");

  let lhs_ty = follow_type::follow(type_params[0]);
  let rhs_ty = follow_type::follow(type_params[1]);

  if lhs_ty == instance || rhs_ty == instance {
    return TypeFunctionReductionResult {
      result: Some(ctx.builtins().never_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  // is_pending 为安全函数：solver 可空由内部判空折叠，非空时指向会话存活求解器。
  if is_pending(lhs_ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(Vec::from([lhs_ty]));
  } else if is_pending(rhs_ty, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(Vec::from([rhs_ty]));
  }

  // C++ 中归一化失败返回空指针：不归约，对驻留性一无所知
  let norm_lhs = ctx.normalizer_mut().try_normalize(lhs_ty);
  let norm_rhs = ctx.normalizer_mut().try_normalize(rhs_ty);
  let (Some(norm_lhs_ty_ref), Some(norm_rhs_ty_ref)) = (norm_lhs, norm_rhs) else {
    return TypeFunctionReductionResult::no_reduction(Vec::new());
  };

  if norm_lhs_ty_ref.should_suppress_errors() || norm_rhs_ty_ref.should_suppress_errors() {
    return TypeFunctionReductionResult {
      result: Some(ctx.builtins().any_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  if get_type::get::<NeverType>(lhs_ty).is_some() || get_type::get::<NeverType>(rhs_ty).is_some() {
    return TypeFunctionReductionResult {
      result: Some(ctx.builtins().never_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  if (norm_lhs_ty_ref.is_subtype_of_string() || norm_lhs_ty_ref.is_exactly_number())
    && (norm_rhs_ty_ref.is_subtype_of_string() || norm_rhs_ty_ref.is_exactly_number())
  {
    return TypeFunctionReductionResult {
      result: Some(ctx.builtins().string_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  // try_distribute_type_function_app 为安全函数：`instance`/切片均为 arena 存活
  // 句柄值原样转发，回调即本安全函数递归，实参形状由每次递归入口的
  // check_arity 守卫维持。
  if let Some(result) = try_distribute_type_function_app(
    concat_type_function,
    instance,
    type_params,
    pack_params,
    ctx,
  ) {
    return result;
  }

  let mut dummy: ErrorVec = Vec::new();
  let mut mm_type = find_metatable_entry(
    Handle::from_ref(ctx.builtins()),
    &mut dummy,
    lhs_ty,
    "__concat",
    Location::new(Position::default(), Position::default()),
  );
  let mut reversed = false;
  if mm_type.is_none() {
    mm_type = find_metatable_entry(
      Handle::from_ref(ctx.builtins()),
      &mut dummy,
      rhs_ty,
      "__concat",
      Location::new(Position::default(), Position::default()),
    );
    reversed = true;
  }

  if mm_type.is_none() {
    return TypeFunctionReductionResult::erroneous();
  }

  // 上方 is_none 分支已早返，mm_type 至此必为 Some。
  let mm_type_ref = follow_type::follow(
    mm_type.expect("上方 is_none 分支已早返，至此必为 Some（cpp 同位判空后直 deref）"),
  );
  // 元方法类型的 `is_pending` 只读查询，复用同一判空的 solver 句柄。
  if is_pending(mm_type_ref, ctx.solver) {
    return TypeFunctionReductionResult::no_reduction(Vec::from([mm_type_ref]));
  }

  let mut inferred_args: Vec<TypeId> = Vec::new();
  if !reversed {
    inferred_args.push(lhs_ty);
    inferred_args.push(rhs_ty);
  } else {
    inferred_args.push(rhs_ty);
    inferred_args.push(lhs_ty);
  }

  let location = if ctx.constraint.is_null() {
    Location::new(Position::default(), Position::default())
  } else {
    // Safety: `constraint` 先经判空守卫才解引用，读取 Copy 的 location 字段；
    // 其指向当前约束，随求解会话存活，只读访问。
    unsafe { (*ctx.constraint).location }
  };

  if fflag::LuauConcatDoesntAlwaysReturnString.get() {
    let args_pack = ctx
      .arena_mut()
      .add_type_pack_vector_type_id_optional_type_pack_id(inferred_args, None);

    // solve_function_call 为安全函数：ctx 各 NonNull 字段的重建与论证收口其体内。
    let ret_pack = solve_function_call(ctx, location, mm_type_ref, args_pack);
    if ret_pack.is_none() {
      return TypeFunctionReductionResult::erroneous();
    }

    // builtins 句柄先物化（NonNull::as_ptr 按值取 self），再取 arena 可变借用，
    // 避免同一语句内并存借用冲突。
    let builtins = Handle::from_ref(ctx.builtins());
    let extracted = extend_type_pack(
      ctx.arena_mut(),
      builtins,
      ret_pack.expect("上方 is_none 分支已早返排除，ret_pack 至此必为 Some"),
      1,
      Vec::new(),
    );
    if extracted.head.is_empty() {
      return TypeFunctionReductionResult::erroneous();
    }

    TypeFunctionReductionResult::reduction(extracted.head[0])
  } else {
    let args_pack = ctx
      .arena_mut()
      .add_type_pack_vector_type_id_optional_type_pack_id(inferred_args, None);

    // solve_function_call 为安全函数，同上；本分支只检查是否求解成功。
    if solve_function_call(ctx, location, mm_type_ref, args_pack).is_none() {
      return TypeFunctionReductionResult::erroneous();
    }

    TypeFunctionReductionResult {
      result: Some(ctx.builtins().string_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
}
