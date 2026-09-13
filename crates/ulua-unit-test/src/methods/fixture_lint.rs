//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.UnitTest:tests/Fixture.cpp:348:fixture_lint`
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
//!   - calls <- test config_disable_a_lint_rule (tests/Config.test.cpp)
//!   - calls <- test config_report_a_syntax_error (tests/Config.test.cpp)
//!   - calls <- test config_lint_warnings_are_ordered (tests/Config.test.cpp)
//!   - calls <- test config_comments (tests/Config.test.cpp)
//!   - calls <- test config_lint_rules_compat (tests/Config.test.cpp)
//!   - calls <- test config_extract_configuration (tests/Config.test.cpp)
//!   - calls <- test config_extract_luau_configuration (tests/Config.test.cpp)
//!   - calls <- method Fixture::parse (tests/Fixture.cpp)
//!   - type_ref <- method Fixture::parse (tests/Fixture.cpp)
//!   - calls <- test frontend_dont_reparse_clean_file_when_linting (tests/Frontend.test.cpp)
//!   - calls <- test linter_clean_code (tests/Linter.test.cpp)
//!   - calls <- test linter_type_function_fully_reduces (tests/Linter.test.cpp)
//!   - calls <- test linter_unknown_global (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_global (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_global_no_replacement (tests/Linter.test.cpp)
//!   - calls <- test linter_placeholder_read (tests/Linter.test.cpp)
//!   - calls <- test linter_placeholder_read_global (tests/Linter.test.cpp)
//!   - calls <- test linter_placeholder_write (tests/Linter.test.cpp)
//!   - calls <- test linter_builtin_global_write (tests/Linter.test.cpp)
//!   - calls <- test linter_multiline_block (tests/Linter.test.cpp)
//!   - calls <- test linter_multiline_block_semicolons_whitelisted (tests/Linter.test.cpp)
//!   - calls <- test linter_multiline_block_missed_semicolon (tests/Linter.test.cpp)
//!   - calls <- test linter_multiline_block_local_do (tests/Linter.test.cpp)
//!   - calls <- test linter_confusing_indentation (tests/Linter.test.cpp)
//!   - calls <- test linter_global_as_local (tests/Linter.test.cpp)
//!   - calls <- test linter_global_as_local_multi_fx (tests/Linter.test.cpp)
//!   - calls <- test linter_global_as_local_multi_fx_with_read (tests/Linter.test.cpp)
//!   - calls <- test linter_global_as_local_with_conditional (tests/Linter.test.cpp)
//!   - calls <- test linter_global_as_local_3_with_conditional_read (tests/Linter.test.cpp)
//!   - calls <- test linter_global_as_local_inner_read (tests/Linter.test.cpp)
//!   - calls <- test linter_global_as_local_multi (tests/Linter.test.cpp)
//!   - calls <- test linter_local_shadow_local (tests/Linter.test.cpp)
//!   - calls <- test linter_local_shadow_global (tests/Linter.test.cpp)
//!   - calls <- test linter_local_shadow_argument (tests/Linter.test.cpp)
//!   - calls <- test linter_local_unused (tests/Linter.test.cpp)
//!   - calls <- test linter_import_unused (tests/Linter.test.cpp)
//!   - calls <- test linter_function_unused (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_basic (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_loop_break (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_loop_continue (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_if_merge (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_error_return_silent (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_assert_false_return_silent (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_error_return_non_silent_branchy (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_error_return_propagate (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_loop_while (tests/Linter.test.cpp)
//!   - calls <- test linter_unreachable_code_loop_repeat (tests/Linter.test.cpp)
//!   - calls <- test linter_unknown_type (tests/Linter.test.cpp)
//!   - calls <- test linter_for_range_table (tests/Linter.test.cpp)
//!   - calls <- test linter_for_range_backwards (tests/Linter.test.cpp)
//!   - calls <- test linter_for_range_imprecise (tests/Linter.test.cpp)
//!   - calls <- test linter_for_range_zero (tests/Linter.test.cpp)
//!   - calls <- test linter_unbalanced_assignment (tests/Linter.test.cpp)
//!   - calls <- test linter_implicit_return (tests/Linter.test.cpp)
//!   - calls <- test linter_implicit_return_infinite_loop (tests/Linter.test.cpp)
//!   - calls <- test linter_type_annotations_should_not_produce_warnings (tests/Linter.test.cpp)
//!   - calls <- test linter_break_from_infinite_loop_makes_statement_reachable (tests/Linter.test.cpp)
//!   - calls <- test linter_ignore_lint_all (tests/Linter.test.cpp)
//!   - calls <- test linter_ignore_lint_specific (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_format (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_pack (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_match (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_match_nested (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_match_sets (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_find_args (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_replace (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_date (tests/Linter.test.cpp)
//!   - calls <- test linter_format_string_typed (tests/Linter.test.cpp)
//!   - calls <- test linter_table_literal (tests/Linter.test.cpp)
//!   - calls <- test linter_read_write_table_props (tests/Linter.test.cpp)
//!   - calls <- test linter_import_only_used_in_type_annotation (tests/Linter.test.cpp)
//!   - calls <- test linter_import_only_used_in_return_type (tests/Linter.test.cpp)
//!   - calls <- test linter_disable_unknown_global_with_type_checking (tests/Linter.test.cpp)
//!   - calls <- test linter_no_spurious_warning_after_a_function_type_alias (tests/Linter.test.cpp)
//!   - calls <- test linter_dead_locals_used (tests/Linter.test.cpp)
//!   - calls <- test linter_local_function_not_dead (tests/Linter.test.cpp)
//!   - calls <- test linter_duplicate_global_function (tests/Linter.test.cpp)
//!   - calls <- test linter_duplicate_local_function (tests/Linter.test.cpp)
//!   - calls <- test linter_duplicate_method (tests/Linter.test.cpp)
//!   - calls <- test linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes (tests/Linter.test.cpp)
//!   - calls <- test linter_lint_hygiene_uaf (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_api_typed (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_api_untyped (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_api_fenv (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_attribute (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_attribute_with_params (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_attribute_function_declaration (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_attribute_table_declaration (tests/Linter.test.cpp)
//!   - calls <- test linter_deprecated_attribute_method_declaration (tests/Linter.test.cpp)
//!   - calls <- test linter_table_operations (tests/Linter.test.cpp)
//!   - calls <- test linter_table_operations_indexer (tests/Linter.test.cpp)
//!   - calls <- test linter_duplicate_conditions (tests/Linter.test.cpp)
//!   - calls <- test linter_duplicate_conditions_expr (tests/Linter.test.cpp)
//!   - calls <- test linter_duplicate_local (tests/Linter.test.cpp)
//!   - calls <- test linter_misleading_and_or (tests/Linter.test.cpp)
//!   - calls <- test linter_wrong_comment (tests/Linter.test.cpp)
//!   - calls <- test linter_wrong_comment_mute_self (tests/Linter.test.cpp)
//!   - calls <- test linter_duplicate_conditions_if_stat_and_expr (tests/Linter.test.cpp)
//!   - calls <- test linter_wrong_comment_optimize (tests/Linter.test.cpp)
//!   - calls <- test linter_test_string_interpolation (tests/Linter.test.cpp)
//!   - calls <- test linter_integer_parsing (tests/Linter.test.cpp)
//!   - calls <- test linter_integer_parsing_decimal_imprecise (tests/Linter.test.cpp)
//!   - calls <- test linter_integer_parsing_hex_imprecise (tests/Linter.test.cpp)
//!   - calls <- test linter_comparison_precedence (tests/Linter.test.cpp)
//!   - calls <- test linter_redundant_native_attribute (tests/Linter.test.cpp)
//!   - calls <- test linter_type_instantiation_lints (tests/Linter.test.cpp)
//! - outgoing:
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - type_ref -> function lint (Analysis/src/Linter.cpp)
//!   - type_ref -> record LintOptions (Config/include/Luau/LinterConfig.h)
//!   - calls -> function fromString (tests/Fixture.cpp)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - translates_to -> rust_item Fixture::lint

use alloc::string::String;

use ulua_analysis::records::lint_result::LintResult;
use ulua_ast::enums::mode::Mode;
use ulua_config::records::lint_options::LintOptions;

use crate::records::fixture::Fixture;

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
  pub fn lint(&mut self, source: &str, lint_options: Option<LintOptions>) -> LintResult {
    let module_name = String::from(MAIN_MODULE_NAME);
    self.config_resolver.default_config.mode = Mode::Strict;
    self
      .file_resolver
      .source
      .insert(module_name.clone(), source.to_owned());

    let frontend = self.get_frontend();
    frontend.mark_dirty(&module_name, None);

    self.lint_module(&module_name, lint_options)
  }
}
