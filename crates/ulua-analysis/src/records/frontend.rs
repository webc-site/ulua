//! Source: `Analysis/include/Luau/Frontend.h` (hand-ported; fields only)

use alloc::{string::String, sync::Arc, vec::Vec};
/// Frontend::Stats (nested struct)
use core::fmt::Debug;
use core::{
  fmt::{Formatter, Result},
  ptr::NonNull,
  sync::atomic::AtomicI32,
};

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, config_resolver::ConfigResolver,
    file_resolver::FileResolver, frontend_module_resolver::FrontendModuleResolver,
    frontend_options::FrontendOptions, global_types::GlobalTypes,
    internal_error_reporter::InternalErrorReporter, require_trace_result::RequireTraceResult,
    source_module::SourceModule, source_node::SourceNode,
  },
  type_aliases::{
    collections::HashMap,
    frontend_callbacks::{JsonLogCallback, ModuleScopeBoolCallback},
    module_name_type::ModuleName,
    scope_ptr_type::ScopePtr,
  },
};
#[derive(Debug, Clone, Copy, Default)]
pub struct FrontendStats {
  pub files: usize,
  pub lines: usize,
  pub files_strict: usize,
  pub files_nonstrict: usize,
  pub types_allocated: usize,
  pub type_packs_allocated: usize,
  pub bool_singletons_minted: usize,
  pub str_singletons_minted: usize,
  pub unique_str_singletons_minted: usize,
  pub time_read: f64,
  pub time_parse: f64,
  pub time_check: f64,
  pub time_lint: f64,
  pub dynamic_constraints_created: usize,
}

pub struct Frontend {
  pub use_new_luau_solver: AtomicI32,

  pub environments: HashMap<String, ScopePtr>,

  pub builtin_types_: BuiltinTypes,
  /// C++ `NotNull<BuiltinTypes>`，指向 `builtin_types_`。以 `Option<NonNull>`
  /// 编码自指针生命周期：构造落位前为 `None`（悬置窗口显式化，取代原
  /// `NonNull::dangling` 占位），由 `wire_self_pointers` 置为 `Some(自地址)`。
  /// 下游读取一律经 [`Frontend::builtin_types_ref`] / [`Frontend::builtin_types_handle`]
  /// chokepoint：`None` 表示「未布线即误用」的调用序契约违例，chokepoint 内
  /// `expect` 明确 panic（原 dangling 指针解引用为 UB，转 panic 是安全升级，
  /// 与「NotNull 恒非空」的 null 判分支一一对应），调用点免 unsafe。
  pub builtin_types: Option<NonNull<BuiltinTypes>>,

  /// C++ `FileResolver* fileResolver`。以 [`NonNull`] 建模（恒非空）：
  /// Frontend 本就是自引用指针结构（`builtin_types` / `config_resolver` 同款，
  /// 由 `wire_self_pointers` 布线），且该句柄被 RequireTracer 等按 C++ 语义以
  /// 裸别名共享（`&mut dyn` 双可变借用无法过借用检查），改 `Arc` 需横跨 5 个
  /// crate 的 30+ 处调用点重构。地址在构造时由调用方以 `&mut dyn FileResolver`
  /// 引用布线（构造函数内 `NonNull::from` 免 unsafe 记录），下游读取统一走
  /// [`Frontend::file_resolver_ref`] / [`Frontend::file_resolver_mut`]，
  /// ErrorConverter / autocomplete / TypeErrorToStringOptions 均已改为受命
  /// 周期引用的安全消费方。
  ///
  /// 此处 `dyn` 保留（运行期开放类型集）：`FileResolver` 由宿主注入实现，
  /// 跨 crate 有 NullFileResolver、unit-test、web、rt、cli 等多家实现方，
  /// 无法 enum_dispatch 穷举；Frontend 泛型化 `FR: FileResolver` 会波及
  /// 上述 30+ 调用点与自引用布线，编译期成本不合理。
  pub file_resolver: NonNull<dyn FileResolver>,
  pub module_resolver: FrontendModuleResolver,
  pub module_resolver_for_autocomplete: FrontendModuleResolver,
  pub globals: GlobalTypes,
  pub globals_for_autocomplete: GlobalTypes,
  /// C++ `ConfigResolver* configResolver`，同 `file_resolver`：恒非空建模的
  /// 外部对象句柄（null 入参以 dangling 占位、契约为从不查询 getConfig），
  /// 构造时布线；读取一律经 [`Frontend::config_resolver_ref`] chokepoint。
  pub config_resolver: NonNull<ConfigResolver>,
  pub options: FrontendOptions,
  pub ice_handler: InternalErrorReporter,
  pub prepare_module_scope: Option<ModuleScopeBoolCallback>,
  pub write_json_log: Option<JsonLogCallback>,

  pub source_nodes: HashMap<ModuleName, Arc<SourceNode>>,
  pub source_modules: HashMap<ModuleName, Arc<SourceModule>>,
  pub require_trace: HashMap<ModuleName, RequireTraceResult>,

  pub stats: FrontendStats,

  pub module_queue: Vec<ModuleName>,
}

impl Frontend {
  /// C++ `fileResolver` 成员的受控读取。构造方布线后指针恒非空且指向存活
  /// 对象（与 `Frontend` 同生命周期约定），此处集中解引用，调用点免 unsafe。
  pub fn file_resolver_ref(&self) -> &dyn FileResolver {
    // SAFETY: 见上；`file_resolver` 由构造方布线为有效对象，未被置空。
    unsafe { self.file_resolver.as_ref() }
  }

  /// 同 [`Frontend::file_resolver_ref`]，可变版（`readSource` / `resolveModule`
  /// 契约为 `&mut self`）。
  pub fn file_resolver_mut(&mut self) -> &mut dyn FileResolver {
    // SAFETY: 见上。
    unsafe { self.file_resolver.as_mut() }
  }

  /// C++ `configResolver` 成员的唯一解引用 chokepoint。入参为 null 时以
  /// dangling 占位布线（同 C++ nullptr 语义：仅当从不查询 `getConfig` 才
  /// 合法），故查询路径经本方法读取即隐含「构造方接了活对象」的调用序契约。
  pub fn config_resolver_ref<'a>(&self) -> &'a ConfigResolver {
    // Safety: `config_resolver` 由构造方（`frontend_*` ctor / `new_boxed`）以
    // 调用方提供的 `&mut` 引用布线为存活对象，比本 `Frontend` 长寿；读取
    // 借用期内单线程序列化驱动（lib.rs 不变量 1）、无并存可变别名，与原
    // 各调用点 `unsafe { self.config_resolver.as_ref() }` 语义逐项同构。
    unsafe { self.config_resolver.as_ref() }
  }

  /// 自指针 `builtin_types`（C++ `NotNull<BuiltinTypes>{&builtinTypes_}`）的
  /// **唯一解引用 chokepoint**（手法对齐 `BuiltinTypes::arena_handle` 与
  /// `Handle::get`：借用生命周期刻意不绑定 `&self`，与原
  /// `unsafe { self.builtin_types.as_ref() }` 的借用检查行为完全同构）。
  ///
  /// 字段已 `Option` 化：`None` 仅在 `wire_self_pointers` 之前的构造悬置窗口内
  /// 出现，此窗口内调用即上游构造序契约违例（原 dangling 指针解引用为 UB），
  /// 故以 `expect` 明确 panic，与「`NotNull` 恒非空」的判空语义一一对应。
  pub fn builtin_types_ref<'a>(&self) -> &'a BuiltinTypes {
    // Safety: `builtin_types` 由 `Frontend::wire_self_pointers` 布线为本结构
    // 自持字段 `builtin_types_` 的自指针，`Some` 时非空、地址与宿主同寿；单线程
    // 序列化驱动（lib.rs 不变量 1）下，读方物化的共享借用存续期内无并存可变别名
    //（BuiltinTypes 单例字段构造后只读，arena 改写一律经 `arena_handle`
    // 契约）。`None` 已在上方 `expect` 拦为契约违例 panic，不入 unsafe 解引用。
    unsafe { self.wired_builtin_types().as_ref() }
  }

  /// `builtin_types` 自指针的句柄形态 chokepoint：供需 `Handle<BuiltinTypes>`
  /// 或裸地址（`CloneState`、检查上下文、fixture 缓存 `*mut`）的调用点复用，
  /// 保留原 `Handle::from_nonnull(self.builtin_types)` 的可变 provenance 语义，
  /// 调用点免触碰 `Option<NonNull>` 裸字段。
  pub fn builtin_types_handle(&self) -> Handle<BuiltinTypes> {
    Handle::from_nonnull(self.wired_builtin_types())
  }

  /// 供上述两个 chokepoint 复用的取址helper：`None`（未布线窗口）即调用序
  /// 契约违例，明确 panic 而非静默解引用悬垂/dangling 指针。
  fn wired_builtin_types(&self) -> NonNull<BuiltinTypes> {
    self
      .builtin_types
      .expect("Frontend::builtin_types 尚未 wire_self_pointers 布线（构造序契约违例）")
  }
}

impl Debug for Frontend {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("Frontend")
      .field("use_new_luau_solver", &self.use_new_luau_solver)
      .field("environments", &self.environments)
      .field("builtin_types_", &self.builtin_types_)
      .field("builtin_types", &self.builtin_types)
      .field("file_resolver", &self.file_resolver)
      .field("module_resolver", &self.module_resolver)
      .field(
        "module_resolver_for_autocomplete",
        &self.module_resolver_for_autocomplete,
      )
      .field("globals", &self.globals)
      .field("globals_for_autocomplete", &self.globals_for_autocomplete)
      .field("config_resolver", &self.config_resolver)
      .field("options", &self.options)
      .field("ice_handler", &self.ice_handler)
      .field(
        "prepare_module_scope",
        &self.prepare_module_scope.as_ref().map(|_| "..."),
      )
      .field(
        "write_json_log",
        &self.write_json_log.as_ref().map(|_| "..."),
      )
      .field("source_nodes", &self.source_nodes)
      .field("source_modules", &self.source_modules)
      .field("require_trace", &self.require_trace)
      .field("stats", &self.stats)
      .field("module_queue", &self.module_queue)
      .finish()
  }
}
