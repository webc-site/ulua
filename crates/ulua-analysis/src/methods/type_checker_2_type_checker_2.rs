use alloc::{boxed::Box, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{solver_mode::SolverMode, type_context::TypeContext},
  records::{
    arena_handle::{Handle, alias, alias_ref},
    builtin_types::BuiltinTypes,
    dcr_logger::DcrLogger,
    module::Module,
    normalizer::Normalizer,
    source_module::SourceModule,
    subtyping::Subtyping,
    type_check_limits::TypeCheckLimits,
    type_checker_2::TypeChecker2,
    type_function_runtime::TypeFunctionRuntime,
    unifier_shared_state::UnifierSharedState,
  },
};
impl TypeChecker2 {
  /// C++ `TypeChecker2::TypeChecker2(NotNull<BuiltinTypes>,
  /// NotNull<TypeFunctionRuntime>, NotNull<UnifierSharedState>,
  /// NotNull<TypeCheckLimits>, DcrLogger*, const SourceModule*, Module*)`
  /// (`Analysis/src/TypeChecker2.cpp:307`).
  ///
  /// Owned constructor. The C++ member-init list wires two self-referential
  /// pointers — `_subtyping`'s `NotNull<Normalizer>` points at the embedded
  /// `normalizer`, and `subtyping` points at the embedded `_subtyping`. Those
  /// cannot be set here because the returned value is moved into its final
  /// slot, so they are left null and wired by [`TypeChecker2::wire_self_pointers`]
  /// once the `TypeChecker2` lives at a stable address. 新调用点一律走
  /// [`TypeChecker2::new_boxed`]，本裸构造仅作为其内部步骤保留。
  ///
  /// 前置条件（调用方须满足，对应 cpp NotNull 语境）：
  /// - `unifier_state` 非空、对齐且比返回的检查器长寿；
  /// - `module` 非空且为 Arc 写穿句柄，取 `&mut (*module).internal_types` 时须无并存别名；
  /// - `type_function_runtime` 非空且比检查器长寿；`logger` 允许 null；
  /// - 返回值的两条自指针未布线，须立即 `wire_self_pointers` 或改走 `new_boxed`。
  pub(crate) fn new(
    builtin_types: Handle<BuiltinTypes>,
    type_function_runtime: *mut TypeFunctionRuntime,
    unifier_state: *mut UnifierSharedState,
    limits: &TypeCheckLimits,
    logger: *mut DcrLogger,
    source_module: &SourceModule,
    module: *mut Module,
  ) -> Self {
    // ice(unifierState->iceHandler)
    // `unifier_state` 由调用方（new_boxed）从 `&mut` 派生，非空由构造担保。
    let ice = alias_ref(unifier_state).ice_handler;

    // &module->internalTypes
    let arena = Handle::from_mut(&mut alias(module).internal_types);

    // normalizer{&module->internalTypes, builtinTypes, unifierState, SolverMode::New, /* cacheInhabitance */ true}
    let normalizer = Normalizer::new(
      Some(arena),
      builtin_types,
      Handle::from_opt_ptr(unifier_state),
      SolverMode::New,
      true,
    );

    // _subtyping{builtinTypes, NotNull{&module->internalTypes}, NotNull{&normalizer},
    //            typeFunctionRuntime, NotNull{unifierState->iceHandler}}
    // The NotNull<Normalizer> is wired in `wire_self_pointers` (it must point
    // at the moved-in `normalizer` field).
    let _subtyping = Subtyping::subtyping_owned(
      builtin_types,
      arena,
      None,
      alias_ref(type_function_runtime),
      ice.get(),
    );

    TypeChecker2 {
      builtin_types,
      type_function_runtime: Handle::from_ptr(type_function_runtime),
      logger: Handle::from_opt_ptr(logger),
      limits: Handle::from_ref(limits),
      ice,
      // `source_module` 为受检共享引用，`Handle::from_ref` 直存（非空与存活由类型承载）。
      source_module: Handle::from_ref(source_module),
      module,
      type_context: TypeContext::default(),
      stack: Vec::new(),
      function_decl_stack: Vec::new(),
      seen_type_function_instances: DenseHashSet::default(),
      normalizer,
      _subtyping,
      // subtyping(&_subtyping) — `None` 悬置窗口止于 `wire_self_pointers` 回填。
      subtyping: None,
      warned_globals: DenseHashSet::default(),
    }
  }

  /// 安全装箱构造器（手法对齐 [`crate::records::frontend::Frontend::new_boxed`]）：
  /// 把「裸构造 → `wire_self_pointers` 自指针回填 → 裸 `subtyping` 指针外露」
  /// 的悬置窗口整段收进本函数体，调用方拿到 `Box<TypeChecker2>` 即布线完成，
  /// 无中间态可误用；除 `module`（Arc 写穿句柄，登记为独立挂账）外入参全部
  /// 收窄为安全引用/Option 引用。
  ///
  /// 前置条件：`module` 须为非空、地址稳定的 Arc 写穿句柄，比返回的检查器长寿且
  /// 单线程独占；其余基础设施实参为受检引用，无额外契约。
  pub(crate) fn new_boxed(
    builtin_types: Handle<BuiltinTypes>,
    type_function_runtime: &mut TypeFunctionRuntime,
    unifier_state: &mut UnifierSharedState,
    limits: &mut TypeCheckLimits,
    logger: Option<&mut DcrLogger>,
    source_module: &SourceModule,
    module: *mut Module,
  ) -> Box<TypeChecker2> {
    // 受检引用按 C++ NotNull 形参语义还原为裸地址（目标均为调用方持有的
    // 存活独占局部，转铸不改变 provenance 纪律）。
    let type_function_runtime = NonNull::from(type_function_runtime).as_ptr();
    let unifier_state = NonNull::from(unifier_state).as_ptr();
    let logger = logger.map_or(null_mut(), |l| NonNull::from(l).as_ptr());
    let mut checker = Box::new(TypeChecker2::new(
      builtin_types,
      type_function_runtime,
      unifier_state,
      limits,
      logger,
      source_module,
      module,
    ));
    // checker 已由 `Box` 落在最终稳定地址且此后不再移动；布线产生的自指针
    // （`_subtyping.normalizer`、`subtyping`）恒指堆内地址。
    checker.wire_self_pointers();
    checker
  }

  /// Wires the two self-referential pointers the C++ member-init list sets:
  /// `_subtyping.normalizer = &normalizer` and `subtyping = &_subtyping`.
  /// Must be called after the `TypeChecker2` is at its final address (i.e.
  /// after the `new(..)` value has been moved into its storage slot) and
  /// before any use of `subtyping`.
  ///
  /// 新调用点一律走 [`TypeChecker2::new_boxed`]，免手写本调用与裸 `new` 对。
  ///
  /// 前置契约：The `TypeChecker2` must not be moved after this call, or the wired
  /// pointers dangle.（函数体只经 `Handle::from_mut` 由引用构造句柄，无 unsafe 操作，
  /// 故该契约是文档约定而非语言强制。）
  pub(crate) fn wire_self_pointers(&mut self) {
    self._subtyping.normalizer = Some(Handle::from_mut(&mut self.normalizer));
    // 孪生对句柄化回填：`Handle::from_mut(&mut self._subtyping)` 与原
    // `&mut self._subtyping as *mut Subtyping` 同一地址、同一自引用契约；
    // 此后 `subtyping` 字段恒 `Some`。
    self.subtyping = Some(Handle::from_mut(&mut self._subtyping));
  }
}
