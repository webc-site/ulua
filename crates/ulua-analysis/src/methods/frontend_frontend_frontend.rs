use alloc::{boxed::Box, vec::Vec};
use core::{
  ptr::{NonNull, null_mut},
  sync::atomic::AtomicI32,
};

use ulua_common::fflag;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, config_resolver::ConfigResolver, file_resolver::FileResolver,
    frontend::Frontend, frontend_module_resolver::FrontendModuleResolver,
    frontend_options::FrontendOptions, global_types::GlobalTypes,
    internal_error_reporter::InternalErrorReporter,
  },
  type_aliases::collections::HashMap,
};

impl Frontend {
  /// Owned constructor for `Frontend::Frontend(SolverMode, FileResolver*,
  /// ConfigResolver*, FrontendOptions)`（`cpp/Analysis/src/Frontend.cpp:461`）。
  ///
  /// # Safety
  /// 与 [`Frontend::wire_self_pointers`] 同契约：`file_resolver` 须比返回的
  /// `Frontend` 长寿（字段按 C++ 语义以裸句柄别名共享），`config_resolver`
  /// 允许空（同 C++ nullptr 语义），且 `Frontend` 落位后不得移动，直至
  /// `wire_self_pointers` 完成自引用布线。
  ///
  /// The C++ member-init list wires several self-referential pointers:
  /// `builtinTypes(NotNull{&builtinTypes_})`, `moduleResolver(this)`,
  /// `moduleResolverForAutocomplete(this)`, and the two `GlobalTypes` members
  /// capture `builtinTypes` (i.e. `&builtinTypes_`). None of those can be set
  /// here because the returned value is moved into its final slot, so they are
  /// left null/dangling-free and wired by [`Frontend::wire_self_pointers`]
  /// once the `Frontend` lives at a stable address.
  ///
  /// `builtinTypes_`'s arena is heap-boxed, so moving the `BuiltinTypes` value
  /// itself is sound; `GlobalTypes::new` runs its arena mutations through the
  /// temporary `&builtinTypes_` pointer (valid for the duration of this call),
  /// and only the cached `builtin_types` back-pointer is re-pointed afterward.
  pub unsafe fn frontend_solver_mode_file_resolver_config_resolver_frontend_options<
    F: FileResolver + 'static,
  >(
    mode: SolverMode,
    file_resolver: &mut F,
    config_resolver: *mut ConfigResolver,
    options: FrontendOptions,
  ) -> Self {
    // useNewLuauSolver(mode)
    let use_new_luau_solver = AtomicI32::new(mode as i32);

    // builtinTypes_ is default-constructed; builtinTypes = NotNull{&builtinTypes_}.
    let mut builtin_types_ = BuiltinTypes::new();

    // getLuauSolverMode() == useNewLuauSolver.load() == mode.
    let solver_mode = mode;

    // globals(builtinTypes, getLuauSolverMode())
    // globalsForAutocomplete(builtinTypes, getLuauSolverMode())
    // `GlobalTypes::new` 现收 `&mut BuiltinTypes` 受检引用，两次调用的可变借用
    // 各自止于返回，`builtin_types_` 随后整体移入返回值（缓存别名由
    // `wire_self_pointers` 落位后重布线），调用点免构造 `NonNull`。
    let globals = GlobalTypes::new(&mut builtin_types_, solver_mode);
    let globals_for_autocomplete = GlobalTypes::new(&mut builtin_types_, solver_mode);

    // config_resolver 为 Sized，可安全构造：C++ 契约下允许 nullptr（仅在从不
    // 查询 getConfig 时安全），空入参以对齐 dangling 占位，占位值绝不可解引用。
    let config_resolver = NonNull::new(config_resolver).unwrap_or_else(NonNull::dangling);
    // file_resolver 以 `&mut F`（F: FileResolver + 'static）引用入参；字段
    // 指针默认携带 `'static` 对象界（自引用结构，见 `records/frontend.rs`
    // 说明），故先显式 unsize 为 `&mut (dyn FileResolver + 'static)` 再由
    // `NonNull::from` 免 unsafe 记录地址，布线语义与 C++ 逐字一致。
    let file_resolver: &mut (dyn FileResolver + 'static) = file_resolver;
    let file_resolver = NonNull::from(file_resolver);

    Frontend {
      use_new_luau_solver,
      environments: HashMap::new(),
      builtin_types_,
      // builtinTypes(NotNull{&builtinTypes_}) — Option::None 显式表示未布线的
      // 构造悬置窗口，由 wire_self_pointers 置 Some；取代原 NonNull::dangling。
      builtin_types: None,
      file_resolver,
      // moduleResolver(this) / moduleResolverForAutocomplete(this) — wired below.
      module_resolver: FrontendModuleResolver::new(None),
      module_resolver_for_autocomplete: FrontendModuleResolver::new(None),
      globals,
      globals_for_autocomplete,
      config_resolver,
      options,
      ice_handler: InternalErrorReporter::default(),
      prepare_module_scope: None,
      write_json_log: None,
      source_nodes: HashMap::new(),
      source_modules: HashMap::new(),
      require_trace: HashMap::new(),
      stats: Default::default(),
      module_queue: Vec::new(),
    }
  }

  /// 安全装箱构造器：把「构造 → 堆上落位 → `wire_self_pointers` 布线」整段
  /// 序列封装在函数内，自指针的悬置窗口不出函数体，调用方拿到即布线完成、
  /// 无中间态可误用，也免去除 `Box` 句柄（栈上可自由移动、堆内容地址恒定）
  /// 外的一切「落位后不得移动」心智负担。这是
  /// [`Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options`]
  /// 手写 unsafe 对的 chokepoint 替代品，新调用点一律走本函数。
  pub fn new_boxed<F: FileResolver + 'static>(
    mode: SolverMode,
    file_resolver: &mut F,
    config_resolver: Option<&mut ConfigResolver>,
    options: FrontendOptions,
  ) -> Box<Frontend> {
    // 入参全部为受检引用/Option 引用，本函数无裸指针形参；C++ nullptr 语义
    // （允许从不查询 getConfig）由 `None` 显式承载。
    let config_resolver = config_resolver.map_or(null_mut(), |c| NonNull::from(c).as_ptr());
    let mut frontend = Box::new(
      // Safety: 透传 unsafe 构造器契约——`file_resolver` 引用比返回的 Box
      // 长寿（由调用方借用期保证），`config_resolver` 为存活指针或 null
      //（C++ nullptr 语义，上方 `Option` 直译）；`Box::new` 把返回值按 move
      // 落入堆槽后即钉死地址，栈临时量携带的占位/旧地址自指针从未被解引用。
      unsafe {
        Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
          mode,
          file_resolver,
          config_resolver,
          options,
        )
      },
    );
    // Safety: `Frontend` 已经 `Box` 落在最终稳定地址且此后不再移动（Box 只
    // 移动句柄），满足 `wire_self_pointers` 的「落位后不得移动」契约。
    unsafe { frontend.wire_self_pointers() };
    frontend
  }

  /// Wires the self-referential pointers the C++ `Frontend` member-init list
  /// sets in place: `builtinTypes(&builtinTypes_)`, the two `GlobalTypes`'
  /// captured `builtinTypes`, and `moduleResolver(this)` /
  /// `moduleResolverForAutocomplete(this)`.
  ///
  /// Must be called once the `Frontend` is at its final address and before any
  /// use of `builtin_types`, `globals.builtin_types`, or the module resolvers.
  ///
  /// # Safety
  /// The `Frontend` must not be moved after this call, or the wired pointers
  /// dangle.
  ///
  /// 函数体本身免 unsafe：所有指针均由引用经 `NonNull::from` 安全构造
  /// （原 `new_unchecked` 转铸与整体 `unsafe` 块已移除），`unsafe fn` 仅
  /// 承载「落位后不得移动」的调用方契约。优先改用 [`Frontend::new_boxed`]
  /// 以免手写本调用。
  pub unsafe fn wire_self_pointers(&mut self) {
    let builtins = NonNull::from(&mut self.builtin_types_);
    self.builtin_types = Some(builtins);
    self.globals.builtin_types = Some(builtins);
    self.globals_for_autocomplete.builtin_types = Some(builtins);

    // `NonNull::from(&mut *self)` 借用止于本语句；句柄为 Copy 裸地址，随后
    // 对 `self.module_resolver*` 字段的写回不再与自借用冲突。
    let this = NonNull::from(&mut *self);
    self.module_resolver.frontend = Some(this);
    self.module_resolver_for_autocomplete.frontend = Some(this);
  }

  /// Owned constructor delegating to the `SolverMode`-taking ctor. The only
  /// difference from C++ ctor #1（`cpp/Analysis/src/Frontend.cpp:474`）is the solver
  /// mode is derived from the `LuauSolverV2` fast flag: `useNewLuauSolver(FFlag::LuauSolverV2 ?
  /// SolverMode::New : SolverMode::Old)`.
  ///
  /// As with the other ctor, the returned value's self-referential pointers
  /// are wired by [`Frontend::wire_self_pointers`] after placement.
  ///
  /// # Safety
  /// 透传 [`Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options`]
  /// 的安全契约：`file_resolver` 须比返回的 `Frontend` 长寿；`Frontend` 落位后不得移动。
  pub unsafe fn frontend_file_resolver_config_resolver_frontend_options<
    F: FileResolver + 'static,
  >(
    file_resolver: &mut F,
    config_resolver: *mut ConfigResolver,
    options: &FrontendOptions,
  ) -> Self {
    let mode = if fflag::LuauSolverV2.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    };

    // SAFETY: 契约由调用方保证，见本函数 # Safety。
    unsafe {
      Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
        mode,
        file_resolver,
        config_resolver,
        options.clone(),
      )
    }
  }
}
