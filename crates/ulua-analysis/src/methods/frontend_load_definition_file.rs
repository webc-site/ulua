//! C++ `Frontend::loadDefinitionFile` (`Analysis/src/Frontend.cpp:218-247`).
use alloc::{string::String, vec::Vec};

use ulua_ast::enums::mode::Mode;
use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  functions::{
    parse_source_for_module::parse_source_for_module, persist_checked_types::persist_checked_types,
  },
  records::{
    frontend::{Frontend, FrontendStats},
    global_types::GlobalTypes,
    load_definition_file_result::LoadDefinitionFileResult,
    require_cycle::RequireCycle,
    source_module::SourceModule,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};

impl Frontend {
  pub fn load_definition_file(
    &mut self,
    globals: &mut GlobalTypes,
    target_scope: ScopePtr,
    source: &str,
    package_name: String,
    capture_comments: bool,
    _type_check_for_autocomplete: bool,
  ) -> LoadDefinitionFileResult {
    LUAU_TIMETRACE_SCOPE!("loadDefinitionFile", "Frontend");

    let mut source_module = SourceModule::new();
    source_module.name = package_name.clone();
    source_module.human_readable_name = package_name.clone();

    let parse_result = parse_source_for_module(source, &mut source_module, capture_comments);
    if !parse_result.errors.is_empty() {
      return LoadDefinitionFileResult {
        success: false,
        parse_result,
        source_module,
        module: None,
      };
    }

    let mut dummy_stats = FrontendStats::default();
    let checked_module = self.check_source_module_mode_vector_require_cycle_optional_scope_ptr_bool_bool_frontend_stats_type_check_limits(
            &source_module,
            Mode::Definition,
            Vec::<RequireCycle>::new(),
            None,
            /* forAutocomplete */ false,
            /* recordJsonLog */ false,
            &mut dummy_stats,
            TypeCheckLimits::default(),
        );

    if !checked_module.errors.is_empty() {
      return LoadDefinitionFileResult {
        success: false,
        parse_result,
        source_module,
        module: Some(checked_module),
      };
    }

    persist_checked_types(checked_module.clone(), globals, target_scope, package_name);

    // Retain the checked module so its `TypeArena` — which owns the types
    // just persisted into the global scope — outlives the returned
    // `LoadDefinitionFileResult`. Without this, dropping the result (e.g. at
    // the end of `register_builtin_globals`, which loads `"@luau"` twice)
    // frees the arena while the global scope still references its types: a
    // use-after-free the type checker then reads, SIGSEGVing on some
    // toolchains (issue #6). Append-only so a repeated package name does not
    // evict an earlier, still-referenced module.
    globals.retained_modules.push(checked_module.clone());

    LoadDefinitionFileResult {
      success: true,
      parse_result,
      source_module,
      module: Some(checked_module),
    }
  }
}
