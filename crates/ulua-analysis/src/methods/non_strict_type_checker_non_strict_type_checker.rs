use alloc::boxed::Box;
use core::ptr::{NonNull, from_ref, null_mut};

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
impl NonStrictTypeChecker {
  pub fn new(
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    type_function_runtime: *mut TypeFunctionRuntime,
    ice: *const InternalErrorReporter,
    unifier_state: *mut UnifierSharedState,
    dfg: *const DataFlowGraph,
    limits: *const TypeCheckLimits,
    module: *mut Module,
  ) -> Self {
    let normalizer = Normalizer::new(
      Some(arena),
      builtin_types,
      Handle::from_opt_ptr(unifier_state),
      SolverMode::New,
      true,
    );
    let subtyping = Subtyping::subtyping_owned(
      builtin_types,
      arena,
      null_mut(),
      type_function_runtime,
      ice as *mut InternalErrorReporter,
    );

    NonStrictTypeChecker {
      builtin_types,
      type_function_runtime: Handle::from_ptr(type_function_runtime),
      ice: Handle::from_ptr(ice as *mut InternalErrorReporter),
      arena,
      module,
      normalizer,
      subtyping,
      dfg,
      stack: Vec::new(),
      cached_negations: DenseHashMap::default(),
      limits: limits as *mut TypeCheckLimits,
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
    limits: &TypeCheckLimits,
    module: *mut Module,
  ) -> Box<NonStrictTypeChecker> {
    // 受检引用按 C++ NotNull/const T* 形参语义还原为裸地址（目标均为调用方
    // 持有的存活对象，转铸不改变 provenance 纪律；`limits`/`dfg` 只读、
    // 原样以 const 指针下传）。
    let type_function_runtime = NonNull::from(type_function_runtime).as_ptr();
    let ice = NonNull::from(&mut *ice).as_ptr();
    let unifier_state = NonNull::from(unifier_state).as_ptr();
    let dfg = from_ref(dfg);
    let limits = from_ref(limits);
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
  pub unsafe fn wire_self_pointers(&mut self) {
    self.subtyping.normalizer = Some(Handle::from_mut(&mut self.normalizer));
  }
}
