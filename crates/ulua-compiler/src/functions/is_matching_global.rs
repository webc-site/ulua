use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_name::AstName, ast_node::AstNode,
  },
  rtti,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{enums::global::Global, functions::get_global_state::get_global_state};

pub fn is_matching_global(
  globals: &DenseHashMap<AstName, Global>,
  node: *mut AstExpr,
  name: &str,
) -> bool {
  let expr_global = unsafe { rtti::ast_node_as::<AstExprGlobal>(node as *mut AstNode) };

  if !expr_global.is_null() {
    let expr = unsafe { &*expr_global };
    return get_global_state(globals, expr.name) == Global::Default && expr.name == name;
  }

  false
}
