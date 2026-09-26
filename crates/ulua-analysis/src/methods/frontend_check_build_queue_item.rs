use alloc::{string::String, vec::Vec};
use core::mem::take;

use ulua_ast::{enums::mode::Mode, records::ast_stat::AstStat};
use ulua_common::{fflag, fint, macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    apply_internal_limit_scaling::apply_internal_limit_scaling, arc_as_mut::arc_as_mut,
    copy_errors::copy_errors, filter_lint_options::filter_lint_options, freeze::freeze,
    get_timestamp::get_timestamp, lint::lint, make_type_check_limits::make_type_check_limits,
    unfreeze::unfreeze,
  },
  records::{
    build_queue_item::BuildQueueItem, frontend::Frontend, module::Module,
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
    let source_node_ptr = arc_as_mut(&item.source_node);

    let mode: Mode = if fflag::DebugLuauForceStrictMode.get() {
      Mode::Strict
    } else if fflag::DebugLuauForceNonStrictMode.get() {
      Mode::Nonstrict
    } else {
      item.source_module.mode.unwrap_or(item.config.mode)
    };

    let source_module_ptr = arc_as_mut(&item.source_module);
    // Safety: 对应 cpp `moduleInfo.sourceModule->mode = {mode;}`（Frontend.cpp
    // checkBuildQueueItem）的原地改写。Arc 不可变，故按 arc_as_mut 惯用法写穿：指针源自
    // `item.source_module`，`item` 经 `&mut` 独占访问且 Arc 在此刻无其它活跃借用，写入为
    // 单线程队列驱动（lib.rs 不变量 1）。
    unsafe {
      (*source_module_ptr).mode = Some(mode);
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
      let module_ptr = arc_as_mut(&module_for_autocomplete);
      // Safety: 对应 cpp `moduleForAutocomplete->checkDurationSec = duration`。`module_ptr`
      // 源自本函数局部独占持有的 `Arc<Module>`（`check(...)` 刚返回的新建模块），直到末尾
      // `item.module = module_for_autocomplete` 移交前指针一直有效；此刻无任何对 Module 内容
      // 的并存借用。
      unsafe {
        (*module_ptr).check_duration_sec = duration;
      }

      // Safety: 调 unsafe fn `populate_expected_types`，其契约要求模块裸指针非空且指向存活
      // Module（上段局部 Arc 界定）；调用期间其内部对 `(*module).ast_types` 等字段做可变
      // 改写，本函数此刻不再持有该 Module 的其它借用（`&item.source_module` 是另一 Arc）。
      unsafe { self.populate_expected_types(&item.source_module, module_ptr, &environment_scope) };

      if let Some(time_limit) = item.options.module_time_limit_sec
        && item.options.apply_internal_limit_scaling
      {
        // Safety: `source_node_ptr` 源自队列项 `item.source_node` 的 Arc（函数首部按
        // arc_as_mut 惯用法取得）；cpp `applyInternalLimitScaling(sourceNode, ...)` 原地写
        // `autocompleteLimitsMult`。此处临时创建仅限本调用语句的 `&mut`，其存活期内对
        // SourceNode 无其它读取或写穿（本处即唯一的可变访问点，单线程驱动）。
        apply_internal_limit_scaling(
          unsafe { &mut *source_node_ptr },
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
    let module_ptr: *mut Module = arc_as_mut(&module);
    // Safety: 非 autocomplete 主路径的 `module->checkDurationSec = duration`。`module` 是
    // `check(...)` 刚返回、由本函数局部独占的 `Arc<Module>`，直到函数末尾移交 `item.module`
    // 前指针一直有效；写入时刻无并存借用（上方对 Module 的最后共享读取已随语句结束）。
    unsafe {
      (*module_ptr).check_duration_sec = duration;
    }

    // Safety: 主路径的 unsafe fn `populate_expected_types` 调用，指针存活性与借用排它性同
    // 上一段对 `module_ptr` 的论证：`&item.source_module` 借的是另一 Arc，Module 内容此刻
    // 仅由该裸指针访问。
    unsafe { self.populate_expected_types(&item.source_module, module_ptr, &environment_scope) };

    if let Some(time_limit) = item.options.module_time_limit_sec
      && item.options.apply_internal_limit_scaling
    {
      // Safety: 主路径的时间超限回缩，对应 cpp `applyInternalLimitScaling(sourceNode, ...)`
      // 的共享引用原地写；`&mut *source_node_ptr` 借用半径止于本调用语句，此刻
      // `item.source_node` 无其它活跃借用（autocomplete 分支已 return，不可达）。
      apply_internal_limit_scaling(unsafe { &mut *source_node_ptr }, module.clone(), time_limit);
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
      // Safety: 对应 cpp New-solver + NoCheck 的 `module->errors.clear()`。`module_ptr`
      // 仍指向局部独占的 Module Arc；其上的只读借用（`&*module`、`custom_module_check` 的
      // `&module`）均已随各自语句结束，此刻清写无并存别名。
      unsafe {
        (*module_ptr).errors.clear();
      }
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
        item.source_module.root.cast::<AstStat>(),
        item.source_module.names.as_ref(),
        &environment_scope,
        module_ptr as *const Module,
        &item.source_module.hotcomments,
        &lint_options,
      );

      item.stats.time_lint += get_timestamp() - lint_timestamp;

      let lint_result = self.classify_lints(&warnings, &item.config);
      // Safety: 对应 cpp `module->lintResult = classifyLints(...)`。`lint` 仅以
      // `module_ptr as *const Module` 做过瞬态只读查询且已返回；写回发生在局部独占 Arc
      // 指向的 Module 上，无并存借用。
      unsafe {
        (*module_ptr).lint_result = lint_result;
      }
    }

    if !item.options.retain_full_type_graphs {
      // copyErrors needs to allocate into interfaceTypes as it copies types out of
      // internalTypes, so we unfreeze it here.
      // Safety: 对应 cpp 的非保留全类型图清理路径（unfreeze → copyErrors → freeze →
      // clear 各 arena/ast_* 表）。所有字段借用均由同一 `module_ptr`（局部独占、尚未移交
      // 的 `Arc<Module>`）派生；每个 `&mut (*module_ptr).x` 的借用半径止于所属语句，块内
      // 逐条顺序执行、互不重叠，单线程独占驱动（lib.rs 不变量 1）下无并发写。
      unsafe {
        unfreeze(&mut (*module_ptr).interface_types);
        // copyErrors(module->errors, module->interfaceTypes, builtinTypes);
        let mut errors = take(&mut (*module_ptr).errors);
        copy_errors(
          &mut errors,
          &mut (*module_ptr).interface_types,
          self.builtin_types_ref(),
        );
        (*module_ptr).errors = errors;
        freeze(&mut (*module_ptr).interface_types);

        (*module_ptr).internal_types.clear();
        (*module_ptr).def_arena.allocator.clear();
        (*module_ptr).key_arena.allocator.clear();

        (*module_ptr).ast_types.clear();
        (*module_ptr).ast_type_packs.clear();
        (*module_ptr).ast_expected_types.clear();
        (*module_ptr).ast_original_call_types.clear();
        (*module_ptr).ast_overload_resolved_types.clear();
        (*module_ptr).ast_for_in_next_types.clear();
        (*module_ptr).ast_resolved_types.clear();
        (*module_ptr).ast_resolved_type_packs.clear();
        (*module_ptr).ast_compound_assign_result_types.clear();
        (*module_ptr).ast_scopes.clear();
        (*module_ptr).upper_bound_contributors.clear();
        (*module_ptr).scopes.clear();
      }
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
        // Safety: 循环依赖错误逐条 push 进 `module->errors`（cpp 同名路径）。`module_ptr`
        // 指向尚未移交的局部独占 Arc  contents，循环体每次迭代为独立末尾写，无并存别名；
        // 上方清理块的可变借用已随其语句块结束。
        unsafe {
          (*module_ptr).errors.push(te);
        }
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
    // Safety: 对应 cpp `module->errors.insert(begin(), parseErrors...)` 的前插合并：
    // `take` 走局部 Module 的 errors，与新建的 parse_errors 拼接后写回。整个块只在
    // 函数末尾 `item.module = module` 移交前访问该 Arc 内容，读改写顺序进行、无并存别名。
    unsafe {
      // module->errors.insert(module->errors.begin(), parseErrors.begin(), parseErrors.end());
      let mut combined = parse_errors;
      combined.append(&mut (*module_ptr).errors);
      (*module_ptr).errors = combined;
    }

    item.module = module;
  }
}
