extern crate ulua_ast;

use alloc::{string::String, vec::Vec};

use ulua_ast::records::ast_expr::AstExpr;

use crate::functions::{
  parse_path_expr::parse_path_expr,
  path_expr_to_module_name_fixture::path_expr_to_module_name_module_name_vector_string_view,
};

pub fn path_expr_to_module_name_module_name_ast_expr(
  current_module_name: &str,
  path_expr: &AstExpr,
) -> Option<String> {
  let segments: Vec<&str> = parse_path_expr(path_expr);
  path_expr_to_module_name_module_name_vector_string_view(current_module_name, &segments)
}
