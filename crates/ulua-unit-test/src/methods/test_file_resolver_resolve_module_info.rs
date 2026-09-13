use ulua_analysis::records::module_info::ModuleInfo;
use ulua_ast::records::ast_expr::AstExpr;

use crate::{
  functions::path_expr_to_module_name_fixture_alt_b::path_expr_to_module_name_module_name_ast_expr,
  records::test_file_resolver::TestFileResolver,
};

impl TestFileResolver {
  pub fn resolve_module_info(
    &self,
    current_module_name: &str,
    path_expr: &AstExpr,
  ) -> Option<ModuleInfo> {
    path_expr_to_module_name_module_name_ast_expr(current_module_name, path_expr).map(|name| {
      ModuleInfo {
        name,
        optional: false,
      }
    })
  }
}
