use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;
use ulua_common::{fint, records::dense_hash_table::DenseDefault};

use crate::{
  enums::{solver_mode::SolverMode, variance::Variance},
  functions::{arc_as_mut::arc_as_mut, follow_type, get_type},
  records::{
    any_type::AnyType, arena_handle::Handle, arena_id::ArenaId, builtin_types::BuiltinTypes,
    count_mismatch::CountMismatchContext, internal_error_reporter::InternalErrorReporter,
    module::Module, normalizer::Normalizer, pending_type::PendingType,
    pending_type_pack::PendingTypePack, subtyping::Subtyping, txn_log::TxnLog, r#type::Type,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, type_pack_var::TypePackVar, unifiable::Error,
    unifier::Unifier, unifier_shared_state::UnifierSharedState, union_type::UnionType,
  },
  type_aliases::{
    scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};
impl DenseDefault for Box<PendingType> {
  fn dense_default() -> Self {
    Box::new(PendingType {
      pending: Type::new(TypeVariant::Any(AnyType::default())),
      dead: true,
    })
  }
}

impl DenseDefault for Box<PendingTypePack> {
  fn dense_default() -> Self {
    Box::new(PendingTypePack {
      pending: TypePackVar {
        ty: TypePackVariant::Error(Error::<TypePackId>::new()),
        persistent: false,
        owning_arena: ArenaId::NONE,
      },
    })
  }
}

/// C++ `checkTypeMatch` (AutocompleteCore.cpp:186-205)：union 两侧分别按
/// any/all 递归下沉，其余走 Subtyping（新求解器）或 Unifier::canUnify（旧路径）
/// 判定 `sub_ty <: super_ty`。
///
/// `scope` 为 cpp `NotNull<Scope> moduleScope` 的引用化：调用点均持
/// `Arc<Scope>` 局部分身（模块根作用域），函数内仅按 crate 写穿纪律取
/// `arc_as_mut` 地址下传，并被返回前的局部消费用完，不留存借用。
pub fn check_type_match(
  module: &Module,
  sub_ty: TypeId,
  super_ty: TypeId,
  scope: &ScopePtr,
  type_arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
) -> bool {
  let sub_ty = follow_type::follow(sub_ty);
  let super_ty = follow_type::follow(super_ty);

  // 递归重入沿用入口前置：module/scope/type_arena/builtin_types 原样透传，
  // option 取自 union.options——与 super_ty/sub_ty 同 arena 的合法 TypeId。
  if let Some(super_union) = get_type::get::<UnionType>(super_ty) {
    return super_union
      .options
      .iter()
      .any(|&option| check_type_match(module, sub_ty, option, scope, type_arena, builtin_types));
  }

  if let Some(sub_union) = get_type::get::<UnionType>(sub_ty) {
    return sub_union
      .options
      .iter()
      .all(|&option| check_type_match(module, option, super_ty, scope, type_arena, builtin_types));
  }

  let mut ice_reporter = InternalErrorReporter {
    on_internal_error: None,
    module_name: String::new(),
  };
  let mut unifier_state = UnifierSharedState::new(&mut ice_reporter as *mut InternalErrorReporter);

  let solver_mode = if module.checked_in_new_solver {
    SolverMode::New
  } else {
    SolverMode::Old
  };

  // 原「空字面量 + setter 接线」等价改为直接 `Normalizer::new`：各缓存字段
  // 默认值与 setter 实参与原字面量逐一对应。
  let mut normalizer = Normalizer::new(
    Some(type_arena),
    builtin_types,
    Some(Handle::from_mut(&mut unifier_state)),
    solver_mode,
    false,
  );

  if module.checked_in_new_solver {
    let limits = TypeCheckLimits::default();
    // 原 increment_strong_count + from_raw 成对手续（等价一次 Arc clone）改由类型
    // 系统直接完成：scope 借用自调用方持有的 Arc<Scope> 分身，clone 即同计数一次
    // 正常强引用增量；root_scope 随本函数返回 drop 只减自己那份计数。
    let root_scope: ScopePtr = scope.clone();
    let scope_ptr = arc_as_mut(scope);
    // 空 `state`（未懒建的 lua_State）与未接线的 `runtime_builder` 收归构造器
    // `TypeFunctionRuntime::new` 单点——对应 cpp `TypeFunctionRuntime.cpp:58` ctor
    // 初始化 `state(nullptr, dummyStateClose)` 与头文件默认
    // `runtimeBuilder = nullptr`（TypeFunctionRuntime.h:327），本调用点不再手写
    // null 字面量。
    let mut type_function_runtime = TypeFunctionRuntime::new(&ice_reporter, &limits, root_scope);

    unifier_state.counters.recursion_limit = fint::LuauTypeInferRecursionLimit.get();
    unifier_state.counters.iteration_limit = fint::LuauTypeInferIterationLimit.get();

    // 原「null 字面量 + setter 接线」等价改为 `Subtyping::subtyping_owned`：
    // 其余字段默认值与原字面量一致（seen_packs 即空 SeenTypePackSet）。
    let mut subtyping = Subtyping::subtyping_owned(
      builtin_types,
      type_arena,
      &mut normalizer as *mut Normalizer,
      &mut type_function_runtime as *mut TypeFunctionRuntime,
      &mut ice_reporter as *mut InternalErrorReporter,
    );

    let result = subtyping.is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope_ptr);

    result.is_subtype
  } else {
    let scope_ptr = arc_as_mut(scope);
    let location = Location::default();
    let mut unifier = Unifier {
      types: type_arena,
      builtin_types,
      normalizer: Handle::from_mut(&mut normalizer),
      scope: Handle::from_ptr(scope_ptr),
      // 根日志的 seen 集接线收归 `TxnLog::new()` 单点：cpp 默认 ctor
      // `TxnLog() : sharedSeen(&ownedSeen)`（TxnLog.h:68-71）构造即把
      // `sharedSeen` 指向自有存储、`parent` 成员默认 `nullptr`
      // （TxnLog.h:271，链根无父），本调用点不再手写字面量。
      log: TxnLog::new(),
      failure: false,
      errors: Vec::new(),
      location,
      variance: Variance::Covariant,
      normalize: true,
      check_inhabited: true,
      ctx: CountMismatchContext::Arg,
      shared_state: Handle::from_mut(&mut unifier_state),
      blocked_types: Vec::new(),
      blocked_type_packs: Vec::new(),
      first_pack_error_pos: None,
    };

    // Cost of normalization can be too high for autocomplete response time requirements
    unifier.normalize = false;
    unifier.check_inhabited = false;

    unifier_state.counters.recursion_limit = fint::LuauTypeInferRecursionLimit.get();
    unifier_state.counters.iteration_limit = fint::LuauTypeInferIterationLimit.get();

    let errors = unifier.can_unify_type_id_type_id(sub_ty, super_ty);
    errors.is_empty()
  }
}
