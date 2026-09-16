//! Source: `Analysis/include/Luau/Frontend.h` (hand-ported; fields only)

use alloc::{string::String, sync::Arc, vec::Vec};
/// Frontend::Stats (nested struct)
use core::fmt::Debug;
use core::{
  fmt::{Formatter, Result},
  sync::atomic::AtomicI32,
};

use ulua_config::type_aliases::module_name::ModuleName;

use crate::{
  records::{
    builtin_types::BuiltinTypes, config_resolver::ConfigResolver, file_resolver::FileResolver,
    frontend_module_resolver::FrontendModuleResolver, frontend_options::FrontendOptions,
    global_types::GlobalTypes, internal_error_reporter::InternalErrorReporter,
    require_trace_result::RequireTraceResult, source_module::SourceModule, source_node::SourceNode,
  },
  type_aliases::{
    collections::HashMap,
    frontend_callbacks::{BuiltinDefinitionsMap, JsonLogCallback, ModuleScopeBoolCallback},
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
  pub builtin_definitions: BuiltinDefinitionsMap,

  pub builtin_types_: BuiltinTypes,
  pub builtin_types: *mut BuiltinTypes, // NotNull, points at builtin_types_

  /// C++ `FileResolver* fileResolver`。保留裸指针（`*mut dyn FileResolver`）：
  /// Frontend 本就是自引用指针结构（`builtin_types` / `config_resolver` 同款，
  /// 由 `wire_self_pointers` 布线），且该指针被 RequireTracer、ErrorConverter、
  /// autocomplete 参数等按 C++ 语义以裸句柄别名共享，改 `Arc` 需横跨 5 个
  /// crate 的 30+ 处调用点重构。指针在构造时由调用方以 `&mut 具体resolver`
  /// 布线，读取统一走 [`Frontend::file_resolver_ref`] / [`Frontend::file_resolver_mut`]。
  pub file_resolver: *mut dyn FileResolver,
  pub module_resolver: FrontendModuleResolver,
  pub module_resolver_for_autocomplete: FrontendModuleResolver,
  pub globals: GlobalTypes,
  pub globals_for_autocomplete: GlobalTypes,
  pub config_resolver: *mut ConfigResolver,
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
    unsafe { &*self.file_resolver }
  }

  /// 同 [`Frontend::file_resolver_ref`]，可变版（`readSource` / `resolveModule`
  /// 契约为 `&mut self`）。
  pub fn file_resolver_mut(&mut self) -> &mut dyn FileResolver {
    // SAFETY: 见上。
    unsafe { &mut *self.file_resolver }
  }
}

impl Debug for Frontend {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("Frontend")
      .field("use_new_luau_solver", &self.use_new_luau_solver)
      .field("environments", &self.environments)
      .field("builtin_definitions_len", &self.builtin_definitions.len())
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
