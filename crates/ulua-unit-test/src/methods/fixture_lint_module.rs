//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.UnitTest:tests/Fixture.cpp:358:fixture_lint_module`
//! Source: `tests/Fixture.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Fixture.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Constraint.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Ast/include/Luau/PrettyPrinter.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeAttach.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//! - incoming:
//!   - declares <- source_file tests/Fixture.cpp
//!   - calls <- method Fixture::lint (tests/Fixture.cpp)
//!   - calls <- test frontend_dont_reparse_clean_file_when_linting (tests/Frontend.test.cpp)
//!   - calls <- test frontend_test_lint_uses_correct_config (tests/Frontend.test.cpp)
//!   - calls <- test frontend_lint_results_are_only_for_checked_module (tests/Frontend.test.cpp)
//!   - calls <- test linter_use_all_parent_scopes_for_globals (tests/Linter.test.cpp)
//! - outgoing:
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - type_ref -> record LintOptions (Config/include/Luau/LinterConfig.h)
//!   - type_ref -> record FrontendOptions (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - translates_to -> rust_item Fixture::lintModule

use ulua_analysis::{records::lint_result::LintResult, type_aliases::module_name_type::ModuleName};
use ulua_config::records::lint_options::LintOptions;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn lint_module(
    &mut self,
    module_name: &ModuleName,
    lint_options: Option<LintOptions>,
  ) -> LintResult {
    let mut options = self.get_frontend().options.clone();
    options.run_lint_checks = true;
    options.enabled_lint_warnings = lint_options;

    self
      .get_frontend()
      .check_module_name_optional_frontend_options(module_name, Some(options))
      .lint_result
  }
}
