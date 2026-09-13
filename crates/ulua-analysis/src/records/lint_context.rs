use alloc::vec::Vec;

use ulua_ast::records::{ast_name::AstName, ast_stat::AstStat};
use ulua_common::records::dense_hash_map::DenseHashMap;
use ulua_config::records::{lint_options::LintOptions, lint_warning::LintWarning};

use crate::{
  records::{global_linter::Global, module::Module},
  type_aliases::scope_ptr_type::ScopePtr,
};
#[derive(Debug, Clone)]
pub struct LintContext {
  pub result: Vec<LintWarning>,
  pub options: LintOptions,
  pub root: *mut AstStat,
  pub placeholder: AstName,
  pub builtin_globals: DenseHashMap<AstName, Global>,
  pub scope: ScopePtr,
  pub module: *const Module,
}
