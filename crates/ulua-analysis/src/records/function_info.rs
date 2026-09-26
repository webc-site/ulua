use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_name::AstName};
use ulua_common::records::dense_hash_set::DenseHashSet;
#[derive(Debug, Clone)]
pub struct FunctionInfo {
  pub ast: *mut AstExprFunction,
  pub dominated_globals: DenseHashSet<AstName>,
  pub conditional_execution: bool,
}
