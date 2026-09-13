use alloc::vec::Vec;

use ulua_ast::records::{ast_expr_global::AstExprGlobal, ast_name::AstName};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  function_info::FunctionInfo, global_linter_alt_b::Global, lint_context::LintContext,
};

#[derive(Debug, Clone)]
pub struct LintGlobalLocal {
  pub(crate) context: *mut LintContext,
  pub(crate) globals: DenseHashMap<AstName, Global>,
  pub(crate) global_refs: Vec<*mut AstExprGlobal>,
  pub(crate) function_stack: Vec<FunctionInfo>,
}
