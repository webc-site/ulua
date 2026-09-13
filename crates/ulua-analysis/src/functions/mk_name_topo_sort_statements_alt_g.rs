use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::{
    mk_name_topo_sort_statements_alt_b::mk_name_ast_expr_local,
    mk_name_topo_sort_statements_alt_c::mk_name_ast_expr_global,
    mk_name_topo_sort_statements_alt_e::mk_name_ast_expr_index_name,
    mk_name_topo_sort_statements_alt_f::mk_name_ast_expr_error,
  },
  records::identifier::Identifier,
};
pub fn mk_name_ast_expr(expr: &AstExpr) -> Option<Identifier> {
  unsafe {
    let local = ast_node_as::<AstExprLocal>(expr as *const AstExpr as *mut AstNode);
    if !local.is_null() {
      return Some(mk_name_ast_expr_local(&*local));
    }

    let global = ast_node_as::<AstExprGlobal>(expr as *const AstExpr as *mut AstNode);
    if !global.is_null() {
      return Some(mk_name_ast_expr_global(&*global));
    }

    let index_name = ast_node_as::<AstExprIndexName>(expr as *const AstExpr as *mut AstNode);
    if !index_name.is_null() {
      return mk_name_ast_expr_index_name(&*index_name);
    }

    let error = ast_node_as::<AstExprError>(expr as *const AstExpr as *mut AstNode);
    if !error.is_null() {
      return Some(mk_name_ast_expr_error(&*error));
    }

    None
  }
}
