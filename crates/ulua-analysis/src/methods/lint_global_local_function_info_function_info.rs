use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_name::AstName};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::function_info::FunctionInfo;
impl FunctionInfo {
  pub fn function_info_ast(ast: *mut AstExprFunction) -> Self {
    Self {
      ast,
      dominated_globals: DenseHashSet::new(AstName::default()),
      conditional_execution: false,
    }
  }
}
