use alloc::vec::Vec;

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::reduction::Reduction,
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry, follow_type,
    get_type, is_pending::is_pending, solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  records::{
    arena_handle::Handle, never_type::NeverType, type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 逐参数契约（对照 cpp `concatTypeFunction`，`cpp/Analysis/src/BuiltinTypeFunctions.cpp:603`）：
/// - `instance`：本 type function 的实例类型 Id，`follow_type_id` 解析并与 lhs/rhs 做自指
///   比较，须指向驱动本次归约的类型 arena 中存活节点。
/// - `type_params`：实参类型切片，契约要求长度恰为 2（`..` 的两侧操作数）。不等长时先经
///   `ice`+`LUAU_ASSERT!(false)` 记录内部错误，随后仍按 `[0]`/`[1]` 索引，调用方必须保证
///   该形状，否则越界为 UB（与 cpp 裸参数列表一致）。
/// - `pack_params`：打包实参切片，契约要求为空。
/// - `ctx`：本次调用的独占借用，指向本次归约期间存活的 `TypeFunctionContext`。其
///   `arena/builtins/normalizer/ice/solver/constraint` 为驱动会话持有的 NonNull/*mut 句柄，
///   本函数按 cpp `ctx->…` 逐字段解引用；单线程独占驱动（lib.rs 不变量 1/2）下，各 `&mut`
///   借用（try_normalize / add_type_pack_* / extend_type_pack）半径止于所属语句，语句间
///   不并存；对 builtins/ice/solver/constraint 的解引用为瞬态只读或按契约的可变访问。
pub unsafe fn concat_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // Safety: `ctx` 依函数级契约为整段归约的独占借用且会话期存活；取可变再借用供后续句柄派生。
  let ctx_ref = &mut *ctx;
  if type_params.len() != 2 || !pack_params.is_empty() {
    ctx_ref.ice().ice_string("concat type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let lhs_ty = follow_type::follow(type_params[0]);
  let rhs_ty = follow_type::follow(type_params[1]);

  if lhs_ty == instance || rhs_ty == instance {
    return TypeFunctionReductionResult {
      result: Some(ctx_ref.builtins().never_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  // Safety: 调 unsafe fn `is_pending`（lhs），`ctx_ref.solver` 为 `*mut ConstraintSolver`，
  // 可空、其内部判空、非 null 时指向会话存活求解器，只读查询。
  if unsafe { is_pending(lhs_ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult::no_reduction(Vec::from([lhs_ty]));
  } else if unsafe {
    // Safety: 同上，对 rhs 的 `is_pending` 只读查询，复用同一判空的 solver 句柄。
    is_pending(rhs_ty, ctx_ref.solver)
  } {
    return TypeFunctionReductionResult::no_reduction(Vec::from([rhs_ty]));
  }

  // C++ 中归一化失败返回空指针：不归约，对驻留性一无所知
  let norm_lhs = ctx_ref.normalizer_mut().try_normalize(lhs_ty);
  let norm_rhs = ctx_ref.normalizer_mut().try_normalize(rhs_ty);
  let (Some(norm_lhs_ty_ref), Some(norm_rhs_ty_ref)) = (norm_lhs, norm_rhs) else {
    return TypeFunctionReductionResult::no_reduction(Vec::new());
  };

  if norm_lhs_ty_ref.should_suppress_errors() || norm_rhs_ty_ref.should_suppress_errors() {
    return TypeFunctionReductionResult {
      result: Some(ctx_ref.builtins().any_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  if get_type::get::<NeverType>(lhs_ty).as_ref().is_some()
    || get_type::get::<NeverType>(rhs_ty).as_ref().is_some()
  {
    return TypeFunctionReductionResult {
      result: Some(ctx_ref.builtins().never_type),
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
      result: Some(ctx_ref.builtins().string_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    };
  }

  // Safety: 调 unsafe fn `try_distribute_type_function_app`，其对 ctx 的契约为借用再派生；
  // 闭包以传入的 `&mut` 引用递归回本 `concat_type_function`（unsafe fn），句柄存活与借用边界由
  // 该工具函数按 cpp `ReducerFunction` 契约维持。
  if let Some(result) = unsafe {
    try_distribute_type_function_app(
      |instance, type_params, pack_params, ctx| {
        concat_type_function(instance, type_params, pack_params, ctx)
      },
      instance,
      type_params,
      pack_params,
      &mut *ctx_ref,
    )
  } {
    return result;
  }

  let mut dummy: ErrorVec = Vec::new();
  let mut mm_type = find_metatable_entry(
    Handle::from_ref(ctx_ref.builtins()),
    &mut dummy,
    lhs_ty,
    "__concat",
    Location::new(Position::default(), Position::default()),
  );
  let mut reversed = false;
  if mm_type.is_none() {
    mm_type = find_metatable_entry(
      Handle::from_ref(ctx_ref.builtins()),
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

  // Safety: 上方 is_none 分支已早返，mm_type 至此必为 Some。
  let mm_type_ref = follow_type::follow(
    mm_type.expect("上方 is_none 分支已早返，至此必为 Some（cpp 同位判空后直 deref）"),
  );
  // Safety: 元方法类型的 `is_pending` 只读查询，solver 句柄判空性同前面两处调用。
  if unsafe { is_pending(mm_type_ref, ctx_ref.solver) } {
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

  if fflag::LuauConcatDoesntAlwaysReturnString.get() {
    let args_pack = ctx_ref
      .arena_mut()
      .add_type_pack_vector_type_id_optional_type_pack_id(inferred_args, None);

    // Safety: 调 unsafe fn `solve_function_call`，其 ctx 契约为指向会话有效上下文的只读借用；
    // `constraint` 先经 `!is_null()` 守卫再解引用读 location。
    let ret_pack = unsafe {
      solve_function_call(
        &*ctx_ref,
        if !ctx_ref.constraint.is_null() {
          (*ctx_ref.constraint).location
        } else {
          Location::new(Position::default(), Position::default())
        },
        mm_type_ref,
        args_pack,
      )
    };
    if ret_pack.is_none() {
      return TypeFunctionReductionResult::erroneous();
    }

    let builtins = Handle::from_ref(ctx_ref.builtins());
    // Safety: 调 unsafe fn `extend_type_pack`；arena 为独占可变借用，builtins 以 Handle 传入只读。
    let extracted = unsafe {
      extend_type_pack(
        ctx_ref.arena_mut(),
        builtins,
        ret_pack.expect("上方 is_none 分支已早返排除，ret_pack 至此必为 Some"),
        1,
        Vec::new(),
      )
    };
    if extracted.head.is_empty() {
      return TypeFunctionReductionResult::erroneous();
    }

    TypeFunctionReductionResult::reduction(extracted.head[0])
  } else {
    let args_pack = ctx_ref
      .arena_mut()
      .add_type_pack_vector_type_id_optional_type_pack_id(inferred_args, None);

    // Safety: 调 unsafe fn `solve_function_call`，其 ctx 契约为指向会话有效上下文的只读借用；
    // `constraint` 先经 `!is_null()` 守卫再解引用读 location。
    if unsafe {
      solve_function_call(
        &*ctx_ref,
        if !ctx_ref.constraint.is_null() {
          (*ctx_ref.constraint).location
        } else {
          Location::new(Position::default(), Position::default())
        },
        mm_type_ref,
        args_pack,
      )
    }
    .is_none()
    {
      return TypeFunctionReductionResult::erroneous();
    }

    TypeFunctionReductionResult {
      result: Some(ctx_ref.builtins().string_type),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
}
