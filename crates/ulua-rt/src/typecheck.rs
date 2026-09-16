//! Static type-checking of Luau source against the host surface (the
//! `typecheck` feature).
//!
//! This is the unique capability `mlua` cannot offer: because ulua ships
//! Luau's static type checker (`ulua-analysis`), a script you are about to run
//! can be type-checked against exactly the API the host exposes *before* it
//! runs. The Luau VM is dynamically typed, so the runtime does not need any of
//! this — but the *static* checker has no knowledge of the host surface unless
//! you tell it.
//!
//! Modelled exactly on the umbrella `ulua` crate's `check` helper (itself a
//! port of `ulua-web`'s `check_script`): build a [`Frontend`] over an in-memory
//! single-source file resolver, register the Luau builtins, optionally load host
//! type definitions into the same global scope, insert the source as the module
//! `"main"`, and type-check it on the validated **old** solver.
//!
//! The one difference from the umbrella's helper is the diagnostic shape: each
//! diagnostic is surfaced as a structured [`TypeDiagnostic`] carrying its source
//! location (line/column, 1-based) rather than a flat `"<line>: <message>"`
//! string.

/// One type-checker diagnostic with its source location (all 1-based).
///
/// Produced by [`check`] / [`check_with_definitions`] and carried inside
/// [`Error::TypeError`](crate::Error::TypeError). Unlike a flat error string,
/// the location fields let an editor / build tool point at the exact span.
use core::{ffi::c_char, fmt, result::Result};
use std::{
  collections::HashMap,
  ffi::CStr,
  panic::{AssertUnwindSafe, catch_unwind},
};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{
    freeze::freeze, register_builtin_globals::register_builtin_globals,
    to_string_error::to_string_type_error, unfreeze::unfreeze,
  },
  records::{
    config_resolver::ConfigResolver, file_resolver::FileResolver, frontend::Frontend,
    frontend_options::FrontendOptions, load_definition_file_result::LoadDefinitionFileResult,
    module_info::ModuleInfo, source_code::SourceCode, type_check_limits::TypeCheckLimits,
    type_error::TypeError,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode, position::Position,
  },
  rtti::ast_node_as,
};
use ulua_config::records::config::Config;

use crate::callback::panic_message;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDiagnostic {
  /// The module that produced this diagnostic, when the checker had one.
  ///
  /// The simple [`check`] / [`check_with_definitions`] helpers preserve their
  /// historical display shape and leave this empty for the synthetic `"main"`
  /// module. Module-aware checks fill it for imported modules.
  pub module: Option<String>,
  /// 1-based start line.
  pub line: u32,
  /// 1-based start column.
  pub column: u32,
  /// 1-based end line.
  pub end_line: u32,
  /// 1-based end column.
  pub end_column: u32,
  /// The human-readable diagnostic message.
  pub message: String,
  /// True when the diagnostic comes from the host `declare` definitions rather
  /// than the checked script.
  pub in_definitions: bool,
}

impl fmt::Display for TypeDiagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.in_definitions {
      write!(f, "(host definitions) ")?;
    }
    if let Some(module) = &self.module {
      write!(f, "{module}:")?;
    }
    write!(f, "{}:{}: {}", self.line, self.column, self.message)
  }
}

/// The fixed module name under which the checked source is registered, matching
/// the umbrella helper's `fileResolver.source["main"] = source`.
const MAIN_MODULE: &str = "main";

/// Minimal single-source in-memory [`FileResolver`] for a string check.
///
/// Holds exactly one module's source ("main").
struct CheckFileResolver {
  source: String,
}

impl CheckFileResolver {
  fn new(source: &str) -> Self {
    CheckFileResolver {
      source: source.to_string(),
    }
  }
}

impl FileResolver for CheckFileResolver {
  /// `readSource` — returns the single source for the `"main"` module.
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    if name != MAIN_MODULE {
      return None;
    }
    Some(SourceCode {
      source: self.source.clone(),
      r#type: SourceCode::MODULE,
    })
  }
}

/// In-memory [`FileResolver`] for module-aware checks.
///
/// Holds a complete module-name -> source map. The resolver supports the common
/// Luau require path shapes used by host-embedded scripts:
///
/// - `require("@alias")`
/// - `require("literal/module/name")`
/// - `require(game.Module)`
/// - `require(script.Parent.Module)`
/// - `require(game:GetService("Service").Module)`
struct CheckModuleFileResolver {
  sources: HashMap<ModuleName, String>,
}

impl FileResolver for CheckModuleFileResolver {
  /// `readSource` — map lookup, every module typed as `SourceCode::MODULE`.
  fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
    let source = self.sources.get(name)?.clone();
    Some(SourceCode {
      source,
      r#type: SourceCode::MODULE,
    })
  }

  /// `resolveModule` — delegates to the inherent require-path resolver.
  fn resolve_module(
    &mut self,
    context: *const ModuleInfo,
    expr: *mut AstExpr,
    _limits: &TypeCheckLimits,
  ) -> Option<ModuleInfo> {
    if expr.is_null() {
      return None;
    }

    // SAFETY: `expr` 指向解析中 AST 的存活节点（C++ resolveModule 契约）。
    let context = if context.is_null() {
      None
    } else {
      Some(unsafe { &*context })
    };

    Self::resolve_module(self, context, unsafe { &*expr })
  }
}

impl CheckModuleFileResolver {
  fn new(modules: &[(&str, &str)]) -> Self {
    let sources = modules
      .iter()
      .map(|(name, source)| ((*name).to_string(), (*source).to_string()))
      .collect();

    CheckModuleFileResolver { sources }
  }

  fn resolve_module(&self, context: Option<&ModuleInfo>, expr: &AstExpr) -> Option<ModuleInfo> {
    let node = expr as *const AstExpr as *mut AstNode;

    unsafe {
      if let Some(string) = ast_node_as::<AstExprConstantString>(node).as_ref() {
        return self.module_info_if_known(constant_string_to_string(string), false);
      }

      if let Some(global) = ast_node_as::<AstExprGlobal>(node).as_ref() {
        let name = ast_name_to_string(global.name.value);

        if name == "script" {
          return context.cloned();
        }

        return self.module_info_if_known(name, true);
      }

      if let Some(index) = ast_node_as::<AstExprIndexName>(node).as_ref() {
        let context = context?;
        let index_name = ast_name_to_string(index.index.value);

        if index_name == "Parent" {
          let last_separator = context.name.rfind('/')?;
          return Some(ModuleInfo {
            name: context.name[..last_separator].to_string(),
            optional: context.optional,
          });
        }

        return self.module_info_if_known(format!("{}/{}", context.name, index_name), true);
      }

      if let Some(index) = ast_node_as::<AstExprIndexExpr>(node).as_ref() {
        let context = context?;
        let index_expr = index.index as *mut AstNode;

        if let Some(index_string) = ast_node_as::<AstExprConstantString>(index_expr).as_ref() {
          return self.module_info_if_known(
            format!(
              "{}/{}",
              context.name,
              constant_string_to_string(index_string)
            ),
            true,
          );
        }
      }

      if let Some(call) = ast_node_as::<AstExprCall>(node).as_ref() {
        let context = context?;

        if call.self_ && call.args.size >= 1 && context.name == "game" {
          let arg = *call.args.data;
          let arg_node = arg as *mut AstNode;
          let func_node = call.func as *mut AstNode;

          if let (Some(index_string), Some(func)) = (
            ast_node_as::<AstExprConstantString>(arg_node).as_ref(),
            ast_node_as::<AstExprIndexName>(func_node).as_ref(),
          ) && ast_name_to_string(func.index.value) == "GetService"
          {
            return self.module_info_if_known(
              format!("game/{}", constant_string_to_string(index_string)),
              true,
            );
          }
        }
      }
    }

    None
  }

  /// `name` 命中源表（或按需命中其子模块前缀 `name/...`）时返回 `ModuleInfo`。
  /// `allow_prefix` 控制是否额外检查子模块前缀（`require(game.Module)` 类
  /// require 路径需要，以便父模块也解析成功）。
  fn module_info_if_known(&self, name: String, allow_prefix: bool) -> Option<ModuleInfo> {
    let child_prefix = format!("{name}/");
    let known = self.sources.contains_key(&name)
      || (allow_prefix && self.sources.keys().any(|m| m.starts_with(&child_prefix)));
    known.then_some(ModuleInfo {
      name,
      optional: false,
    })
  }
}

fn ast_name_to_string(name: *const c_char) -> String {
  if name.is_null() {
    String::new()
  } else {
    unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() }
  }
}

fn constant_string_to_string(expr: &AstExprConstantString) -> String {
  expr
    .value
    .as_slice()
    .iter()
    .map(|&c| c as u8 as char)
    .collect()
}

/// Minimal [`ConfigResolver`] returning a default [`Config`].
///
/// `#[repr(C)]` with `base` first so the `getConfig` thunk can cast the
/// `*const ConfigResolver` receiver back to `*const CheckConfigResolver`.
#[repr(C)]
struct CheckConfigResolver {
  base: ConfigResolver,
  default_config: Config,
}

/// `getConfig` thunk — returns the single default config.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CheckConfigResolver`.
unsafe fn check_config_resolver_get_config_thunk(
  this: *const ConfigResolver,
  _name: *const ModuleName,
  _limits: *const TypeCheckLimits,
) -> *const Config {
  let this = this as *const CheckConfigResolver;
  // SAFETY: per this fn's contract, `this` points at a live `CheckConfigResolver`.
  unsafe { &(*this).default_config as *const Config }
}

impl CheckConfigResolver {
  fn new() -> Self {
    CheckConfigResolver {
      base: ConfigResolver {
        get_config: Some(check_config_resolver_get_config_thunk),
      },
      default_config: Config::default(),
    }
  }
}

/// The fixed package name under which host definitions are registered. Mirrors
/// `Fixture::loadDefinition`'s `"@test"`; `@`-prefixed names are the convention
/// for synthetic (non-file) modules.
const HOST_DEFINITIONS_PACKAGE: &str = "@host";

/// 把 Luau 0 基 `Location` 提升为 1 基诊断的共用样板（check 结果、definitions
/// 解析/类型错误共用）。
fn location_diagnostic(
  module: Option<String>,
  begin: Position,
  end: Position,
  message: String,
  in_definitions: bool,
) -> TypeDiagnostic {
  TypeDiagnostic {
    module,
    line: begin.line + 1,
    column: begin.column + 1,
    end_line: end.line + 1,
    end_column: end.column + 1,
    message,
    in_definitions,
  }
}

/// 注册 Luau builtins 与可选 host definitions（`run_check` / `run_check_modules`
/// 共用的 wire 后段）。
///
/// 返回 `false` = definitions 加载失败（诊断已写入 `diagnostics`，调用方应
/// 立即返回——在半建表面上继续检查没有意义）。
///
/// # Safety
/// `frontend` 必须已通过 [`Frontend::wire_self_pointers`] 布线，且整个调用
/// 期间不被移动（`register_builtin_globals` 需要同时可变借用 frontend 与其
/// globals 字段，经裸指针拆借）。
unsafe fn register_builtins_and_defs(
  frontend: &mut Frontend,
  definitions: Option<&str>,
  diagnostics: &mut Vec<TypeDiagnostic>,
) -> bool {
  let frontend_ptr = frontend as *mut Frontend;
  // Luau::unfreeze / registerBuiltinGlobals / freeze（与 C++ 注册顺序一致）。
  unsafe {
    unfreeze((*frontend_ptr).globals.global_types_mut());
    register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
    freeze((*frontend_ptr).globals.global_types_mut());
  }

  // Register host type definitions, if any, into the same global scope the
  // builtins live in. A script then type-checks against the host-provided
  // surface (the Rust functions / userdata exposed to the runtime). Pattern
  // mirrors `Fixture::loadDefinition`: unfreeze -> loadDefinitionFile(globals,
  // globalScope, …) -> freeze.
  if let Some(defs) = definitions.filter(|defs| !defs.is_empty()) {
    unsafe {
      unfreeze((*frontend_ptr).globals.global_types_mut());
      let target_scope = (*frontend_ptr).globals.global_scope();
      let result = (*frontend_ptr).load_definition_file(
        &mut (*frontend_ptr).globals,
        target_scope,
        defs,
        String::from(HOST_DEFINITIONS_PACKAGE),
        /* captureComments */ false,
        /* typeCheckForAutocomplete */ false,
      );
      freeze((*frontend_ptr).globals.global_types_mut());

      // Malformed host definitions are a usage error, surfaced with
      // `in_definitions: true` so they are distinguishable from script
      // diagnostics.
      if !result.success {
        push_definition_diagnostics(diagnostics, &result);
        return false;
      }
    }
  }
  true
}

/// 构造 frontend（`run_check` / `run_check_modules` / [`Checker::new`] 共用）。
/// 裸指针入参必须在调用点从**地址稳定**的对象取得（frontend 内部会保存它们）；
/// 自指针布线由调用方在 frontend 自身地址稳定（入 Box / 不再移动）后执行。
fn make_frontend(file_resolver: *mut dyn FileResolver, config: *mut ConfigResolver) -> Frontend {
  let options = FrontendOptions::default();
  Frontend::frontend_file_resolver_config_resolver_frontend_options(file_resolver, config, &options)
}

/// The fallible body, run under `catch_unwind` so a panic in the type checker
/// surfaces as a diagnostic rather than unwinding into the caller. Returns the
/// collected [`TypeDiagnostic`]s, or an empty `Vec` when the source type-checks
/// clean.
///
/// `definitions`, when present and non-empty, is Luau definition-file syntax
/// (`declare function …`, `declare class …`, `declare x: T`) describing the host
/// surface; it is registered into the global scope *after* the builtins (so it
/// may reference them) and *before* the script is checked.
fn run_check(source: &str, definitions: Option<&str>) -> Vec<TypeDiagnostic> {
  let mut diagnostics = Vec::new();

  let mut file_resolver = CheckFileResolver::new(source);
  let mut config_resolver = CheckConfigResolver::new();
  let mut frontend = make_frontend(
    &mut file_resolver as *mut dyn FileResolver,
    &mut config_resolver.base,
  );
  unsafe {
    frontend.wire_self_pointers();
  }

  // Use the validated OLD solver path.
  frontend.set_luau_solver_mode(SolverMode::Old);

  // Register builtins + host definitions, then type-check the script.
  if !unsafe { register_builtins_and_defs(&mut frontend, definitions, &mut diagnostics) } {
    return diagnostics;
  }

  // Luau::CheckResult checkResult = frontend.check("main");
  let check_result =
    frontend.check_module_name_optional_frontend_options(&MAIN_MODULE.to_string(), None);

  for err in &check_result.errors {
    diagnostics.push(location_diagnostic(
      None,
      err.location.begin,
      err.location.end,
      to_string_type_error(err),
      false,
    ));
  }

  diagnostics
}

fn push_definition_diagnostics(
  diagnostics: &mut Vec<TypeDiagnostic>,
  result: &LoadDefinitionFileResult,
) {
  for err in &result.parse_result.errors {
    diagnostics.push(location_diagnostic(
      None,
      err.get_location().begin,
      err.get_location().end,
      err.get_message().to_string(),
      true,
    ));
  }
  if let Some(module) = &result.module {
    for err in &module.errors {
      diagnostics.push(location_diagnostic(
        None,
        err.location.begin,
        err.location.end,
        to_string_type_error(err),
        true,
      ));
    }
  }
  if diagnostics.is_empty() {
    diagnostics.push(TypeDiagnostic {
      module: None,
      line: 1,
      column: 1,
      end_line: 1,
      end_column: 1,
      message: "failed to load".to_string(),
      in_definitions: true,
    });
  }
}

fn push_type_error_diagnostic(
  diagnostics: &mut Vec<TypeDiagnostic>,
  err: &TypeError,
  fallback_module: Option<&str>,
) {
  let module = if err.module_name.is_empty() {
    fallback_module.map(str::to_string)
  } else {
    Some(err.module_name.clone())
  };
  diagnostics.push(location_diagnostic(
    module,
    err.location.begin,
    err.location.end,
    to_string_type_error(err),
    false,
  ));
}

/// Type-check Luau `source`. Returns `Ok(())` if it type-checks clean, or `Err`
/// of the structured diagnostics on type errors.
///
/// ```
/// # #[cfg(feature = "typecheck")] {
/// ulua_rt::check("local x: number = 1").unwrap();
/// assert!(ulua_rt::check("local x: number = \"oops\"").is_err());
/// # }
/// ```
pub fn check(source: &str) -> Result<(), Vec<TypeDiagnostic>> {
  check_inner(source, None)
}

/// Type-check Luau `source` with extra host type `definitions` in scope.
///
/// `definitions` is Luau **definition-file syntax** describing the host-provided
/// globals — the Rust functions, values, and userdata you expose to the runtime
/// (e.g. via [`Lua::create_function`](crate::Lua::create_function) /
/// [`UserData`](crate::UserData)):
///
/// ```text
/// declare function add(a: number, b: number): number
/// declare config: { name: string, retries: number }
/// declare class Vec2
///     x: number
///     y: number
///     function magnitude(self): number
/// end
/// ```
///
/// Returns `Ok(())` when the source type-checks clean against the builtins plus
/// the host definitions, or `Err` of the structured diagnostics. Errors in the
/// definitions themselves are reported with `in_definitions == true`.
///
/// ```
/// # #[cfg(feature = "typecheck")] {
/// // The script references a host function the checker would otherwise reject:
/// ulua_rt::check("local n: number = add(1, 2)").unwrap_err();
/// ulua_rt::check_with_definitions(
///     "local n: number = add(1, 2)",
///     "declare function add(a: number, b: number): number",
/// )
/// .unwrap();
/// # }
/// ```
pub fn check_with_definitions(source: &str, definitions: &str) -> Result<(), Vec<TypeDiagnostic>> {
  check_inner(source, Some(definitions))
}

/// Type-check a graph of Luau modules.
///
/// `root_module` is the module name to check first. `modules` must contain a
/// `(module_name, source)` pair for `root_module`; any modules reachable through
/// supported `require(...)` paths are checked and their exported type surfaces
/// are used for the requiring script.
///
/// Supported require paths are the common embedded-script forms:
///
/// ```text
/// require("@alias")
/// require("literal/module/name")
/// require(game.Module)
/// require(script.Parent.Module)
/// require(game:GetService("Service").Module)
/// ```
pub fn check_modules(
  root_module: &str,
  modules: &[(&str, &str)],
) -> Result<(), Vec<TypeDiagnostic>> {
  check_modules_inner(root_module, modules, None)
}

/// Type-check a graph of Luau modules with extra host type `definitions` in
/// scope. See [`check_modules`] and [`check_with_definitions`].
pub fn check_modules_with_definitions(
  root_module: &str,
  modules: &[(&str, &str)],
  definitions: &str,
) -> Result<(), Vec<TypeDiagnostic>> {
  check_modules_inner(root_module, modules, Some(definitions))
}

/// Shared body of [`check`] / [`check_with_definitions`]: run the checker under
/// `catch_unwind` (so a panic in the type checker becomes a diagnostic rather
/// than unwinding into the caller) and fold the diagnostics into a `Result`.
fn check_inner(source: &str, definitions: Option<&str>) -> Result<(), Vec<TypeDiagnostic>> {
  // A panic inside the checker becomes a single diagnostic.
  let owned = source.to_string();
  let owned_defs = definitions.map(|d| d.to_string());
  checked(None, move || run_check(&owned, owned_defs.as_deref()))
}

/// 跑 `f` 并把 panic 折叠为单条诊断；空诊断集折叠为 `Ok(())`。
/// [`check_inner`] / [`check_modules_inner`] 共用。
fn checked(
  module: Option<String>,
  f: impl FnOnce() -> Vec<TypeDiagnostic>,
) -> Result<(), Vec<TypeDiagnostic>> {
  let diagnostics = match catch_unwind(AssertUnwindSafe(f)) {
    Ok(diagnostics) => diagnostics,
    Err(payload) => vec![TypeDiagnostic {
      module,
      line: 1,
      column: 1,
      end_line: 1,
      end_column: 1,
      message: panic_message(&payload),
      in_definitions: false,
    }],
  };
  if diagnostics.is_empty() {
    Ok(())
  } else {
    Err(diagnostics)
  }
}

fn check_modules_inner(
  root_module: &str,
  modules: &[(&str, &str)],
  definitions: Option<&str>,
) -> Result<(), Vec<TypeDiagnostic>> {
  let owned_root = root_module.to_string();
  let owned_modules: Vec<(String, String)> = modules
    .iter()
    .map(|(name, source)| ((*name).to_string(), (*source).to_string()))
    .collect();
  let owned_defs = definitions.map(|d| d.to_string());

  checked(Some(root_module.to_string()), move || {
    let borrowed: Vec<(&str, &str)> = owned_modules
      .iter()
      .map(|(name, source)| (name.as_str(), source.as_str()))
      .collect();
    run_check_modules(&owned_root, &borrowed, owned_defs.as_deref())
  })
}

fn run_check_modules(
  root_module: &str,
  modules: &[(&str, &str)],
  definitions: Option<&str>,
) -> Vec<TypeDiagnostic> {
  let mut diagnostics = Vec::new();

  if !modules.iter().any(|(name, _)| *name == root_module) {
    diagnostics.push(TypeDiagnostic {
      module: Some(root_module.to_string()),
      line: 1,
      column: 1,
      end_line: 1,
      end_column: 1,
      message: format!("root module '{root_module}' is not present in module sources"),
      in_definitions: false,
    });
    return diagnostics;
  }

  let mut file_resolver = CheckModuleFileResolver::new(modules);
  let mut config_resolver = CheckConfigResolver::new();
  let mut frontend = make_frontend(
    &mut file_resolver as *mut dyn FileResolver,
    &mut config_resolver.base,
  );
  unsafe {
    frontend.wire_self_pointers();
  }
  frontend.set_luau_solver_mode(SolverMode::Old);

  if !unsafe { register_builtins_and_defs(&mut frontend, definitions, &mut diagnostics) } {
    return diagnostics;
  }

  let check_result =
    frontend.check_module_name_optional_frontend_options(&root_module.to_string(), None);

  for err in &check_result.errors {
    push_type_error_diagnostic(&mut diagnostics, err, Some(root_module));
  }

  diagnostics
}

/// A reusable type checker: registers the Luau builtins **once** and checks many
/// sources against the shared global environment.
///
/// A one-shot [`check`] rebuilds the whole frontend and re-parses + re-checks the
/// entire `@luau` builtin definition file on every call — that setup dominates the
/// cost. `Checker` pays it once at construction, so each [`Checker::check`] only
/// parses and checks the input itself (orders of magnitude faster across many
/// snippets — fuzzers, language servers, batch linters).
///
/// Host definitions are not supported here: they mutate the global environment and
/// so can't be cached this way — use [`check_with_definitions`] for those.
pub struct Checker {
  // Boxed so their addresses are stable for the checker's whole lifetime:
  // `Frontend` stores a raw `*mut dyn FileResolver` into the resolver and
  // `wire_self_pointers` makes the frontend self-referential. Moving the
  // `Checker` moves only the `Box` pointers; the pointees stay put on the heap,
  // so every stored pointer stays valid. `_config_resolver` is kept alive
  // solely because the frontend points into it.
  file_resolver: Box<CheckFileResolver>,
  _config_resolver: Box<CheckConfigResolver>,
  frontend: Box<Frontend>,
}

impl Checker {
  /// Build a checker with the Luau builtins registered (the expensive,
  /// once-only step).
  pub fn new() -> Self {
    let mut file_resolver = Box::new(CheckFileResolver::new(""));
    let mut config_resolver = Box::new(CheckConfigResolver::new());
    let mut frontend = Box::new(make_frontend(
      &mut *file_resolver as *mut dyn FileResolver,
      &mut config_resolver.base,
    ));
    // Wire AFTER boxing so the self-pointers target the stable heap address.
    unsafe {
      frontend.wire_self_pointers();
    }
    frontend.set_luau_solver_mode(SolverMode::Old);

    // Register the Luau builtins into the global scope once (the cost we're
    // amortizing). `definitions == None` 时 [`register_builtins_and_defs`]
    // 恒成功，只做 builtin 注册。
    let ok = unsafe { register_builtins_and_defs(&mut frontend, None, &mut Vec::new()) };
    debug_assert!(ok);

    Checker {
      file_resolver,
      _config_resolver: config_resolver,
      frontend,
    }
  }

  /// Type-check `source` against the cached global environment. `Ok(())` when it
  /// checks clean, otherwise the diagnostics. Never panics on malformed input
  /// (the checker returns errors).
  pub fn check(&mut self, source: &str) -> Result<(), Vec<TypeDiagnostic>> {
    // Point the single "main" module at the new source and force a re-check.
    self.file_resolver.source = source.to_string();
    self.frontend.mark_dirty(&MAIN_MODULE.to_string(), None);

    let check_result = self
      .frontend
      .check_module_name_optional_frontend_options(&MAIN_MODULE.to_string(), None);

    let mut diagnostics = Vec::new();
    for err in &check_result.errors {
      diagnostics.push(location_diagnostic(
        None,
        err.location.begin,
        err.location.end,
        to_string_type_error(err),
        false,
      ));
    }
    if diagnostics.is_empty() {
      Ok(())
    } else {
      Err(diagnostics)
    }
  }
}

impl Default for Checker {
  fn default() -> Self {
    Self::new()
  }
}
