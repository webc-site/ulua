use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{global_linter_alt_c::Global, lint_context::LintContext, local_linter::Local};

#[derive(Debug, Clone)]
pub struct LintLocalHygiene {
  pub(crate) context: *mut LintContext,
  pub(crate) locals: DenseHashMap<*mut AstLocal, Local>,
  pub(crate) imports: DenseHashMap<AstName, *mut AstLocal>,
  pub(crate) globals: DenseHashMap<AstName, Global>,
}
