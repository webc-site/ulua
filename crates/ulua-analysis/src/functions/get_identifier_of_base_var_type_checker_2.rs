use alloc::string::{String, ToString};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
pub fn get_identifier_of_base_var_mut(node: *mut AstExpr) -> Option<String> {
  unsafe {
    let global = ast_node_as::<AstExprGlobal>(node as *mut AstNode);
    if !global.is_null() {
      // AstName 由词法器 intern，必为合法 UTF-8；空名（null）按 "" 处理。
      return Some((*global).name.as_str_or_empty().to_string());
    }

    let local = ast_node_as::<AstExprLocal>(node as *mut AstNode);
    if !local.is_null() {
      return Some((*(*local).local).name.as_str_or_empty().to_string());
    }

    let index_expr = ast_node_as::<AstExprIndexExpr>(node as *mut AstNode);
    if !index_expr.is_null() {
      return get_identifier_of_base_var_mut((*index_expr).expr);
    }

    let index_name = ast_node_as::<AstExprIndexName>(node as *mut AstNode);
    if !index_name.is_null() {
      return get_identifier_of_base_var_mut((*index_name).expr);
    }

    None
  }
}
