use alloc::vec::Vec;

use ulua_common::{
  FFlag,
  macros::{
    luau_assert::LUAU_ASSERT, luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT,
    luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
  },
  records::dense_hash_set::DenseHashSet,
};

use crate::{
  enums::solver_mode::SolverMode,
  functions::make_type_check_limits::make_type_check_limits,
  records::{
    build_queue_item::BuildQueueItem, check_result::CheckResult, frontend::Frontend,
    frontend_options::FrontendOptions,
  },
  type_aliases::module_name_type::ModuleName,
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

    if FFlag::DebugLuauLogSolverToJson.get() {
      LUAU_ASSERT!(build_queue_items.last().unwrap().name == *name);
      build_queue_items.last_mut().unwrap().record_json_log = true;
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
}
