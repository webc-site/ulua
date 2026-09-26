use alloc::{boxed::Box, vec::Vec};
use core::ptr::{NonNull, from_ref, null_mut};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::{solver_mode::SolverMode, type_context::TypeContext},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, dcr_logger::DcrLogger, module::Module,
    normalizer::Normalizer, source_module::SourceModule, subtyping::Subtyping,
    type_check_limits::TypeCheckLimits, type_checker_2::TypeChecker2,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
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
  /// # Safety
  /// 逐参数（对应 cpp 构造签名的 NotNull 语境，`Analysis/src/TypeChecker2.cpp:307`）：
  /// - `unifier_state` 非空、对齐且比返回的检查器长寿——本函数立即读其 `ice_handler`，
  ///   句柄还存入 `normalizer`/`_subtyping` 供整次 check 解引用；
  /// - `module` 非空且为 Arc 写穿句柄（C++ `NotNull<Module&>`），函数取
  ///   `&mut (*module).internal_types` 物化为 arena 句柄，要求此刻无其它借用别名；
  /// - `type_function_runtime`/`limits` 非空且比检查器长寿（仅存为句柄延后使用）；
  ///   `logger` 允许 null（C++ `DcrLogger*` 可空，以 `Option` 句柄承载）；
  /// - `source_module` 指向存活 `SourceModule`，只按 C++ `const SourceModule*` 只读使用。
  ///
  /// 返回值的两条自指针未布线，须立即 `wire_self_pointers` 或改走 `new_boxed`。
  pub unsafe fn new(
    builtin_types: Handle<BuiltinTypes>,
    type_function_runtime: *mut TypeFunctionRuntime,
    unifier_state: *mut UnifierSharedState,
    limits: *mut TypeCheckLimits,
    logger: *mut DcrLogger,
    source_module: *const SourceModule,
    module: *mut Module,
  ) -> Self {
    // ice(unifierState->iceHandler)
    // Safety: `unifier_state` 是入口按 C++ NotNull<UnifierSharedState> 契约传入的
    // 非空指针（TypeCheckSharedState 级持有，覆盖整次 check），此处只拷出其
    // `ice_handler` 裸句柄存字段，不解引用目标。
    let ice = unsafe { (*unifier_state).ice_handler };

    // &module->internalTypes
    // Safety: `module` 同为 NotNull 契约入参，指向本次 check 期间存活且独占的
    // Module；`&mut (*module).internal_types` 复刻 C++ `&module->internalTypes`——
    // 构造发生在该句柄被任何其它持有者使用之前，单线程此刻无其它借用与之别名，
    // arena 字段驻留在 Module 内、地址稳定，Handle::from_mut 物化后即固化为
    // 会话级句柄（与原裸指针持久化语义同构）。
    let arena = Handle::from_mut(unsafe { &mut (*module).internal_types });

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
    let _subtyping =
      Subtyping::subtyping_owned(builtin_types, arena, null_mut(), type_function_runtime, ice);

    TypeChecker2 {
      builtin_types,
      type_function_runtime: Handle::from_ptr(type_function_runtime),
      // 句柄化存字段：`logger` 原 C++ `DcrLogger*` 可空，null 哨兵以 `Option` 承载；
      // `limits` 为 NotNull 契约入参，`from_ptr` 判 null 即确定性 panic。
      logger: Handle::from_opt_ptr(logger),
      limits: Handle::from_ptr(limits),
      ice: Handle::from_ptr(ice),
      // Safety: `source_module` 由 `new_boxed` 门面经 `from_ref` 从受检
      // `&SourceModule` 派生，非空、对齐且在检查器存活期内有效；转 `*mut`
      // 仅为 `Handle` 存储形态，句柄消费面按 C++ `const SourceModule*` 只读使用。
      source_module: unsafe { Handle::from_raw(source_module as *mut SourceModule) },
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
  /// # Safety
  /// `module` 须为非空、地址稳定的 Arc 写穿句柄，比返回的检查器长寿且在本
  /// 检查器存活期内单线程独占（C++ `Module&` 引用形参契约，透传给 `new` 的
  /// 解引用）；其余基础设施实参为受检引用，无额外契约。
  pub unsafe fn new_boxed(
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
    let limits = NonNull::from(limits).as_ptr();
    let logger = logger.map_or(null_mut(), |l| NonNull::from(l).as_ptr());
    let source_module = from_ref(source_module);
    let mut checker = Box::new(
      // Safety: `unifier_state`/`module` 满足 `new` 的契约——前者由本函数
      // `&mut` 形参派生、调用期内独占存活；后者由调用方按 C++ NotNull<Module&>
      // 契约传入（Arc 写穿句柄，比本次检查长寿）；其余裸指针皆派生自活引用，
      // `Box::new` 把返回值落入堆槽即钉死地址，栈临时量携带的未布线态从未被
      // 解引用。
      unsafe {
        TypeChecker2::new(
          builtin_types,
          type_function_runtime,
          unifier_state,
          limits,
          logger,
          source_module,
          module,
        )
      },
    );
    // Safety: `Frontend`/`TypeChecker2` 同契约——checker 已由 `Box` 落在最终
    // 稳定地址且此后不再移动（Box 只移动句柄），布线产生的自指针
    // （`_subtyping.normalizer`、`subtyping`）恒指堆内地址。
    unsafe { checker.wire_self_pointers() };
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
  /// # Safety
  /// The `TypeChecker2` must not be moved after this call, or the wired
  /// pointers dangle.
  pub unsafe fn wire_self_pointers(&mut self) {
    self._subtyping.normalizer = Some(Handle::from_mut(&mut self.normalizer));
    // 孪生对句柄化回填：`Handle::from_mut(&mut self._subtyping)` 与原
    // `&mut self._subtyping as *mut Subtyping` 同一地址、同一自引用契约；
    // 此后 `subtyping` 字段恒 `Some`。
    self.subtyping = Some(Handle::from_mut(&mut self._subtyping));
  }
}
