use core::ptr::null;

use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal,
    ast_name::AstName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

pub fn get_identifier(node: *mut AstExpr) -> AstName {
  if node.is_null() {
    return AstName { value: null() };
  }

  unsafe {
    let node_base = node as *mut AstNode;

    let global = ast_node_as::<AstExprGlobal>(node_base);
    if !global.is_null() {
      return (*global).name;
    }

    let local = ast_node_as::<AstExprLocal>(node_base);
    if !local.is_null() && !(*local).local.is_null() {
      return (*(*local).local).name;
    }
  }

  AstName { value: null() }
}
