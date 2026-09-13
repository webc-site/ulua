//! C++ free function `lint` (`Analysis/src/Linter.cpp:3517`).

use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::{
  ast_name::AstName, ast_name_table::AstNameTable, ast_stat::AstStat, hot_comment::HotComment,
};
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
  methods::{
    lint_implicit_return_process::lint_implicit_return_process,
    lint_local_hygiene_process::lint_local_hygiene_process,
    lint_redundant_native_attribute_process::lint_redundant_native_attribute_process,
    lint_unbalanced_assignment_process::lint_unbalanced_assignment_process,
    lint_unknown_type_process::lint_unknown_type_process,
  },
  records::{
    lint_comparison_precedence::LintComparisonPrecedence, lint_context::LintContext,
    lint_deprecated_api::LintDeprecatedApi, lint_duplicate_condition::LintDuplicateCondition,
    lint_duplicate_function::LintDuplicateFunction, lint_duplicate_local::LintDuplicateLocal,
    lint_for_range::LintForRange, lint_format_string::LintFormatString,
    lint_global_local::LintGlobalLocal, lint_integer_parsing::LintIntegerParsing,
    lint_misleading_and_or::LintMisleadingAndOr, lint_multi_line_statement::LintMultiLineStatement,
    lint_same_line_statement::LintSameLineStatement, lint_table_literal::LintTableLiteral,
    lint_table_operations::LintTableOperations, lint_uninitialized_local::LintUninitializedLocal,
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
    placeholder: unsafe { names.get(c"_".as_ptr()) },
    builtin_globals: DenseHashMap::new(AstName::new()),
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
    LintMultiLineStatement::new(null_mut()).process(&mut context);
  }

  if context.warning_enabled(Code::SameLineStatement) {
    LintSameLineStatement::new(null_mut()).process(&mut context);
  }

  if context.warning_enabled(Code::LocalShadow)
    || context.warning_enabled(Code::FunctionUnused)
    || context.warning_enabled(Code::ImportUnused)
    || context.warning_enabled(Code::LocalUnused)
  {
    lint_local_hygiene_process(&mut context);
  }

  if context.warning_enabled(Code::FunctionUnused) {
    LintUnusedFunction::new().process(&mut context);
  }

  if context.warning_enabled(Code::UnreachableCode) {
    LintUnreachableCode::process(&mut context);
  }

  if context.warning_enabled(Code::UnknownType) {
    lint_unknown_type_process(&mut context);
  }

  if context.warning_enabled(Code::ForRange) {
    LintForRange::process(&mut context);
  }

  if context.warning_enabled(Code::UnbalancedAssignment) {
    lint_unbalanced_assignment_process(&mut context);
  }

  if context.warning_enabled(Code::ImplicitReturn) {
    lint_implicit_return_process(&mut context);
  }

  if context.warning_enabled(Code::FormatString) {
    LintFormatString {
      context: null_mut(),
    }
    .process(&mut context);
  }

  if context.warning_enabled(Code::TableLiteral) {
    LintTableLiteral {
      context: null_mut(),
    }
    .process(&mut context);
  }

  if context.warning_enabled(Code::UninitializedLocal) {
    LintUninitializedLocal::process(&mut context);
  }

  if context.warning_enabled(Code::DuplicateFunction) {
    LintDuplicateFunction::new(&mut context as *mut LintContext).process();
  }

  if context.warning_enabled(Code::DeprecatedApi) {
    LintDeprecatedApi {
      context: null_mut(),
      function_type_scope_stack: Vec::new(),
    }
    .process(&mut context);
  }

  if context.warning_enabled(Code::TableOperations) {
    LintTableOperations::process(&mut context);
  }

  if context.warning_enabled(Code::DuplicateCondition) {
    LintDuplicateCondition {
      context: &mut context as *mut LintContext,
    }
    .process();
  }

  if context.warning_enabled(Code::DuplicateLocal) {
    LintDuplicateLocal::process(&mut context);
  }

  if context.warning_enabled(Code::MisleadingAndOr) {
    LintMisleadingAndOr {
      context: null_mut(),
    }
    .process(&mut context);
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
    lint_redundant_native_attribute_process(&mut context);
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
