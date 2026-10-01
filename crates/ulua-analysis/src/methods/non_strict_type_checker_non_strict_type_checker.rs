use alloc::boxed::Box;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, data_flow_graph::DataFlowGraph,
    internal_error_reporter::InternalErrorReporter, module::Module,
    non_strict_type_checker::NonStrictTypeChecker, normalizer::Normalizer, subtyping::Subtyping,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
};
impl<'a> NonStrictTypeChecker<'a> {
  /// 形参全为受检句柄/引用（`module` 为 Arc 写穿裸句柄字段，仅存址不解引用），
  /// 契约由类型承载；新调用点一律走 [`Self::new_boxed`]。
  pub fn new(
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    type_function_runtime: &TypeFunctionRuntime,
    ice: &InternalErrorReporter,
    unifier_state: &mut UnifierSharedState,
    dfg: &DataFlowGraph,
    limits: &'a TypeCheckLimits,
    module: *mut Module,
  ) -> Self {
    let normalizer = Normalizer::new(
      Some(arena),
      builtin_types,
      Some(Handle::from_mut(unifier_state)),
      SolverMode::New,
      true,
    );
    let subtyping =
      Subtyping::subtyping_owned(builtin_types, arena, None, type_function_runtime, ice);

    NonStrictTypeChecker {
      builtin_types,
      type_function_runtime: Handle::from_ref(type_function_runtime),
      ice: Handle::from_ref(ice),
      arena,
      module,
      normalizer,
      subtyping,
      // 引用即非空 + 存活证明（C++ `const DataFlowGraph&` 形参直译），safe 接线。
      dfg: Handle::from_ref(dfg),
      stack: Vec::new(),
      cached_negations: DenseHashMap::default(),
      // `limits` 全程只读，共享借用直存字段（对齐 `OverloadResolver::limits` 形状）。
      limits,
      non_strict_recursion_count: 0,
    }
  }

  /// 安全装箱构造器（手法对齐 [`crate::records::frontend::Frontend::new_boxed`]）：
  /// 把「裸构造 → `subtyping.normalizer` 自指针回填 → 未布线中间态外露」的
  /// 悬置窗口整段收进本函数体，调用方拿到 `Box<NonStrictTypeChecker>` 即布线
  /// 完成，无中间态可误用；除 `module`（Arc 写穿句柄，独立挂账）外入参全部
  /// 收窄为受检引用，原 `ice as *mut` 的共享转独占转铸也随 `NonNull::from`
  /// 一并收进门面。
  pub fn new_boxed(
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    type_function_runtime: &mut TypeFunctionRuntime,
    ice: &mut InternalErrorReporter,
    unifier_state: &mut UnifierSharedState,
    dfg: &DataFlowGraph,
    limits: &'a TypeCheckLimits,
    module: *mut Module,
  ) -> Box<NonStrictTypeChecker<'a>> {
    // 受检引用按 C++ NotNull/const T& 形参语义直传（`limits`/`dfg` 为只读共享
    // 借用，原样入字段/句柄）；`module` 为 Arc 写穿裸句柄字段（独立挂账），
    // 仅存址不解引用。
    let mut checker = Box::new(NonStrictTypeChecker::new(
      arena,
      builtin_types,
      type_function_runtime,
      ice,
      unifier_state,
      dfg,
      limits,
      module,
    ));
    // Safety: checker 已由 `Box` 落在最终稳定地址且此后不再移动（Box 只移动
    // 句柄），接线的 `subtyping.normalizer` 自指针恒指堆内地址。
    unsafe { checker.wire_self_pointers() };
    checker
  }

  /// Wires `subtyping.normalizer` to the embedded `normalizer` after this
  /// checker has moved into its final stack slot.
  ///
  /// 新调用点一律走 [`NonStrictTypeChecker::new_boxed`]，免手写本调用。
  ///
  /// # Safety
  /// The checker must not be moved after this call.
  pub(crate) unsafe fn wire_self_pointers(&mut self) {
    self.subtyping.normalizer = Some(Handle::from_mut(&mut self.normalizer));
  }
}
