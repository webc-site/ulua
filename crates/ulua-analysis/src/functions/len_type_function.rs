use ulua_ast::records::{location::Location, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type, get_type, is_pending::is_pending,
    solve_function_call::solve_function_call,
    try_distribute_type_function_app::try_distribute_type_function_app,
  },
  records::{
    arena_handle::Handle, metatable_type::MetatableType, table_type::TableType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, type_pack::TypePack,
  },
  type_aliases::{error_vec::ErrorVec, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 逐参数契约（对照 cpp `lenTypeFunction`，BuiltinTypeFunctions.cpp）：
/// - `instance`：本 type function 的实例类型 Id；函数以 `follow_type_id` 解析并做自指
///   比较，须指向驱动本次归约的类型 arena 中存活节点。
/// - `type_params`：实参类型切片。契约要求长度恰为 1（`#` 的一元操作数）。不等长时先
///   经 `ice`+`LUAU_ASSERT!(false)` 记录内部错误，随后仍按 `type_params[0]` 索引，故
///   调用方必须保证该形状，否则为 UB（与 cpp 对 `ConstTypeFunctionParameterList` 的
///   裸引用一致）。
/// - `pack_params`：打包实参切片，契约要求为空。
/// - `ctx`：本次调用的独占借用，指向本次归约期间存活的 `TypeFunctionContext`。其
///   `arena/builtins/normalizer/ice/solver/constraint` 是驱动会话持有的
///   NonNull/*mut 句柄，本函数按 cpp `ctx->…` 逐字段解引用。单线程独占驱动
///   （lib.rs 不变量 1/2）下，各 `&mut` 借用（try_normalize / is_inhabited_normalized_type
///   / type_from_normal / add_type_pack_t）半径止于所属语句，语句间不并存；对
///   builtins/ice/solver/constraint 的解引用为瞬态只读或按契约的可变访问。
pub unsafe fn len_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // Safety: `ctx` 依函数级契约为整段归约的独占借用且会话期存活；取可变再借用，后续各字段句柄据此派生。
  let ctx_ref = &mut *ctx;
  if type_params.len() != 1 || !pack_params.is_empty() {
    ctx_ref.ice().ice_string("len type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let operand_ty = follow_type::follow(type_params[0]);

  if operand_ty == instance {
    return TypeFunctionReductionResult::reduction(ctx_ref.builtins().never_type);
  }

  // Safety: 调 unsafe fn `is_pending`；`ctx_ref.solver` 是 `*mut ConstraintSolver`，可空，
  // 其内部已判空、非 null 时指向会话存活求解器，函数对其只读查询。
  if unsafe { is_pending(operand_ty, ctx_ref.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![operand_ty]);
  }

  // C++ 中归一化失败返回空指针（isInhabited(nullptr) 亦为 HitLimits）：不归约
  let Some(norm_ty) = ctx_ref.normalizer_mut().try_normalize(operand_ty) else {
    return TypeFunctionReductionResult::no_reduction(vec![]);
  };
  let inhabited = ctx_ref
    .normalizer_mut()
    .is_inhabited_normalized_type(norm_ty.as_ref());

  if inhabited == NormalizationResult::HitLimits {
    return TypeFunctionReductionResult::no_reduction(vec![]);
  }

  if (*norm_ty).should_suppress_errors() {
    return TypeFunctionReductionResult::reduction(ctx_ref.builtins().number_type);
  }

  if inhabited == NormalizationResult::False || { (*norm_ty).is_subtype_of_string() } {
    return TypeFunctionReductionResult::reduction(ctx_ref.builtins().number_type);
  }

  let normalized_operand =
    follow_type::follow(ctx_ref.normalizer_mut().type_from_normal(norm_ty.as_ref()));

  if { (*norm_ty).has_top_table() } || get_type::get::<TableType>(normalized_operand).is_some() {
    return TypeFunctionReductionResult::reduction(ctx_ref.builtins().number_type);
  }

  // Safety: 调 unsafe fn `try_distribute_type_function_app`，其对 ctx 的契约为本借用的
  // 瞬态独占再借用；闭包以同一再借用 `c` 递归回本 `len_type_function`（unsafe fn），
  // ctx 派生句柄的存活与借用边界由该工具函数按 cpp `ReducerFunction` 契约维持。
  if let Some(result) = unsafe {
    try_distribute_type_function_app(
      |i, tp, pp, c| len_type_function(i, tp, pp, c),
      instance,
      type_params,
      pack_params,
      &mut *ctx_ref,
    )
  } {
    return result;
  }

  let mut dummy: ErrorVec = vec![];
  let mm_type = find_metatable_entry(
    Handle::from_ref(ctx_ref.builtins()),
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
      return TypeFunctionReductionResult::reduction(ctx_ref.builtins().number_type);
    }

    return TypeFunctionReductionResult::erroneous();
  };

  let mm_type_followed = follow_type::follow(mm_type);
  // Safety: 针对元方法类型的第二个 `is_pending` 调用，solver 句柄判空与只读性同前一处论证。
  if unsafe { is_pending(mm_type_followed, ctx_ref.solver) } {
    return TypeFunctionReductionResult::no_reduction(vec![mm_type_followed]);
  }

  let args_pack = ctx_ref
    .arena_mut()
    .add_type_pack_t(TypePack::single(operand_ty));

  // Safety: 调 unsafe fn `solve_function_call`，其 ctx 契约为指向会话有效上下文的只读
  // 借用（resolver 内各句柄按 cpp `ctx->…` 解引用，不并存改写）；
  // `ctx_ref.constraint` 先经 `!is_null()` 守卫才解引用读取 location（当前约束随会话存活，只读）。
  if unsafe {
    solve_function_call(
      ctx_ref,
      if !ctx_ref.constraint.is_null() {
        (*ctx_ref.constraint).location
      } else {
        Location::new(
          Position { line: 0, column: 0 },
          Position { line: 0, column: 0 },
        )
      },
      mm_type_followed,
      args_pack,
    )
  }
  .is_none()
  {
    return TypeFunctionReductionResult::erroneous();
  }

  TypeFunctionReductionResult::reduction(ctx_ref.builtins().number_type)
}
