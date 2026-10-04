use alloc::{string::String, vec::Vec};
use core::{mem::take, ptr::null_mut};

use ulua_ast::{enums::mode::Mode, records::ast_stat::AstStat};
use ulua_common::{fint, macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    apply_internal_limit_scaling::apply_internal_limit_scaling, copy_errors::copy_errors,
    filter_lint_options::filter_lint_options, freeze::freeze, get_timestamp::get_timestamp,
    lint::lint, make_type_check_limits::make_type_check_limits, shared_mut::shared_mut,
    unfreeze::unfreeze,
  },
  records::{
    arena_handle::alias, build_queue_item::BuildQueueItem, frontend::Frontend, module::Module,
    module_has_cyclic_dependency::ModuleHasCyclicDependency, syntax_error::SyntaxError,
    type_error::TypeError,
  },
  type_aliases::type_error_data::TypeErrorData,
};
impl Frontend {
  pub fn check_build_queue_item(&mut self, item: &mut BuildQueueItem) {
    // SAFETY: `item.source_node` / `item.source_module` are `Arc`s owned by the queue.
    // The C++ takes them by reference and mutates `sourceModule.mode` in place; we mirror
    // that with raw pointers through the `Arc`.
    let source_node_ptr = shared_mut(&item.source_node);

    // 对应 cpp `Frontend.cpp` `checkBuildQueueItem`（1776-1783 `Mode mode;` 决策链）；
    // 上游的两个调试旗标 `DebugLuauForceStrictMode` / `DebugLuauForceNonStrictMode` 已按
    // r7 deadcode 仲裁摘除（全仓无任何路径置 true，恒 false 分支即下面的 `unwrap_or` 兜底）。
    // 移植侧的 strict/nonstrict 由 Frontend 构造期写入的 `item.config.mode` 决定，行为等价。
    let mode: Mode = item.source_module.mode.unwrap_or(item.config.mode);

    let source_module_ptr = shared_mut(&item.source_module);
    // Safety: 对应 cpp `moduleInfo.sourceModule->mode = {mode;}`（Frontend.cpp
    // checkBuildQueueItem）的原地改写。Arc 不可变，故按 shared_mut 惯用法写穿：指针源自
    // `item.source_module`，`item` 经 `&mut` 独占访问且 Arc 在此刻无其它活跃借用，写入为
    // 单线程队列驱动（lib.rs 不变量 1）。
    {
      source_module_ptr.mode = Some(mode);
    }

    let environment_scope = item.environment_scope.clone();
    let timestamp = get_timestamp();
    let require_cycles = item.require_cycles.clone();

    let mut type_check_limits = make_type_check_limits(&item.options);

    // TODO (C++ comment retained): dirty ad hoc solution for autocomplete timeouts.
    if item.options.apply_internal_limit_scaling {
      // 经 Arc 共享引用读取即可（与 cpp `sourceNode.autocompleteLimitsMult` 同值），无需裸指针。
      let autocomplete_mult = item.source_node.autocomplete_limits_mult;

      if fint::LuauTarjanChildLimit.get() > 0 {
        type_check_limits.instantiation_child_limit =
          Some(1.max((fint::LuauTarjanChildLimit.get() as f64 * autocomplete_mult) as i32));
      } else {
        type_check_limits.instantiation_child_limit = None;
      }

      if fint::LuauTypeInferIterationLimit.get() > 0 {
        type_check_limits.unifier_iteration_limit =
          Some(1.max((fint::LuauTypeInferIterationLimit.get() as f64 * autocomplete_mult) as i32));
      } else {
        type_check_limits.unifier_iteration_limit = None;
      }
    }

    if item.options.for_autocomplete {
      // The autocomplete typecheck is always in strict mode with DM awareness.
      let module_for_autocomplete = self.check_source_module_mode_vector_require_cycle_optional_scope_ptr_bool_bool_frontend_stats_type_check_limits(
                &item.source_module,
                Mode::Strict,
                require_cycles,
                Some(environment_scope.clone()),
                /* for_autocomplete */ true,
                /* record_json_log */ false,
                &mut item.stats,
                type_check_limits,
            );

      let duration = get_timestamp() - timestamp;
      let module_ptr = shared_mut(&module_for_autocomplete);
      // Safety: 对应 cpp `moduleForAutocomplete->checkDurationSec = duration`。`module_ptr`
      // 源自本函数局部独占持有的 `Arc<Module>`（`check(...)` 刚返回的新建模块），直到末尾
      // `item.module = module_for_autocomplete` 移交前指针一直有效；此刻无任何对 Module 内容
      // 的并存借用。
      {
        module_ptr.check_duration_sec = duration;
      }

      // 调用序契约（`populate_expected_types` 为 safe fn，其体内借用经 safe 门面）：
      // 其契约要求模块裸指针非空且指向存活 Module（上段局部 Arc 界定）；调用期间其内部
      // 对 `(*module).ast_types` 等字段做可变改写，本函数此刻不再持有该 Module 的其它
      // 借用（`&item.source_module` 是另一 Arc）。
      self.populate_expected_types(&item.source_module, module_ptr, &environment_scope);

      if let Some(time_limit) = item.options.module_time_limit_sec
        && item.options.apply_internal_limit_scaling
      {
        apply_internal_limit_scaling(
          alias(source_node_ptr),
          module_for_autocomplete.clone(),
          time_limit,
        );
      }

      item.stats.time_check += duration;
      item.stats.files_strict += 1;

      if item.options.collect_type_allocation_stats {
        let m = &*module_for_autocomplete;
        item.stats.types_allocated += m.internal_types.types.size();
        item.stats.type_packs_allocated += m.internal_types.type_packs.size();
        item.stats.bool_singletons_minted += m.internal_types.bool_singletons_minted;
        item.stats.str_singletons_minted += m.internal_types.str_singletons_minted;
        item.stats.unique_str_singletons_minted +=
          m.internal_types.unique_str_singletons_minted.size();
      }

      if let Some(custom_module_check) = item.options.custom_module_check {
        custom_module_check(&item.source_module, &module_for_autocomplete);
      }

      item.module = module_for_autocomplete;
      return;
    }

    let module = self.check_source_module_mode_vector_require_cycle_optional_scope_ptr_bool_bool_frontend_stats_type_check_limits(
            &item.source_module,
            mode,
            require_cycles.clone(),
            Some(environment_scope.clone()),
            /* for_autocomplete */ false,
            item.record_json_log,
            &mut item.stats,
            type_check_limits,
        );

    let duration = get_timestamp() - timestamp;
    let module_ptr: *mut Module = shared_mut(&module);
    alias(module_ptr).check_duration_sec = duration;

    // 调用序契约（同上一段 `populate_expected_types` 的 safe fn 头文档）：指针存活
    // 与借用排它性由 `module_ptr` 的来源 Arc 与调用时序界定，`&item.source_module`
    // 借的是另一 Arc，Module 内容此刻仅由该指针访问。
    self.populate_expected_types(&item.source_module, module_ptr, &environment_scope);

    if let Some(time_limit) = item.options.module_time_limit_sec
      && item.options.apply_internal_limit_scaling
    {
      apply_internal_limit_scaling(alias(source_node_ptr), module.clone(), time_limit);
    }

    item.stats.time_check += duration;
    item.stats.files_strict += if mode == Mode::Strict { 1 } else { 0 };
    item.stats.files_nonstrict += if mode == Mode::Nonstrict { 1 } else { 0 };

    if item.options.collect_type_allocation_stats {
      let m = &*module;
      item.stats.types_allocated += m.internal_types.types.size();
      item.stats.type_packs_allocated += m.internal_types.type_packs.size();
      item.stats.bool_singletons_minted += m.internal_types.bool_singletons_minted;
      item.stats.str_singletons_minted += m.internal_types.str_singletons_minted;
      item.stats.unique_str_singletons_minted +=
        m.internal_types.unique_str_singletons_minted.size();
    }

    if let Some(custom_module_check) = item.options.custom_module_check {
      custom_module_check(&item.source_module, &module);
    }

    if self.get_luau_solver_mode() == SolverMode::New && mode == Mode::NoCheck {
      alias(module_ptr).errors.clear();
    }

    if item.options.run_lint_checks {
      LUAU_TIMETRACE_SCOPE!("lint", "Frontend");

      let mut lint_options = item
        .options
        .enabled_lint_warnings
        .unwrap_or(item.config.enabled_lint);
      filter_lint_options(&mut lint_options, &item.source_module.hotcomments, mode);

      let lint_timestamp = get_timestamp();

      let warnings = lint(
        // cpp `lint(sourceModule.root, ...)`：`lint` 形参仍持 `*mut AstStat`
        //（lint 子系统指针身份面，不在本波范围），`Option<Handle>` 于此折叠，
        // `None` ≡ nullptr 透传，与原桥逐位等价。
        item
          .source_module
          .root
          .map_or_else(null_mut, |h| h.cast::<AstStat>().as_ptr()),
        item.source_module.names.as_ref(),
        &environment_scope,
        module_ptr as *const Module,
        &item.source_module.hotcomments,
        &lint_options,
      );

      item.stats.time_lint += get_timestamp() - lint_timestamp;

      let lint_result = self.classify_lints(&warnings, &item.config);
      alias(module_ptr).lint_result = lint_result;
    }

    if !item.options.retain_full_type_graphs {
      // copyErrors needs to allocate into interfaceTypes as it copies types out of
      // internalTypes, so we unfreeze it here.
      let m = alias(module_ptr);
      unfreeze(&mut m.interface_types);
      // copyErrors(module->errors, module->interfaceTypes, builtinTypes);
      let mut errors = take(&mut m.errors);
      copy_errors(
        &mut errors,
        &mut m.interface_types,
        self.builtin_types_ref(),
      );
      m.errors = errors;
      freeze(&mut m.interface_types);

      m.internal_types.clear();
      m.def_arena.allocator.clear();
      m.key_arena.allocator.clear();

      m.ast_types.clear();
      m.ast_type_packs.clear();
      m.ast_expected_types.clear();
      m.ast_original_call_types.clear();
      m.ast_overload_resolved_types.clear();
      m.ast_for_in_next_types.clear();
      m.ast_resolved_types.clear();
      m.ast_resolved_type_packs.clear();
      m.ast_compound_assign_result_types.clear();
      m.ast_scopes.clear();
      m.upper_bound_contributors.clear();
      m.scopes.clear();
    }

    if mode != Mode::NoCheck {
      for cyc in &require_cycles {
        let te = TypeError {
          location: cyc.location,
          module_name: item.name.clone(),
          data: TypeErrorData::ModuleHasCyclicDependency(ModuleHasCyclicDependency::new(
            cyc.path.clone(),
          )),
        };
        alias(module_ptr).errors.push(te);
      }
    }

    let mut parse_errors: Vec<TypeError> = Vec::new();
    for pe in &item.source_module.parse_errors {
      parse_errors.push(TypeError {
        location: *pe.get_location(),
        module_name: item.name.clone(),
        data: TypeErrorData::SyntaxError(SyntaxError::new(String::from(pe.what()))),
      });
    }
    // module->errors.insert(module->errors.begin(), parseErrors.begin(), parseErrors.end());
    let mut combined = parse_errors;
    combined.append(&mut alias(module_ptr).errors);
    alias(module_ptr).errors = combined;

    item.module = module;
  }
}
