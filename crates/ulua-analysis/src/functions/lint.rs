//! C++ free function `lint` (`Analysis/src/Linter.cpp:3517`).

use alloc::vec::Vec;

use ulua_ast::records::{ast_name_table::AstNameTable, ast_stat::AstStat, hot_comment::HotComment};
use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_config::{
  enums::code::Code,
  records::{lint_options::LintOptions, lint_warning::LintWarning},
};

use crate::{
  functions::{
    fill_builtin_globals::fill_builtin_globals,
    has_native_comment_directive::has_native_comment_directive, lint_comments::lint_comments,
  },
  records::{
    lint_comparison_precedence::LintComparisonPrecedence, lint_context::LintContext,
    lint_deprecated_api::LintDeprecatedApi, lint_duplicate_condition::LintDuplicateCondition,
    lint_duplicate_function::LintDuplicateFunction, lint_duplicate_local::LintDuplicateLocal,
    lint_for_range::LintForRange, lint_format_string::LintFormatString,
    lint_global_local::LintGlobalLocal, lint_implicit_return::LintImplicitReturn,
    lint_integer_parsing::LintIntegerParsing, lint_local_hygiene::LintLocalHygiene,
    lint_misleading_and_or::LintMisleadingAndOr, lint_multi_line_statement::LintMultiLineStatement,
    lint_redundant_native_attribute::LintRedundantNativeAttribute,
    lint_same_line_statement::LintSameLineStatement, lint_table_literal::LintTableLiteral,
    lint_table_operations::LintTableOperations,
    lint_unbalanced_assignment::LintUnbalancedAssignment,
    lint_uninitialized_local::LintUninitializedLocal, lint_unknown_type::LintUnknownType,
    lint_unreachable_code::LintUnreachableCode, lint_unused_function::LintUnusedFunction,
    module::Module, warning_comparator::WarningComparator,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
pub fn lint(
  root: *mut AstStat,
  names: &AstNameTable,
  env: &ScopePtr,
  module: *const Module,
  hotcomments: &[HotComment],
  options: &LintOptions,
) -> Vec<LintWarning> {
  let mut context = LintContext {
    result: Vec::new(),
    options: *options,
    root,
    placeholder: names.get_str("_"),
    builtin_globals: DenseHashMap::default(),
    scope: env.clone(),
    module,
  };

  fill_builtin_globals(&mut context, names, env);

  if context.warning_enabled(Code::UnknownGlobal)
    || context.warning_enabled(Code::DeprecatedGlobal)
    || context.warning_enabled(Code::GlobalUsedAsLocal)
    || context.warning_enabled(Code::PlaceholderRead)
    || context.warning_enabled(Code::BuiltinGlobalWrite)
  {
    LintGlobalLocal::process(&mut context);
  }

  if context.warning_enabled(Code::MultiLineStatement) {
    LintMultiLineStatement::process(&mut context);
  }

  if context.warning_enabled(Code::SameLineStatement) {
    LintSameLineStatement::process(&mut context);
  }

  if context.warning_enabled(Code::LocalShadow)
    || context.warning_enabled(Code::FunctionUnused)
    || context.warning_enabled(Code::ImportUnused)
    || context.warning_enabled(Code::LocalUnused)
  {
    LintLocalHygiene::process(&mut context);
  }

  if context.warning_enabled(Code::FunctionUnused) {
    LintUnusedFunction::process(&mut context);
  }

  if context.warning_enabled(Code::UnreachableCode) {
    LintUnreachableCode::process(&mut context);
  }

  if context.warning_enabled(Code::UnknownType) {
    LintUnknownType::process(&mut context);
  }

  if context.warning_enabled(Code::ForRange) {
    LintForRange::process(&mut context);
  }

  if context.warning_enabled(Code::UnbalancedAssignment) {
    LintUnbalancedAssignment::process(&mut context);
  }

  if context.warning_enabled(Code::ImplicitReturn) {
    LintImplicitReturn::process(&mut context);
  }

  if context.warning_enabled(Code::FormatString) {
    LintFormatString::process(&mut context);
  }

  if context.warning_enabled(Code::TableLiteral) {
    LintTableLiteral::process(&mut context);
  }

  if context.warning_enabled(Code::UninitializedLocal) {
    LintUninitializedLocal::process(&mut context);
  }

  if context.warning_enabled(Code::DuplicateFunction) {
    LintDuplicateFunction::process(&mut context);
  }

  if context.warning_enabled(Code::DeprecatedApi) {
    LintDeprecatedApi::process(&mut context);
  }

  if context.warning_enabled(Code::TableOperations) {
    LintTableOperations::process(&mut context);
  }

  if context.warning_enabled(Code::DuplicateCondition) {
    LintDuplicateCondition::process(&mut context);
  }

  if context.warning_enabled(Code::DuplicateLocal) {
    LintDuplicateLocal::process(&mut context);
  }

  if context.warning_enabled(Code::MisleadingAndOr) {
    LintMisleadingAndOr::process(&mut context);
  }

  if context.warning_enabled(Code::CommentDirective) {
    lint_comments(&mut context, hotcomments);
  }

  if context.warning_enabled(Code::IntegerParsing) {
    LintIntegerParsing::process(&mut context);
  }

  if context.warning_enabled(Code::ComparisonPrecedence) {
    LintComparisonPrecedence::process(&mut context);
  }

  if context.warning_enabled(Code::RedundantNativeAttribute)
    && has_native_comment_directive(hotcomments)
  {
    LintRedundantNativeAttribute::process(&mut context);
  }

  let comparator = WarningComparator::default();
  context.result.sort_by(|lhs, rhs| {
    let c = comparator.compare_location_location(&lhs.location, &rhs.location);
    if c != 0 {
      return c.cmp(&0);
    }
    (lhs.code as i32).cmp(&(rhs.code as i32))
  });

  context.result
}
