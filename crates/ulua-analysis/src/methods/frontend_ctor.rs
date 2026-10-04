use alloc::{boxed::Box, vec::Vec};
use core::{ptr::NonNull, sync::atomic::AtomicI32};

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
  /// 前置契约：与 [`Frontend::wire_self_pointers`] 同契约：`Frontend` 落位后
  /// 不得移动，直至 `wire_self_pointers` 完成自引用布线。函数体只由引用经
  /// `NonNull::from` 记录裸地址，无 unsafe 操作，故上述契约是文档约定而非语言
  /// 强制。`file_resolver` / `config_resolver` 均为移交所有权的 `Box<dyn …>`
  /// （cpp 的 `FileResolver*` / `ConfigResolver*` 借用形态在此收窄为独占所有权，
  /// 见 [`Frontend::file_resolver`] / [`Frontend::config_resolver`] 字段注），
  /// 构造侧仅 move、无地址布线；cpp nullptr 缺位由宿主传入
  /// [`crate::records::null_config_resolver::NullConfigResolver`] 活实例表达。
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
  pub(crate) fn frontend_solver_mode_file_resolver_config_resolver_frontend_options(
    mode: SolverMode,
    file_resolver: Box<dyn FileResolver>,
    config_resolver: Box<dyn ConfigResolver>,
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

    // file_resolver / config_resolver 均为移交所有权的 `Box<dyn …>`：直接 move 进
    // 字段，无地址布线、无 Drop 守卫与之对偶（Frontend 析构即释放解析器）。

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
  pub fn new_boxed(
    mode: SolverMode,
    file_resolver: Box<dyn FileResolver>,
    config_resolver: Box<dyn ConfigResolver>,
    options: FrontendOptions,
  ) -> Box<Frontend> {
    // 入参全部为移交所有权的 `Box`，本函数无裸指针形参；cpp nullptr 缺位场景
    // （从不查询 getConfig）由宿主传入 NullConfigResolver 活实例承载。
    let mut frontend = Box::new(
      // 说明：透传构造器契约——两解析器所有权随参数移交，由 `Box` 字段独占
      //（无长寿契约、无半截句柄布线路径）；`Box::new` 把返回值按 move 落入堆槽
      // 后即钉死地址，栈临时量携带的旧地址自指针从未被解引用。
      Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
        mode,
        file_resolver,
        config_resolver,
        options,
      ),
    );
    // 说明：`Frontend` 已经 `Box` 落在最终稳定地址且此后不再移动（Box 只
    // 移动句柄），满足 `wire_self_pointers` 的「落位后不得移动」契约。
    frontend.wire_self_pointers();
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
  /// 前置契约：The `Frontend` must not be moved after this call, or the wired
  /// pointers dangle.
  ///
  /// 函数体所有指针均由引用经 `NonNull::from` 安全构造（原 `new_unchecked`
  /// 转铸与整体 `unsafe` 块已移除），无 unsafe 操作，故上述契约是文档约定
  /// 而非语言强制。优先改用 [`Frontend::new_boxed`] 以免手写本调用。
  pub(crate) fn wire_self_pointers(&mut self) {
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
}
