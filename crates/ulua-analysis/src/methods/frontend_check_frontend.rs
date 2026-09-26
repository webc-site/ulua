use alloc::{rc::Rc, string::String, vec::Vec};

use ulua_ast::enums::mode::Mode;
use ulua_common::{
  fflag,
  macros::{
    luau_assert::LUAU_ASSERT, luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT,
    luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
  },
  records::dense_hash_set::DenseHashSet,
};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    check_frontend::{CheckArgs, check as check_new_solver},
    make_type_check_limits::make_type_check_limits,
  },
  records::{
    build_queue_item::BuildQueueItem,
    check_result::CheckResult,
    frontend::{Frontend, FrontendStats},
    frontend_options::FrontendOptions,
    module_resolver::ModuleResolverRef,
    require_cycle::RequireCycle,
    source_module::SourceModule,
    stats::Stats,
    type_check_limits::TypeCheckLimits,
    type_checker::TypeChecker,
  },
  type_aliases::{
    module_name_type::ModuleName, module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr,
  },
};

impl Frontend {
  pub fn check_module_name_optional_frontend_options(
    &mut self,
    name: &ModuleName,
    option_override: Option<FrontendOptions>,
  ) -> CheckResult {
    LUAU_TIMETRACE_SCOPE!("Frontend::check", "Frontend");
    LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

    let mut frontend_options = option_override.unwrap_or_else(|| self.options.clone());
    if self.get_luau_solver_mode() == SolverMode::New {
      frontend_options.for_autocomplete = false;
    }

    if let Some(result) = self.get_check_result(name, true, frontend_options.for_autocomplete) {
      return result;
    }

    let mut build_queue: Vec<ModuleName> = Vec::new();
    let cycle_detected = self.parse_graph(
      &mut build_queue,
      name,
      &make_type_check_limits(&frontend_options),
      frontend_options.for_autocomplete,
    );

    let mut seen: DenseHashSet<ModuleName> = DenseHashSet::new(ModuleName::default());
    let mut build_queue_items: Vec<BuildQueueItem> = Vec::new();
    self.add_build_queue_items(
      &mut build_queue_items,
      &build_queue,
      cycle_detected,
      &mut seen,
      &frontend_options,
    );
    LUAU_ASSERT!(!build_queue_items.is_empty());

    if fflag::DebugLuauLogSolverToJson.get() {
      LUAU_ASSERT!(
        build_queue_items
          .last()
          .expect("紧邻上方非空断言蕴含取尾必命中")
          .name
          == *name
      );
      build_queue_items
        .last_mut()
        .expect("紧邻上方非空断言蕴含取尾必命中")
        .record_json_log = true;
    }

    self.check_build_queue_items(&mut build_queue_items);

    let mut check_result = CheckResult::default();

    for item in &build_queue_items {
      if item.module.timeout {
        check_result.timeout_hits.push(item.name.clone());
      }

      if item.module.cancelled {
        return CheckResult::default();
      }

      check_result
        .errors
        .extend(item.module.errors.iter().cloned());

      if item.name == *name {
        check_result.lint_result = item.module.lint_result.clone();
      }
    }

    check_result
  }

  pub fn check_source_module_mode_vector_require_cycle_optional_scope_ptr_bool_bool_frontend_stats_type_check_limits(
    &mut self,
    source_module: &SourceModule,
    mode: Mode,
    require_cycles: Vec<RequireCycle>,
    environment_scope: Option<ScopePtr>,
    for_autocomplete: bool,
    record_json_log: bool,
    stats: &mut FrontendStats,
    type_check_limits: TypeCheckLimits,
  ) -> ModulePtr {
    if self.get_luau_solver_mode() == SolverMode::New {
      let prepare_module_scope_wrap = {
        let prepare_module_scope = self.prepare_module_scope.clone();
        Rc::new(move |name: &ModuleName, scope: &ScopePtr| {
          if let Some(prepare_module_scope) = &prepare_module_scope {
            prepare_module_scope(name, scope, for_autocomplete);
          }
        })
      };

      let mut function_stats = Stats {
        files: stats.files,
        lines: stats.lines,
        files_strict: stats.files_strict,
        files_nonstrict: stats.files_nonstrict,
        types_allocated: stats.types_allocated,
        type_packs_allocated: stats.type_packs_allocated,
        bool_singletons_minted: stats.bool_singletons_minted,
        str_singletons_minted: stats.str_singletons_minted,
        unique_str_singletons_minted: stats.unique_str_singletons_minted,
        time_read: stats.time_read,
        time_parse: stats.time_parse,
        time_check: stats.time_check,
        time_lint: stats.time_lint,
        dynamic_constraints_created: stats.dynamic_constraints_created,
      };

      let write_json_log = self
        .write_json_log
        .clone()
        .unwrap_or_else(|| Rc::new(|_: &ModuleName, _: String| {}));

      // 句柄为 Copy，先于任何 `&mut self` 字段借用取得，即结束对 `self` 的共享
      // 借用，避免与 resolver/ice_handler 的可变借用并存。
      let builtin_types = self.builtin_types_handle();

      // 按 cpp `ModuleResolver*` 借出 resolver：两分支同为
      // `FrontendModuleResolver`，直接由 `&mut` 构造 [`ModuleResolverRef`]
      // 具体分派句柄（`Copy`，按值流转，无 vtable/胖指针）。
      let module_resolver = ModuleResolverRef::from(if for_autocomplete {
        &mut self.module_resolver_for_autocomplete
      } else {
        &mut self.module_resolver
      });

      // 实参皆为有生命周期的安全引用（globals 树与调用入参）；
      // builtin_types/ice_handler/module_resolver 由本 Frontend 持有并比本次
      // 调用长寿，全程单线程驱动——满足 check_new_solver 文档所述不变量。
      let module = check_new_solver(CheckArgs {
        source_module,
        mode,
        require_cycles: &require_cycles,
        parent_scope: environment_scope
          .as_ref()
          .unwrap_or(&self.globals.global_scope),
        type_function_scope: &self.globals.global_type_function_scope,
        builtin_types,
        ice_handler: &mut self.ice_handler,
        module_resolver,
        prepare_module_scope: prepare_module_scope_wrap,
        options: self.options.clone(),
        limits: type_check_limits,
        record_json_log,
        stats: &mut function_stats,
        write_json_log,
      });

      stats.files = function_stats.files;
      stats.lines = function_stats.lines;
      stats.files_strict = function_stats.files_strict;
      stats.files_nonstrict = function_stats.files_nonstrict;
      stats.types_allocated = function_stats.types_allocated;
      stats.type_packs_allocated = function_stats.type_packs_allocated;
      stats.bool_singletons_minted = function_stats.bool_singletons_minted;
      stats.str_singletons_minted = function_stats.str_singletons_minted;
      stats.unique_str_singletons_minted = function_stats.unique_str_singletons_minted;
      stats.time_read = function_stats.time_read;
      stats.time_parse = function_stats.time_parse;
      stats.time_check = function_stats.time_check;
      stats.time_lint = function_stats.time_lint;
      stats.dynamic_constraints_created = function_stats.dynamic_constraints_created;

      module
    } else {
      // 句柄为 Copy，先于 `global_scope`/`resolver` 借用取得，结束对 `self` 的
      // 共享借用，避免与其后 `&mut self.module_resolver`/`&mut self.ice_handler` 并存。
      let builtin_types = self.builtin_types_handle();
      let global_scope = if for_autocomplete {
        &self.globals_for_autocomplete.global_scope
      } else {
        &self.globals.global_scope
      };
      let resolver = ModuleResolverRef::from(if for_autocomplete {
        &mut self.module_resolver_for_autocomplete
      } else {
        &mut self.module_resolver
      });
      // Safety: 满足 TypeChecker::new 的 # Safety 契约——global_scope 借用 self.globals(_for_
      // autocomplete).global_scope（Arc，比返回的 Box<TypeChecker> 长寿，checker 全程经它解引用）；
      // resolver 由 `&mut self.module_resolver…` 构造的 `ModuleResolverRef` 分派句柄，本 Frontend
      // 持有、检查期单线程独占；builtin_types 由 Frontend chokepoint 取句柄（恒非空、
      // 长寿），ice_handler 为 &mut self.ice_handler，二者均比 checker 及其
      // unifier_state 长寿。
      let mut type_checker =
        unsafe { TypeChecker::new(global_scope, resolver, builtin_types, &mut self.ice_handler) };
      if self.prepare_module_scope.is_some() {
        let prepare_module_scope = self.prepare_module_scope.clone();
        type_checker.prepare_module_scope = Some(Rc::new(move |name, scope| {
          if let Some(prepare_module_scope) = &prepare_module_scope {
            prepare_module_scope(name, scope, for_autocomplete);
          }
        }));
      }
      type_checker.require_cycles = require_cycles;
      type_checker.finish_time = type_check_limits.finish_time();
      type_checker.instantiation_child_limit = type_check_limits.instantiation_child_limit();
      type_checker.unifier_iteration_limit = type_check_limits.unifier_iteration_limit();
      type_checker.cancellation_token = type_check_limits.cancellation_token();

      type_checker.check_source_module(source_module, mode, environment_scope)
    }
  }
}
