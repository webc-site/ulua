use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::lint_duplicate_function::LintDuplicateFunction;

impl LintDuplicateFunction {
  pub fn build_name(&self, expr: *mut AstExpr) -> String {
    unsafe {
      let local = ast_node_as::<AstExprLocal>(expr as *mut AstNode);
      if let Some(local) = local.as_ref() {
        let name = (*local.local).name;
        if !name.is_null() {
          return name.to_string();
        }
      }

      let global = ast_node_as::<AstExprGlobal>(expr as *mut AstNode);
      if let Some(global) = global.as_ref() {
        let name = global.name;
        if !name.is_null() {
          return name.to_string();
        }
      }

      let index_name = ast_node_as::<AstExprIndexName>(expr as *mut AstNode);
      if let Some(index_name) = index_name.as_ref() {
        let lhs = self.build_name(index_name.expr);
        if lhs.is_empty() {
          return lhs;
        }
        return format!("{}.{}", lhs, index_name.index);
      }
    }
    String::new()
  }
}
